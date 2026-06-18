//! Real-app e2e: keyboard synthesis (press, key-down, key-up).
//!
//! Key presses are targeted at TextEdit so synthesized input never leaks to the
//! test runner's own window. Physical input can be policy-denied by default, so
//! each command is asserted as ok-or-allowed-error. key-down is always paired
//! with key-up before any assertion so a held modifier can never leak out.
#![cfg(target_os = "macos")]

mod common;
use common::*;

const KEY_CODES: &[&str] = &["ACTION_FAILED", "PERM_DENIED", "POLICY_DENIED"];

#[test]
#[ignore = "synthesizes a key combo into TextEdit"]
fn press_combo_into_textedit_envelope() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };

    let select_all = run(&["press", "cmd+a", "--app", "TextEdit"]);
    assert_ok_or_codes(&select_all, "press", KEY_CODES);

    let escape = run(&["press", "escape", "--app", "TextEdit"]);
    assert_ok_or_codes(&escape, "press", KEY_CODES);
}

#[test]
#[ignore = "holds and releases a modifier"]
fn key_down_then_up_is_balanced() {
    if skip_without_ax() {
        return;
    }
    let down = run(&["key-down", "shift"]);
    let up = run(&["key-up", "shift"]);
    assert_ok_or_codes(&down, "key-down", KEY_CODES);
    assert_ok_or_codes(&up, "key-up", KEY_CODES);
}

#[test]
#[ignore = "types characters via press into a TextEdit document"]
fn press_characters_into_editor() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = fresh_textedit() else {
        return;
    };

    let snap = run(&["snapshot", "--app", "TextEdit", "-i"]);
    if let Some(field) = find_ref_by_role(&snap["data"]["tree"], "textfield") {
        let sid = snapshot_id(&snap).unwrap_or_default();
        let _ = run(&["focus", &field, "--snapshot", &sid]);
    }

    for ch in ["k", "e", "y"] {
        let p = run(&["press", ch, "--app", "TextEdit"]);
        assert_ok_or_codes(&p, "press", KEY_CODES);
    }
}
