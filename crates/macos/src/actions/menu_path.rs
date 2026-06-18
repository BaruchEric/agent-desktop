use agent_desktop_core::{action_result::ActionResult, error::AdapterError};

#[cfg(target_os = "macos")]
mod imp {
    use super::*;
    use crate::tree::{
        AXElement,
        attributes::{copy_ax_array, copy_bool_attr, copy_string_attr},
        element::element_for_pid,
    };
    use agent_desktop_core::error::ErrorCode;

    fn menubar_root(pid: i32) -> Option<AXElement> {
        let app = element_for_pid(pid);
        copy_ax_array(&app, "AXChildren")?
            .into_iter()
            .find(|ch| copy_string_attr(ch, "AXRole").as_deref() == Some("AXMenuBar"))
    }

    fn unwrap_menu(children: Vec<AXElement>) -> Vec<AXElement> {
        if let Some(first) = children.first() {
            if copy_string_attr(first, "AXRole").as_deref() == Some("AXMenu") {
                return copy_ax_array(first, "AXChildren").unwrap_or_default();
            }
        }
        children
    }

    fn sibling_titles(children: &[AXElement]) -> Vec<String> {
        children
            .iter()
            .filter_map(|ch| copy_string_attr(ch, "AXTitle"))
            .filter(|t| !t.is_empty())
            .collect()
    }

    fn resolve_path(pid: i32, path: &[String]) -> Result<AXElement, AdapterError> {
        let menubar = menubar_root(pid).ok_or_else(|| {
            AdapterError::new(ErrorCode::ElementNotFound, "Application has no menu bar")
                .with_suggestion("Verify the app is running and owns a standard menu bar.")
        })?;
        let mut children = copy_ax_array(&menubar, "AXChildren").unwrap_or_default();
        let last = path.len().saturating_sub(1);
        let mut target: Option<AXElement> = None;
        for (i, seg) in path.iter().enumerate() {
            let found = children
                .iter()
                .find(|ch| copy_string_attr(ch, "AXTitle").as_deref() == Some(seg.as_str()))
                .cloned()
                .ok_or_else(|| {
                    let near = sibling_titles(&children).join(", ");
                    AdapterError::new(
                        ErrorCode::ElementNotFound,
                        format!("Menu segment '{seg}' not found"),
                    )
                    .with_suggestion(format!("Available at this level: {near}"))
                })?;
            if i == last {
                target = Some(found);
                break;
            }
            children = unwrap_menu(copy_ax_array(&found, "AXChildren").unwrap_or_default());
        }
        target.ok_or_else(|| AdapterError::new(ErrorCode::InvalidArgs, "Empty menu path"))
    }

    fn press_with_retry(el: &AXElement) -> Result<(), AdapterError> {
        use accessibility_sys::{
            AXUIElementPerformAction, kAXErrorCannotComplete, kAXErrorSuccess,
        };
        use core_foundation::{base::TCFType, string::CFString};
        let action = CFString::new("AXPress");
        for attempt in 0u64..3 {
            let err = unsafe { AXUIElementPerformAction(el.0, action.as_concrete_TypeRef()) };
            if err == kAXErrorSuccess {
                return Ok(());
            }
            if err != kAXErrorCannotComplete {
                return Err(
                    AdapterError::new(ErrorCode::ActionFailed, "Menu item press failed")
                        .with_platform_detail(format!("AXError {err}")),
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(50 * (attempt + 1)));
        }
        Err(
            AdapterError::new(ErrorCode::Timeout, "Menu item press did not complete")
                .with_suggestion("Bring the app to the foreground and retry."),
        )
    }

    pub fn select_menu_path(pid: i32, path: &[String]) -> Result<ActionResult, AdapterError> {
        let target = resolve_path(pid, path)?;
        if copy_bool_attr(&target, "AXEnabled") == Some(false) {
            return Err(AdapterError::new(
                ErrorCode::ActionFailed,
                "Menu item is disabled",
            ));
        }
        press_with_retry(&target)?;
        Ok(ActionResult::new("select_menu_path"))
    }

    pub fn list_menu_paths(pid: i32) -> Result<Vec<String>, AdapterError> {
        let menubar = menubar_root(pid).ok_or_else(|| {
            AdapterError::new(ErrorCode::ElementNotFound, "Application has no menu bar")
        })?;
        let mut out = Vec::new();
        let tops = copy_ax_array(&menubar, "AXChildren").unwrap_or_default();
        for top in &tops {
            let Some(title) = copy_string_attr(top, "AXTitle") else {
                continue;
            };
            let leaves = unwrap_menu(copy_ax_array(top, "AXChildren").unwrap_or_default());
            for leaf in &leaves {
                if let Some(name) = copy_string_attr(leaf, "AXTitle") {
                    if !name.is_empty() {
                        out.push(format!("{title} > {name}"));
                    }
                }
            }
        }
        Ok(out)
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    use super::*;

    pub fn select_menu_path(_pid: i32, _path: &[String]) -> Result<ActionResult, AdapterError> {
        Err(AdapterError::not_supported("select_menu_path"))
    }

    pub fn list_menu_paths(_pid: i32) -> Result<Vec<String>, AdapterError> {
        Err(AdapterError::not_supported("list_menu_paths"))
    }
}

pub use imp::{list_menu_paths, select_menu_path};
