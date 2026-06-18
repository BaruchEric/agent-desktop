//! Real-app e2e: the `wait` command across its modes.
//!
//! Sleep is fully deterministic (it must report the elapsed budget). The
//! presence waits assert the timeout/negative path against deliberately absent
//! targets so the suite never depends on transient UI timing.
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "exercises the sleep mode of wait"]
fn wait_sleep_reports_waited_ms() {
    if !bin_exists() {
        return;
    }
    let v = run(&["wait", "150"]);
    assert_eq!(v["ok"], true, "wait sleep must succeed: {v}");
    assert_eq!(
        v["data"]["waited_ms"], 150,
        "elapsed budget must echo back: {v}"
    );
}

#[test]
#[ignore = "requires Accessibility permission"]
fn wait_for_absent_window_times_out() {
    if skip_without_ax() {
        return;
    }
    let v = run(&[
        "wait",
        "--window",
        "zzz-nonexistent-window-zzz",
        "--timeout",
        "800",
    ]);
    assert_failed_with_codes(&v, "wait", &["TIMEOUT", "WINDOW_NOT_FOUND"]);
}

#[test]
#[ignore = "requires Accessibility permission and TextEdit"]
fn wait_for_absent_text_times_out() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = AppGuard::launch("TextEdit") else {
        return;
    };
    let v = run(&[
        "wait",
        "--text",
        "zzz-absent-text-zzz",
        "--app",
        "TextEdit",
        "--timeout",
        "800",
    ]);
    assert_failed_with_codes(
        &v,
        "wait",
        &["TIMEOUT", "ELEMENT_NOT_FOUND", "ACTION_FAILED"],
    );
}
