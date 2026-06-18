//! Shared harness for the gated real-app e2e test suite.
//!
//! Each `[[test]]` target is its own compilation unit, so any helper a given
//! test binary does not call is dead code there. `#![allow(dead_code,
//! unused_imports)]` keeps `cargo clippy --all-targets -- -D warnings` green,
//! mirroring `crates/ffi/tests/common/mod.rs`.
#![allow(dead_code, unused_imports)]

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

/// Absolute path to the freshly built `agent-desktop` binary that sits next to
/// the integration-test executable (`target/<profile>/agent-desktop`).
pub fn bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("current_exe");
    p.pop();
    p.pop();
    p.push("agent-desktop");
    p
}

/// True once the binary has been built; lets tests bail cleanly when invoked
/// before `cargo build`.
pub fn bin_exists() -> bool {
    bin().exists()
}

/// Run the binary and parse its JSON envelope. Panics only on a non-JSON
/// payload, surfacing the raw stdout for debugging.
pub fn run(args: &[&str]) -> Value {
    run_env(args, &[])
}

/// Run the binary with extra environment variables (e.g. `AGENT_DESKTOP_ENABLE_EXEC`).
pub fn run_env(args: &[&str], envs: &[(&str, &str)]) -> Value {
    let mut cmd = Command::new(bin());
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let output = cmd.output().expect("failed to run agent-desktop");
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("agent-desktop output is not valid JSON ({e}); args={args:?}; stdout=\n{stdout}")
    })
}

/// True when the accessibility permission is granted to the test runner. The
/// permission state is invariant for a run, so the probe is spawned once per
/// test binary and cached.
pub fn ax_granted() -> bool {
    static CACHED: OnceLock<bool> = OnceLock::new();
    *CACHED.get_or_init(|| {
        let report = run(&["permissions"]);
        report["ok"] == true && report["data"]["accessibility"]["state"] == "granted"
    })
}

/// Top-of-test guard: returns true (test should early-return) when the binary
/// is missing or accessibility is not granted. Real-app behaviour is
/// unobservable without the permission, so the suite skips rather than fails —
/// matching the permission-tolerant convention in `system_control_test.rs`.
pub fn skip_without_ax() -> bool {
    if !bin_exists() {
        eprintln!("SKIP: agent-desktop binary not built");
        return true;
    }
    if !ax_granted() {
        eprintln!("SKIP: accessibility permission not granted to test runner");
        return true;
    }
    false
}

/// Error code from a failed envelope, or empty string when `ok` / absent.
pub fn error_code(v: &Value) -> String {
    v["error"]["code"].as_str().unwrap_or_default().to_string()
}

/// Assert a command either succeeded or failed with one of the tolerated codes.
/// Used for inherently nondeterministic actions (mouse, scroll, navigation)
/// where the side effect cannot be observed reliably but the envelope contract
/// still must hold.
pub fn assert_ok_or_codes(v: &Value, command: &str, allowed: &[&str]) {
    if v["ok"] == true {
        assert_eq!(v["command"], command, "command echo must match: {v}");
        return;
    }
    let code = error_code(v);
    assert!(
        allowed.contains(&code.as_str()),
        "{command} failed with unexpected code {code:?}; allowed={allowed:?}; envelope={v}"
    );
}

/// Assert a command failed with one of the tolerated error codes. The negative
/// counterpart to [`assert_ok_or_codes`], for paths that must never succeed
/// (bogus refs, absent targets, out-of-range indices).
pub fn assert_failed_with_codes(v: &Value, command: &str, allowed: &[&str]) {
    assert_eq!(v["ok"], false, "{command} must fail: {v}");
    let code = error_code(v);
    assert!(
        allowed.contains(&code.as_str()),
        "{command} failed with unexpected code {code:?}; allowed={allowed:?}; envelope={v}"
    );
}

/// Assert every window in a `list-windows` payload belongs to `app` and carries
/// a string id — the filtered-list contract shared by app/browser tests.
pub fn assert_windows_all_belong(windows: &[Value], app: &str) {
    for w in windows {
        assert!(
            w["app_name"]
                .as_str()
                .map(|s| s.eq_ignore_ascii_case(app))
                .unwrap_or(false),
            "filtered list-windows must only return {app} windows: {w}"
        );
        assert!(w["id"].is_string(), "each window must carry an id: {w}");
    }
}

/// RAII guard that quits an app it launched, so a run never leaves stray
/// windows on the real desktop. Returns `None` when the app is not installed
/// (`APP_NOT_FOUND`) or cannot launch, letting browser/app tests skip cleanly.
pub struct AppGuard {
    app: String,
    close_on_drop: bool,
}

impl AppGuard {
    /// Launch `app` with a short timeout; `None` if it is unavailable. The app
    /// is force-quit on drop **only when this guard actually launched it** — an
    /// app that was already running (possibly with the user's unsaved work) is
    /// left exactly as it was found.
    pub fn launch(app: &str) -> Option<AppGuard> {
        let already_running = app_running(app);
        let v = run(&["launch", app, "--timeout", "12000"]);
        if v["ok"] == true {
            Some(AppGuard {
                app: app.to_string(),
                close_on_drop: !already_running,
            })
        } else {
            eprintln!("SKIP: could not launch {app}: {}", error_code(&v));
            None
        }
    }

    pub fn app(&self) -> &str {
        &self.app
    }

    /// True when this guard started the app (it was not already running). Tests
    /// that close the app explicitly should gate on this to avoid quitting an
    /// instance the user already had open.
    pub fn launched_fresh(&self) -> bool {
        self.close_on_drop
    }
}

impl Drop for AppGuard {
    fn drop(&mut self) {
        if self.close_on_drop {
            let _ = Command::new(bin())
                .args(["close-app", &self.app, "--force"])
                .output();
        }
    }
}

/// True when a GUI app with this name is currently running.
fn app_running(app: &str) -> bool {
    let v = run(&["list-apps", "--app", app]);
    v["data"]["apps"]
        .as_array()
        .map(|apps| !apps.is_empty())
        .unwrap_or(false)
}

/// Launch TextEdit, open a fresh blank document, and return its guard. `None`
/// when TextEdit is unavailable. The guard force-quits TextEdit on drop only if
/// this call started it.
pub fn fresh_textedit() -> Option<AppGuard> {
    let guard = AppGuard::launch("TextEdit")?;
    let _ = run(&["press", "cmd+n", "--app", "TextEdit"]);
    let _ = run(&["wait", "500"]);
    Some(guard)
}

/// Like [`fresh_textedit`] but also snapshots the new document and locates the
/// editor field, returning `(guard, snapshot_id, editor_ref)`. `None` when the
/// field is not exposed.
pub fn fresh_textedit_with_field() -> Option<(AppGuard, String, String)> {
    let guard = fresh_textedit()?;
    let snap = run(&["snapshot", "--app", "TextEdit", "-i"]);
    if snap["ok"] != true {
        return None;
    }
    let sid = snapshot_id(&snap)?;
    let field = find_ref_by_role(&snap["data"]["tree"], "textfield")?;
    Some((guard, sid, field))
}

/// Snapshot id from a snapshot envelope, or `None`.
pub fn snapshot_id(snap: &Value) -> Option<String> {
    snap["data"]["snapshot_id"].as_str().map(str::to_string)
}

/// First ref in document order whose node satisfies `pred`, or `None`.
fn find_ref(node: &Value, pred: &impl Fn(&Value) -> bool) -> Option<String> {
    if pred(node) {
        if let Some(r) = node.get("ref_id").and_then(|v| v.as_str()) {
            return Some(r.to_string());
        }
    }
    children(node).iter().find_map(|c| find_ref(c, pred))
}

/// First element ref found in document order, or `None`.
pub fn first_ref_id(node: &Value) -> Option<String> {
    find_ref(node, &|_| true)
}

/// First ref whose node has the given role, or `None`.
pub fn find_ref_by_role(node: &Value, role: &str) -> Option<String> {
    find_ref(node, &|n| {
        n.get("role").and_then(|r| r.as_str()) == Some(role)
    })
}

/// True when any node in the subtree has the given role.
pub fn any_node_has_role(node: &Value, role: &str) -> bool {
    if node.get("role").and_then(|r| r.as_str()) == Some(role) {
        return true;
    }
    children(node).iter().any(|c| any_node_has_role(c, role))
}

/// True when any node's name/value/description contains `needle`.
pub fn tree_contains_text(node: &Value, needle: &str) -> bool {
    for key in ["name", "value", "description"] {
        if node
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.contains(needle))
            .unwrap_or(false)
        {
            return true;
        }
    }
    children(node).iter().any(|c| tree_contains_text(c, needle))
}

fn children(node: &Value) -> &[Value] {
    node.get("children")
        .and_then(|c| c.as_array())
        .map(|c| c.as_slice())
        .unwrap_or(&[])
}
