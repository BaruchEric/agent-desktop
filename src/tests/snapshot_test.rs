/// Integration tests for the snapshot command.
///
/// These tests require macOS with Accessibility permissions granted to the
/// terminal running the tests. They are skipped automatically on other
/// platforms or when the binary is not built.
#[cfg(test)]
mod tests {
    use std::process::Command;

    fn agent_desktop_bin() -> std::path::PathBuf {
        let mut p = std::env::current_exe().unwrap();
        p.pop();
        p.pop();
        p.push("agent-desktop");
        p
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn snapshot_finder_returns_non_empty_tree() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["snapshot", "--app", "Finder"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], true);
        assert!(json["data"]["ref_count"].as_u64().unwrap_or(0) > 0);
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn snapshot_textedit_returns_refs() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["snapshot", "--app", "TextEdit"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], true);
    }

    #[test]
    fn version_command_outputs_json() {
        let bin = agent_desktop_bin();
        if !bin.exists() {
            return;
        }
        let output = Command::new(&bin)
            .args(["version", "--json"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], true);
        assert!(json["data"]["version"].is_string());
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn snapshot_skeleton_returns_shallow_tree_with_children_count() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["snapshot", "--app", "Finder", "--skeleton", "-i"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], true);
        let tree = &json["data"]["tree"];
        let max_depth = find_max_depth(tree, 0);
        assert!(
            max_depth <= 4,
            "skeleton must clamp to depth ~3, got depth {max_depth}"
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn snapshot_skeleton_refresh_does_not_accumulate_stale_refs() {
        let bin = agent_desktop_bin();
        let run = |extra: &[&str]| {
            let mut args = vec!["snapshot", "--app", "Finder", "--skeleton", "-i"];
            args.extend_from_slice(extra);
            Command::new(&bin)
                .args(&args)
                .output()
                .expect("failed to run agent-desktop")
        };

        let first = run(&[]);
        let first_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&first.stdout)).unwrap();
        let first_count = first_json["data"]["ref_count"].as_u64().unwrap_or(0);

        let second = run(&[]);
        let second_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&second.stdout)).unwrap();
        let second_count = second_json["data"]["ref_count"].as_u64().unwrap_or(0);

        assert_eq!(
            first_count, second_count,
            "repeated skeleton refresh must produce identical ref_count (no accumulation)"
        );
    }

    #[test]
    fn snapshot_invalid_root_ref_format_returns_invalid_args() {
        let bin = agent_desktop_bin();
        if !bin.exists() {
            return;
        }
        let output = Command::new(&bin)
            .args(["snapshot", "--app", "Finder", "--root", "bad-ref"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], false);
        assert_eq!(
            json["error"]["code"], "INVALID_ARGS",
            "malformed --root must return INVALID_ARGS, got: {}",
            json["error"]["code"]
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn snapshot_root_drill_returns_non_empty_subtree() {
        let bin = agent_desktop_bin();
        let skeleton_out = Command::new(&bin)
            .args(["snapshot", "--app", "Finder", "--skeleton", "-i"])
            .output()
            .expect("failed to run agent-desktop");

        let skeleton_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&skeleton_out.stdout)).unwrap();
        assert_eq!(skeleton_json["ok"], true);

        let first_ref = first_ref_id(&skeleton_json["data"]["tree"]);
        let Some(ref_id) = first_ref else {
            return;
        };

        let drill_out = Command::new(&bin)
            .args(["snapshot", "--app", "Finder", "--root", &ref_id, "-i"])
            .output()
            .expect("failed to run agent-desktop");

        let drill_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&drill_out.stdout)).unwrap();

        assert_eq!(drill_json["ok"], true);
        assert!(
            drill_json["data"]["ref_count"].as_u64().unwrap_or(0) > 0,
            "drill-down must return refs"
        );
    }

    fn find_max_depth(node: &serde_json::Value, depth: usize) -> usize {
        let children = match node.get("children").and_then(|c| c.as_array()) {
            Some(c) if !c.is_empty() => c,
            _ => return depth,
        };
        children
            .iter()
            .map(|c| find_max_depth(c, depth + 1))
            .max()
            .unwrap_or(depth)
    }

    fn first_ref_id(node: &serde_json::Value) -> Option<String> {
        if let Some(r) = node.get("ref_id").and_then(|v| v.as_str()) {
            return Some(r.to_string());
        }
        if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
            for child in children {
                if let Some(r) = first_ref_id(child) {
                    return Some(r);
                }
            }
        }
        None
    }

    #[test]
    fn list_apps_on_non_macos_errors_gracefully() {
        #[cfg(not(target_os = "macos"))]
        {
            let bin = agent_desktop_bin();
            if !bin.exists() {
                return;
            }
            let output = Command::new(&bin)
                .args(["list-apps"])
                .output()
                .expect("failed to run agent-desktop");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let json: serde_json::Value =
                serde_json::from_str(&stdout).expect("output is not valid JSON");

            assert_eq!(json["ok"], false);
            assert_eq!(json["error"]["code"], "PLATFORM_NOT_SUPPORTED");
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn menu_path_activates_textedit_format_item() {
        let bin = agent_desktop_bin();

        Command::new(&bin)
            .args(["launch", "TextEdit", "--timeout", "8000"])
            .output()
            .expect("failed to launch TextEdit");

        Command::new(&bin)
            .args(["press", "cmd+n", "--app", "TextEdit"])
            .output()
            .expect("failed to open new TextEdit document");

        let format_path = if menu_path_exists(&bin, "TextEdit", "Format > Make Plain Text") {
            "Format > Make Plain Text"
        } else {
            "Format > Make Rich Text"
        };

        let output = Command::new(&bin)
            .args(["menu", "--app", "TextEdit", "--path", format_path])
            .output()
            .expect("failed to run menu command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        Command::new(&bin)
            .args(["close-app", "TextEdit", "--force"])
            .output()
            .ok();

        assert_eq!(json["ok"], true, "menu --path must succeed, got: {json}");
        assert_eq!(json["data"]["action"], "menu");
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn menu_list_returns_non_empty_paths_for_running_app() {
        let bin = agent_desktop_bin();

        let output = Command::new(&bin)
            .args(["menu", "--app", "Finder", "--list"])
            .output()
            .expect("failed to run menu --list");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], true);
        let paths = json["data"]["paths"]
            .as_array()
            .expect("data.paths must be an array");
        assert!(
            !paths.is_empty(),
            "menu --list must return at least one path"
        );

        let has_apple = paths.iter().any(|p| {
            p.as_str()
                .map(|s| s.starts_with("Apple >"))
                .unwrap_or(false)
        });
        assert!(
            has_apple,
            "Finder menu must include at least one Apple > item"
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn dock_surface_snapshot_yields_dockitem_refs() {
        let bin = agent_desktop_bin();

        let output = Command::new(&bin)
            .args(["snapshot", "--app", "Dock", "--surface", "dock"])
            .output()
            .expect("failed to run snapshot --surface dock");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(
            json["ok"], true,
            "dock surface snapshot must succeed, got: {json}"
        );
        assert!(
            json["data"]["ref_count"].as_u64().unwrap_or(0) > 0,
            "Dock snapshot must allocate at least one ref"
        );

        let has_dockitem = any_node_has_role(&json["data"]["tree"], "dockitem");
        assert!(
            has_dockitem,
            "Dock surface tree must contain at least one dockitem node"
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn dock_ref_resolves_to_dockitem_role() {
        let bin = agent_desktop_bin();

        let snap_out = Command::new(&bin)
            .args(["snapshot", "--app", "Dock", "--surface", "dock"])
            .output()
            .expect("failed to snapshot Dock");

        let snap_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&snap_out.stdout)).unwrap();
        assert_eq!(snap_json["ok"], true);

        let snapshot_id = snap_json["data"]["snapshot_id"]
            .as_str()
            .unwrap_or_default();
        let ref_id = match first_ref_id(&snap_json["data"]["tree"]) {
            Some(r) => r,
            None => return,
        };

        let get_out = Command::new(&bin)
            .args([
                "get",
                &ref_id,
                "--snapshot",
                snapshot_id,
                "--property",
                "role",
            ])
            .output()
            .expect("failed to run get command");

        let get_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&get_out.stdout)).unwrap();

        assert_eq!(
            get_json["ok"], true,
            "get on dock ref must succeed, got: {get_json}"
        );
        assert_eq!(get_json["data"]["value"], "dockitem");
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires Accessibility permissions and running macOS apps"]
    fn control_center_extras_menubar_surface_non_empty() {
        let bin = agent_desktop_bin();

        let output = Command::new(&bin)
            .args([
                "snapshot",
                "--app",
                "ControlCenter",
                "--surface",
                "extras-menubar",
            ])
            .output()
            .expect("failed to run snapshot --surface extras-menubar");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(
            json["ok"], true,
            "extras-menubar snapshot must succeed, got: {json}"
        );
        assert!(
            json["data"]["tree"].is_object(),
            "data.tree must be present"
        );
    }

    fn menu_path_exists(bin: &std::path::Path, app: &str, path: &str) -> bool {
        let out = Command::new(bin)
            .args(["menu", "--app", app, "--list"])
            .output()
            .unwrap_or_else(|_| panic!("failed to list menus for {app}"));
        let json: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&out.stdout)).unwrap_or_default();
        json["data"]["paths"]
            .as_array()
            .map(|paths| paths.iter().any(|p| p.as_str() == Some(path)))
            .unwrap_or(false)
    }

    fn any_node_has_role(node: &serde_json::Value, role: &str) -> bool {
        if node.get("role").and_then(|r| r.as_str()) == Some(role) {
            return true;
        }
        node.get("children")
            .and_then(|c| c.as_array())
            .map(|children| children.iter().any(|c| any_node_has_role(c, role)))
            .unwrap_or(false)
    }
}
