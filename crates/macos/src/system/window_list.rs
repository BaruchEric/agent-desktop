use agent_desktop_core::{adapter::WindowFilter, error::AdapterError, node::WindowInfo};

pub(crate) fn list_windows_impl(filter: &WindowFilter) -> Result<Vec<WindowInfo>, AdapterError> {
    #[cfg(target_os = "macos")]
    {
        for attempt in 0..3 {
            let windows = list_windows_once(filter);
            if !windows.is_empty() || attempt == 2 {
                return Ok(windows);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Ok(vec![])
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = filter;
        Err(AdapterError::not_supported("list_windows"))
    }
}

#[cfg(target_os = "macos")]
fn list_windows_once(filter: &WindowFilter) -> Vec<WindowInfo> {
    #[cfg(target_os = "macos")]
    {
        use core_foundation::base::{CFType, TCFType};
        use core_foundation::number::CFNumber;
        use core_foundation::string::CFString;
        use core_foundation_sys::dictionary::CFDictionaryGetValue;
        use core_graphics::display::CGDisplay;
        use core_graphics::window::{
            kCGWindowLayer, kCGWindowListOptionOnScreenOnly, kCGWindowName, kCGWindowNumber,
            kCGWindowOwnerName, kCGWindowOwnerPID,
        };
        use std::{collections::HashMap, ffi::c_void};

        unsafe fn dict_string(dict: *const c_void, key: *const c_void) -> Option<String> {
            let val = unsafe { CFDictionaryGetValue(dict as _, key) };
            if val.is_null() {
                return None;
            }
            unsafe { CFType::wrap_under_get_rule(val as _) }
                .downcast::<CFString>()
                .map(|s| s.to_string())
        }

        unsafe fn dict_i64(dict: *const c_void, key: *const c_void) -> Option<i64> {
            let val = unsafe { CFDictionaryGetValue(dict as _, key) };
            if val.is_null() {
                return None;
            }
            unsafe { CFType::wrap_under_get_rule(val as _) }
                .downcast::<CFNumber>()
                .and_then(|n| n.to_i64())
        }

        let arr = match CGDisplay::window_list_info(kCGWindowListOptionOnScreenOnly, None) {
            Some(a) => a,
            None => return vec![],
        };

        let app_filter = filter.app.as_deref().unwrap_or("").to_lowercase();
        let mut candidates = Vec::new();

        for raw in arr.get_all_values() {
            if raw.is_null() {
                continue;
            }
            let layer = unsafe { dict_i64(raw, kCGWindowLayer as _) }.unwrap_or(99);
            if layer != 0 {
                continue;
            }

            let app_name = match unsafe { dict_string(raw, kCGWindowOwnerName as _) } {
                Some(n) if !n.is_empty() => n,
                _ => continue,
            };
            if !app_filter.is_empty() && !app_name.to_lowercase().contains(&app_filter) {
                continue;
            }

            let title = match unsafe { dict_string(raw, kCGWindowName as _) } {
                Some(t) if !t.is_empty() => t,
                _ => app_name.clone(),
            };

            let pid = unsafe { dict_i64(raw, kCGWindowOwnerPID as _) }.unwrap_or(0) as i32;
            let window_number = unsafe { dict_i64(raw, kCGWindowNumber as _) }.unwrap_or(0);

            candidates.push((app_name, title, pid, window_number));
        }

        let mut title_counts: HashMap<(i32, String), usize> = HashMap::new();
        for (_, title, pid, _) in &candidates {
            *title_counts.entry((*pid, title.clone())).or_insert(0) += 1;
        }

        let mut focus_cache: HashMap<i32, FocusedWindowIdentity> = HashMap::new();
        let mut windows = Vec::new();
        let mut focused_seen = false;

        for (app_name, title, pid, window_number) in candidates {
            let title_count = title_counts
                .get(&(pid, title.clone()))
                .copied()
                .unwrap_or(0);
            let identity = focus_cache
                .entry(pid)
                .or_insert_with(|| focused_window_identity(pid));
            let is_focused = !focused_seen
                && matches_focused_window(&title, window_number, identity, title_count);
            if filter.focused_only && !is_focused {
                continue;
            }
            focused_seen |= is_focused;

            windows.push(WindowInfo {
                id: format!("w-{window_number}"),
                title,
                app: app_name,
                pid,
                bounds: None,
                is_focused,
            });
        }
        if windows.is_empty() {
            if let Some(app_name) = filter.app.as_deref() {
                if let Some(window) = ax_window_for_app(app_name) {
                    if !filter.focused_only || window.is_focused {
                        windows.push(window);
                    }
                }
            }
        }
        windows
    }
}

#[cfg(target_os = "macos")]
fn ax_window_for_app(app_name: &str) -> Option<WindowInfo> {
    let pid = crate::system::app_list::pid_for_app_name(app_name)?;
    let app = crate::tree::element_for_pid(pid);
    let window = crate::tree::copy_element_attr(&app, "AXFocusedWindow")
        .or_else(|| crate::tree::copy_element_attr(&app, "AXMainWindow"))
        .or_else(|| {
            crate::tree::copy_ax_array(&app, "AXWindows")
                .and_then(|windows| windows.into_iter().next())
        })?;
    if crate::tree::copy_string_attr(&window, "AXRole").as_deref() != Some("AXWindow") {
        return None;
    }
    let title =
        crate::tree::copy_string_attr(&window, "AXTitle").unwrap_or_else(|| app_name.into());
    let window_number = crate::tree::copy_i64_attr(&window, "AXWindowNumber").unwrap_or(0);
    let is_focused = crate::tree::copy_bool_attr(&app, "AXFrontmost") == Some(true);
    Some(WindowInfo {
        id: format!("w-{window_number}"),
        title,
        app: app_name.to_string(),
        pid,
        bounds: None,
        is_focused,
    })
}

#[cfg(target_os = "macos")]
type FocusedWindowIdentity = Option<(Option<String>, Option<i64>)>;

#[cfg(target_os = "macos")]
fn focused_window_identity(pid: i32) -> FocusedWindowIdentity {
    let app = crate::tree::element_for_pid(pid);
    if crate::tree::copy_bool_attr(&app, "AXFrontmost") != Some(true) {
        return None;
    }
    let window = crate::tree::copy_element_attr(&app, "AXFocusedWindow")?;
    Some((
        crate::tree::copy_string_attr(&window, "AXTitle"),
        crate::tree::copy_i64_attr(&window, "AXWindowNumber"),
    ))
}

#[cfg(target_os = "macos")]
fn matches_focused_window(
    title: &str,
    window_number: i64,
    identity: &FocusedWindowIdentity,
    same_title_count: usize,
) -> bool {
    let Some((focused_title, focused_number)) = identity else {
        return false;
    };
    if let Some(number) = focused_number {
        return *number == window_number;
    }
    focused_title.as_deref() == Some(title) && same_title_count == 1
}
