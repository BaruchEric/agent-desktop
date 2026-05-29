use super::*;

fn window(app: &str, pid: i32) -> WindowInfo {
    WindowInfo {
        id: format!("w-{pid}"),
        title: app.to_string(),
        app: app.to_string(),
        pid,
        bounds: None,
        is_focused: false,
    }
}

#[test]
fn apps_from_windows_deduplicates_by_pid() {
    let apps = apps_from_windows(&[window("Finder", 10), window("Finder", 10)]);

    assert_eq!(apps.len(), 1);
    assert_eq!(apps[0].name, "Finder");
}

#[test]
fn apps_from_windows_keeps_same_name_with_distinct_pids() {
    let apps = apps_from_windows(&[window("Preview", 10), window("Preview", 11)]);

    assert_eq!(apps.len(), 2);
}

#[test]
fn matches_app_filter_accepts_case_insensitive_substring() {
    assert!(matches_app_filter("Docker Desktop", "docker"));
    assert!(!matches_app_filter("Finder", "docker"));
}

#[test]
fn retry_empty_skips_known_missing_app_filter() {
    let filter = WindowFilter {
        app: Some("Missing".to_string()),
        focused_only: false,
    };

    assert!(!should_retry_empty(&filter, None));
}

#[test]
fn retry_empty_allows_known_app_filter_for_ax_race() {
    let filter = WindowFilter {
        app: Some("Mail".to_string()),
        focused_only: false,
    };

    assert!(should_retry_empty(&filter, Some(42)));
}

fn apps_from_windows(windows: &[WindowInfo]) -> Vec<AppInfo> {
    let mut seen_pids = std::collections::HashSet::new();
    let mut apps = Vec::new();

    for window in windows {
        if window.pid > 0 && seen_pids.insert(window.pid) {
            apps.push(AppInfo {
                name: window.app.clone(),
                pid: window.pid,
                bundle_id: None,
            });
        }
    }

    apps
}
