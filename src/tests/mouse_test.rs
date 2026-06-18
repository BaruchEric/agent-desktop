//! Real-app e2e: mouse synthesis (hover, mouse-move, mouse-click, mouse-down,
//! mouse-up, drag).
//!
//! The default interaction policy forbids cursor movement, so most of these are
//! expected to return POLICY_DENIED on a stock setup; the suite asserts the
//! envelope contract (ok-or-allowed-error) rather than observing the cursor.
//! mouse-down is always followed by mouse-up so a button is never left held.
#![cfg(target_os = "macos")]

mod common;
use common::*;

const MOUSE_CODES: &[&str] = &["PERM_DENIED", "POLICY_DENIED", "ACTION_FAILED"];

#[test]
#[ignore = "synthesizes cursor movement"]
fn mouse_move_envelope() {
    if !bin_exists() {
        return;
    }
    let m = run(&["mouse-move", "--xy", "400,400"]);
    assert_ok_or_codes(&m, "mouse-move", MOUSE_CODES);
    if m["ok"] == true {
        assert_eq!(m["data"]["moved"], true, "mouse-move payload: {m}");
    }
}

#[test]
#[ignore = "synthesizes a hover"]
fn hover_xy_envelope() {
    if !bin_exists() {
        return;
    }
    let h = run(&["hover", "--xy", "420,420", "--duration", "100"]);
    assert_ok_or_codes(&h, "hover", MOUSE_CODES);
    if h["ok"] == true {
        assert_eq!(h["data"]["hovered"], true, "hover payload: {h}");
    }
}

#[test]
#[ignore = "synthesizes a click at a screen point"]
fn mouse_click_envelope() {
    if !bin_exists() {
        return;
    }
    let c = run(&[
        "mouse-click",
        "--xy",
        "500,500",
        "--button",
        "left",
        "--count",
        "1",
    ]);
    assert_ok_or_codes(&c, "mouse-click", MOUSE_CODES);
    if c["ok"] == true {
        assert_eq!(c["data"]["clicked"], true, "mouse-click payload: {c}");
    }
}

#[test]
#[ignore = "presses then releases the mouse button"]
fn mouse_down_then_up_is_balanced() {
    if !bin_exists() {
        return;
    }
    let _ = run(&["mouse-move", "--xy", "450,450"]);
    let down = run(&["mouse-down", "--xy", "450,450", "--button", "left"]);
    let up = run(&["mouse-up", "--xy", "450,450", "--button", "left"]);
    assert_ok_or_codes(&down, "mouse-down", MOUSE_CODES);
    assert_ok_or_codes(&up, "mouse-up", MOUSE_CODES);
}

#[test]
#[ignore = "synthesizes a drag between two points"]
fn drag_between_points_envelope() {
    if !bin_exists() {
        return;
    }
    let d = run(&[
        "drag",
        "--from-xy",
        "400,400",
        "--to-xy",
        "500,500",
        "--duration",
        "200",
    ]);
    assert_ok_or_codes(&d, "drag", MOUSE_CODES);
    if d["ok"] == true {
        assert_eq!(d["data"]["dragged"], true, "drag payload: {d}");
    }
}
