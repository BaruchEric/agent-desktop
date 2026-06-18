//! Real-app e2e: scroll and scroll-to.
//!
//! A scroll's visual effect is not reliably observable through the tree, so the
//! happy paths assert the envelope contract against a real ref discovered in
//! Finder. The bogus-ref path is a deterministic negative assertion.
#![cfg(target_os = "macos")]

mod common;
use common::*;

const SCROLL_CODES: &[&str] = &[
    "ACTION_FAILED",
    "ACTION_NOT_SUPPORTED",
    "STALE_REF",
    "PERM_DENIED",
    "POLICY_DENIED",
];

/// Snapshot Finder (always running) and return (snapshot_id, first_ref).
fn finder_ref() -> Option<(String, String)> {
    let snap = run(&["snapshot", "--app", "Finder", "-i"]);
    if snap["ok"] != true {
        return None;
    }
    let sid = snapshot_id(&snap)?;
    let r = first_ref_id(&snap["data"]["tree"])?;
    Some((sid, r))
}

#[test]
#[ignore = "scrolls a real Finder element"]
fn scroll_found_ref_envelope() {
    if skip_without_ax() {
        return;
    }
    let Some((sid, r)) = finder_ref() else {
        return;
    };
    let s = run(&[
        "scroll",
        &r,
        "--direction",
        "down",
        "--amount",
        "3",
        "--snapshot",
        &sid,
    ]);
    assert_ok_or_codes(&s, "scroll", SCROLL_CODES);
}

#[test]
#[ignore = "scrolls a real Finder element into view"]
fn scroll_to_found_ref_envelope() {
    if skip_without_ax() {
        return;
    }
    let Some((sid, r)) = finder_ref() else {
        return;
    };
    let s = run(&["scroll-to", &r, "--snapshot", &sid]);
    assert_ok_or_codes(&s, "scroll-to", SCROLL_CODES);
}

#[test]
#[ignore = "negative path; needs only the built binary"]
fn scroll_unknown_ref_returns_error() {
    if !bin_exists() {
        return;
    }
    let v = run(&[
        "scroll",
        "@e99999",
        "--snapshot",
        "no-such-snapshot",
        "--direction",
        "down",
    ]);
    assert_failed_with_codes(
        &v,
        "scroll",
        &[
            "STALE_REF",
            "SNAPSHOT_NOT_FOUND",
            "ELEMENT_NOT_FOUND",
            "INVALID_ARGS",
        ],
    );
}
