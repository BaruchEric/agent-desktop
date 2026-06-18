//! Real-app e2e: Notification Center commands.
//!
//! A notification cannot be made to arrive on demand, so these assert the
//! envelope contract: listing returns a well-formed shape, dismiss-all succeeds
//! or reports nothing to dismiss, and an out-of-range dismiss fails cleanly.
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "queries Notification Center"]
fn list_notifications_returns_envelope() {
    if !bin_exists() {
        return;
    }
    let v = run(&["list-notifications"]);
    assert_ok_or_codes(&v, "list-notifications", &["PERM_DENIED", "ACTION_FAILED"]);
    if v["ok"] == true {
        assert!(
            v["data"]["notifications"].is_array(),
            "notifications must be an array: {v}"
        );
        assert!(
            v["data"]["count"].is_number(),
            "count must be a number: {v}"
        );
    }
}

#[test]
#[ignore = "queries Notification Center"]
fn dismiss_all_notifications_envelope() {
    if !bin_exists() {
        return;
    }
    let v = run(&["dismiss-all-notifications"]);
    assert_ok_or_codes(
        &v,
        "dismiss-all-notifications",
        &["PERM_DENIED", "ACTION_FAILED", "NOTIFICATION_NOT_FOUND"],
    );
}

#[test]
#[ignore = "queries Notification Center"]
fn dismiss_out_of_range_index_fails_cleanly() {
    if !bin_exists() {
        return;
    }
    let v = run(&["dismiss-notification", "99999"]);
    assert_failed_with_codes(
        &v,
        "dismiss-notification",
        &[
            "NOTIFICATION_NOT_FOUND",
            "INVALID_ARGS",
            "PERM_DENIED",
            "ACTION_FAILED",
        ],
    );
}
