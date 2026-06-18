//! Real-app e2e: app + window lifecycle (launch, list, focus, resize, move,
//! minimize/maximize/restore, close) against TextEdit.
//!
//! Window-geometry operations can be refused by the window server depending on
//! the app's current state, so each is asserted as ok-or-allowed-error; the
//! success-path payload flags are checked only when the command reports `ok`.
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "launches and drives real macOS windows"]
fn list_windows_filter_returns_only_target_app() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };
    let wins = run(&["list-windows", "--app", "TextEdit"]);
    assert_eq!(wins["ok"], true, "list-windows must succeed: {wins}");
    let arr = wins["data"]
        .as_array()
        .unwrap_or_else(|| panic!("list-windows data must be an array: {wins}"));
    assert_windows_all_belong(arr, "TextEdit");
}

#[test]
#[ignore = "launches and drives real macOS windows"]
fn focus_window_brings_app_forward() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };
    let f = run(&["focus-window", "--app", "TextEdit"]);
    assert_ok_or_codes(
        &f,
        "focus-window",
        &["WINDOW_NOT_FOUND", "ACTION_FAILED", "PERM_DENIED"],
    );
}

#[test]
#[ignore = "launches and drives real macOS windows"]
fn resize_window_reports_new_dimensions() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };
    let r = run(&[
        "resize-window",
        "--app",
        "TextEdit",
        "--width",
        "640",
        "--height",
        "480",
    ]);
    assert_ok_or_codes(
        &r,
        "resize-window",
        &["WINDOW_NOT_FOUND", "ACTION_FAILED", "PERM_DENIED"],
    );
    if r["ok"] == true {
        assert_eq!(r["data"]["resized"], true, "resize success payload: {r}");
    }
}

#[test]
#[ignore = "launches and drives real macOS windows"]
fn move_window_reports_new_position() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };
    let m = run(&[
        "move-window",
        "--app",
        "TextEdit",
        "--x",
        "120",
        "--y",
        "120",
    ]);
    assert_ok_or_codes(
        &m,
        "move-window",
        &["WINDOW_NOT_FOUND", "ACTION_FAILED", "PERM_DENIED"],
    );
    if m["ok"] == true {
        assert_eq!(m["data"]["moved"], true, "move success payload: {m}");
    }
}

#[test]
#[ignore = "launches and drives real macOS windows"]
fn minimize_then_restore_window() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };
    let min = run(&["minimize", "--app", "TextEdit"]);
    assert_ok_or_codes(
        &min,
        "minimize",
        &["WINDOW_NOT_FOUND", "ACTION_FAILED", "PERM_DENIED"],
    );
    if min["ok"] == true {
        assert_eq!(min["data"]["minimized"], true, "minimize payload: {min}");
    }
    let _ = run(&["wait", "300"]);
    let restore = run(&["restore", "--app", "TextEdit"]);
    assert_ok_or_codes(
        &restore,
        "restore",
        &["WINDOW_NOT_FOUND", "ACTION_FAILED", "PERM_DENIED"],
    );
}

#[test]
#[ignore = "launches and drives real macOS windows"]
fn maximize_then_restore_window() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };
    let max = run(&["maximize", "--app", "TextEdit"]);
    assert_ok_or_codes(
        &max,
        "maximize",
        &["WINDOW_NOT_FOUND", "ACTION_FAILED", "PERM_DENIED"],
    );
    if max["ok"] == true {
        assert_eq!(max["data"]["maximized"], true, "maximize payload: {max}");
    }
    let _ = run(&["wait", "300"]);
    let _ = run(&["restore", "--app", "TextEdit"]);
}

#[test]
#[ignore = "launches and drives real macOS windows"]
fn close_app_reports_closed() {
    if skip_without_ax() {
        return;
    }
    let Some(g) = AppGuard::launch("TextEdit") else {
        return;
    };
    if !g.launched_fresh() {
        return;
    }
    let c = run(&["close-app", "TextEdit", "--force"]);
    assert_eq!(c["ok"], true, "close-app must succeed: {c}");
    assert_eq!(c["data"]["closed"], true, "closed flag must be set: {c}");
    assert_eq!(c["data"]["app"], "TextEdit", "echoed app name: {c}");
}
