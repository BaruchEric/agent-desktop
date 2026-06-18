/// Integration tests for system-control commands (volume, appearance, wifi, run-shell).
///
/// All tests require a macOS build of agent-desktop and are ignored by default.
/// Run manually with:
///   cargo test --test system_control_test -- --ignored
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
    #[ignore = "requires macOS hardware/permissions"]
    fn volume_get_returns_envelope() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["volume", "--get"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        if json["ok"] == true {
            assert!(
                json["data"]["output_volume"].is_number(),
                "data.output_volume must be a number when ok=true, got: {json}"
            );
        } else {
            let code = json["error"]["code"].as_str().unwrap_or("");
            assert!(
                code == "PERM_DENIED" || code == "ACTION_FAILED",
                "on failure, error.code must be PERM_DENIED or ACTION_FAILED, got: {json}"
            );
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires macOS hardware/permissions"]
    fn appearance_get_returns_envelope() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["appearance", "--get"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        if json["ok"] == true {
            assert!(
                json["data"]["dark"].is_boolean(),
                "data.dark must be a boolean when ok=true, got: {json}"
            );
        } else {
            assert_eq!(
                json["error"]["code"], "PERM_DENIED",
                "on failure, error.code must be PERM_DENIED, got: {json}"
            );
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires macOS hardware/permissions"]
    fn wifi_status_returns_envelope() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["wifi", "--status"])
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        if json["ok"] == true {
            assert!(
                json["data"]["wifi_power"].is_boolean(),
                "data.wifi_power must be a boolean when ok=true, got: {json}"
            );
        } else {
            let code = json["error"]["code"].as_str().unwrap_or("");
            assert!(
                code == "PERM_DENIED" || code == "ACTION_FAILED",
                "on failure, error.code must be PERM_DENIED or ACTION_FAILED, got: {json}"
            );
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires macOS hardware/permissions"]
    fn run_shell_echo_roundtrips() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["run-shell", "echo ci-test"])
            .env("AGENT_DESKTOP_ENABLE_EXEC", "1")
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(json["ok"], true, "run-shell echo must succeed, got: {json}");
        assert_eq!(
            json["data"]["exit_code"], 0,
            "exit_code must be 0, got: {json}"
        );
        assert_eq!(
            json["data"]["stdout"], "ci-test\n",
            "stdout must be 'ci-test\\n', got: {json}"
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore = "requires macOS hardware/permissions"]
    fn run_shell_disabled_is_policy_denied() {
        let bin = agent_desktop_bin();
        let output = Command::new(&bin)
            .args(["run-shell", "echo x"])
            .env_remove("AGENT_DESKTOP_ENABLE_EXEC")
            .output()
            .expect("failed to run agent-desktop");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("output is not valid JSON");

        assert_eq!(
            json["ok"], false,
            "run-shell without EXEC env must fail, got: {json}"
        );
        assert_eq!(
            json["error"]["code"], "POLICY_DENIED",
            "error.code must be POLICY_DENIED, got: {json}"
        );
    }
}
