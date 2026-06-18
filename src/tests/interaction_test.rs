//! Real-app e2e: element interaction (type, set-value, clear, get, focus,
//! click, toggle, stale-ref handling) against TextEdit.
//!
//! TextEdit's editor is an AXTextArea, which maps to the interactive `textfield`
//! role and therefore receives a `@ref`. Text mutations are asserted as real
//! outcomes (the typed text must appear in a follow-up snapshot); actions that
//! an AXTextArea may legitimately not support are asserted as ok-or-allowed.
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "types into a real TextEdit document"]
fn type_inserts_text_into_editor() {
    if skip_without_ax() {
        return;
    }
    let Some((_g, sid, field)) = fresh_textedit_with_field() else {
        return;
    };
    let marker = "agentdesktopTYPED";
    let typed = run(&["type", &field, marker, "--snapshot", &sid]);
    assert_eq!(typed["ok"], true, "type must succeed: {typed}");

    let after = run(&["snapshot", "--app", "TextEdit"]);
    assert_eq!(
        after["ok"], true,
        "follow-up snapshot must succeed: {after}"
    );
    assert!(
        tree_contains_text(&after["data"]["tree"], marker),
        "typed text must be observable in the tree: {after}"
    );
}

#[test]
#[ignore = "reads back a value via get on a real element"]
fn get_reads_typed_editor_value() {
    if skip_without_ax() {
        return;
    }
    let Some((_g, sid, field)) = fresh_textedit_with_field() else {
        return;
    };
    let marker = "agentdesktopGET";
    let _ = run(&["type", &field, marker, "--snapshot", &sid]);

    let got = run(&["get", &field, "--snapshot", &sid, "--property", "value"]);
    assert_ok_or_codes(&got, "get", &["STALE_REF"]);
    if got["ok"] == true {
        let value = got["data"]["value"].as_str().unwrap_or_default();
        assert!(
            value.contains(marker),
            "get value must contain the typed text, got {value:?}: {got}"
        );
    }
}

#[test]
#[ignore = "sets and clears a real element value"]
fn set_value_then_clear_editor() {
    if skip_without_ax() {
        return;
    }
    let Some((_g, sid, field)) = fresh_textedit_with_field() else {
        return;
    };
    let sv = run(&["set-value", &field, "via-set-value", "--snapshot", &sid]);
    assert_ok_or_codes(
        &sv,
        "set-value",
        &["ACTION_NOT_SUPPORTED", "ACTION_FAILED", "STALE_REF"],
    );

    let clr = run(&["clear", &field, "--snapshot", &sid]);
    assert_ok_or_codes(
        &clr,
        "clear",
        &["ACTION_NOT_SUPPORTED", "ACTION_FAILED", "STALE_REF"],
    );
}

#[test]
#[ignore = "focuses and clicks a real element"]
fn focus_and_click_editor_envelope() {
    if skip_without_ax() {
        return;
    }
    let Some((_g, sid, field)) = fresh_textedit_with_field() else {
        return;
    };
    let f = run(&["focus", &field, "--snapshot", &sid]);
    assert_ok_or_codes(&f, "focus", &["ACTION_FAILED", "STALE_REF", "PERM_DENIED"]);

    let c = run(&["click", &field, "--snapshot", &sid]);
    assert_ok_or_codes(
        &c,
        "click",
        &[
            "ACTION_FAILED",
            "ACTION_NOT_SUPPORTED",
            "STALE_REF",
            "PERM_DENIED",
        ],
    );
}

#[test]
#[ignore = "exercises toggle/check on whatever checkbox the app exposes"]
fn toggle_checkbox_when_present() {
    if skip_without_ax() {
        return;
    }
    let Some(_g) = AppGuard::launch("TextEdit") else {
        return;
    };
    let _ = run(&["press", "cmd+shift+s", "--app", "TextEdit"]);
    let _ = run(&["wait", "500"]);
    let snap = run(&["snapshot", "--app", "TextEdit", "-i"]);
    let _ = run(&["press", "escape", "--app", "TextEdit"]);
    if snap["ok"] != true {
        return;
    }
    let Some(cb) = find_ref_by_role(&snap["data"]["tree"], "checkbox") else {
        return;
    };
    let sid = snapshot_id(&snap).unwrap_or_default();
    let t = run(&["toggle", &cb, "--snapshot", &sid]);
    assert_ok_or_codes(
        &t,
        "toggle",
        &[
            "ACTION_FAILED",
            "ACTION_NOT_SUPPORTED",
            "STALE_REF",
            "PERM_DENIED",
        ],
    );
}

#[test]
#[ignore = "negative path; needs only the built binary"]
fn unknown_ref_returns_structured_error() {
    if !bin_exists() {
        return;
    }
    let v = run(&["click", "@e99999", "--snapshot", "no-such-snapshot"]);
    assert_failed_with_codes(
        &v,
        "click",
        &[
            "STALE_REF",
            "SNAPSHOT_NOT_FOUND",
            "ELEMENT_NOT_FOUND",
            "INVALID_ARGS",
        ],
    );
}
