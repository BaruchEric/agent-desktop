//! Real-app e2e: drive web browsers as desktop accessibility apps.
//!
//! agent-desktop treats a browser like any other app — it reads the window's
//! accessibility tree and operates the browser chrome (toolbar buttons, the
//! address field), never the web-page DOM (that is agent-browser's job). Safari
//! ships with every macOS install; other browsers are probed at runtime and
//! skipped when absent. Navigation depends on the network, timing, and focus,
//! so it is asserted only at the envelope level, never as "navigated to X".
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "requires Accessibility permission and Safari"]
fn safari_snapshot_yields_interactive_refs() {
    if skip_without_ax() {
        return;
    }
    let Some(_guard) = AppGuard::launch("Safari") else {
        return;
    };
    let _ = run(&["wait", "600"]);

    let snap = run(&["snapshot", "--app", "Safari", "-i"]);
    if snap["ok"] != true {
        return;
    }
    assert!(
        snap["data"]["ref_count"].as_u64().unwrap_or(0) > 0,
        "Safari window snapshot must allocate at least one ref: {snap}"
    );
    assert!(
        snap["data"]["tree"].is_object(),
        "Safari snapshot must carry a tree object: {snap}"
    );
}

#[test]
#[ignore = "requires Accessibility permission and Safari"]
fn safari_window_is_listed_for_app() {
    if skip_without_ax() {
        return;
    }
    let Some(_guard) = AppGuard::launch("Safari") else {
        return;
    };
    let _ = run(&["wait", "600"]);

    let wins = run(&["list-windows", "--app", "Safari"]);
    assert_eq!(wins["ok"], true, "list-windows must succeed: {wins}");
    let arr = wins["data"]
        .as_array()
        .unwrap_or_else(|| panic!("list-windows data must be an array: {wins}"));
    assert_windows_all_belong(arr, "Safari");
}

#[test]
#[ignore = "requires Accessibility permission and Safari; navigation is envelope-only"]
fn safari_address_field_navigation_envelope() {
    if skip_without_ax() {
        return;
    }
    let Some(_guard) = AppGuard::launch("Safari") else {
        return;
    };
    let _ = run(&["wait", "700"]);

    let snap = run(&["snapshot", "--app", "Safari", "-i"]);
    if snap["ok"] != true {
        return;
    }
    let sid = snapshot_id(&snap).unwrap_or_default();
    let tree = &snap["data"]["tree"];

    let field = find_ref_by_role(tree, "textfield").or_else(|| find_ref_by_role(tree, "combobox"));
    let Some(field) = field else {
        return;
    };

    let typed = run(&["type", &field, "example.com", "--snapshot", &sid]);
    assert_ok_or_codes(
        &typed,
        "type",
        &[
            "STALE_REF",
            "ACTION_FAILED",
            "ACTION_NOT_SUPPORTED",
            "PERM_DENIED",
            "POLICY_DENIED",
        ],
    );

    let go = run(&["press", "return", "--app", "Safari"]);
    assert_ok_or_codes(
        &go,
        "press",
        &["ACTION_FAILED", "PERM_DENIED", "POLICY_DENIED"],
    );
}

#[test]
#[ignore = "requires Accessibility permission; skipped when Chrome is not installed"]
fn chrome_snapshot_when_installed() {
    if skip_without_ax() {
        return;
    }
    let Some(_guard) = AppGuard::launch("Google Chrome") else {
        return;
    };
    let _ = run(&["wait", "900"]);

    let snap = run(&["snapshot", "--app", "Google Chrome", "-i"]);
    if snap["ok"] != true {
        return;
    }
    assert!(
        snap["data"]["ref_count"].as_u64().unwrap_or(0) > 0,
        "Chrome window snapshot must allocate at least one ref: {snap}"
    );
}
