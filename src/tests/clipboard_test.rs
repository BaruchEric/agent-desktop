//! Real-app e2e: clipboard get/set/clear round-trips.
//!
//! The most deterministic slice of the suite — it asserts real outcomes
//! through the macOS pasteboard without needing Accessibility permission or a
//! visible app. Gated `#[ignore]` so a bare `cargo test` never mutates the
//! user's clipboard.
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "mutates the macOS clipboard"]
fn clipboard_set_get_roundtrips() {
    if !bin_exists() {
        return;
    }
    let marker = "agent-desktop-e2e-clipboard";
    let set = run(&["clipboard-set", marker]);
    assert_eq!(set["ok"], true, "clipboard-set must succeed: {set}");

    let get = run(&["clipboard-get"]);
    assert_eq!(get["ok"], true, "clipboard-get must succeed: {get}");
    assert_eq!(
        get["data"]["text"], marker,
        "clipboard must round-trip the written text: {get}"
    );
}

#[test]
#[ignore = "mutates the macOS clipboard"]
fn clipboard_set_overwrites_previous() {
    if !bin_exists() {
        return;
    }
    assert_eq!(run(&["clipboard-set", "first-value"])["ok"], true);
    assert_eq!(run(&["clipboard-set", "second-value"])["ok"], true);

    let get = run(&["clipboard-get"]);
    assert_eq!(
        get["data"]["text"], "second-value",
        "the most recent write must win: {get}"
    );
}

#[test]
#[ignore = "mutates the macOS clipboard"]
fn clipboard_clear_empties_contents() {
    if !bin_exists() {
        return;
    }
    assert_eq!(run(&["clipboard-set", "to-be-cleared"])["ok"], true);

    let cleared = run(&["clipboard-clear"]);
    assert_eq!(
        cleared["ok"], true,
        "clipboard-clear must succeed: {cleared}"
    );

    let get = run(&["clipboard-get"]);
    assert_ok_or_codes(&get, "clipboard-get", &["ACTION_FAILED"]);
    if get["ok"] == true {
        assert_eq!(
            get["data"]["text"], "",
            "cleared clipboard must read back empty: {get}"
        );
    }
}
