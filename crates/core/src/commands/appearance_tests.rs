use super::*;
use crate::system::test_support::MockSystemAdapter;

#[test]
fn set_dark_returns_state() {
    let adapter = MockSystemAdapter::default();
    let v = execute(
        AppearanceArgs {
            get: false,
            dark: true,
            light: false,
            toggle: false,
        },
        &adapter,
    )
    .unwrap();
    assert_eq!(v["dark"], true);
}

#[test]
fn set_light_returns_state() {
    let adapter = MockSystemAdapter::default();
    let v = execute(
        AppearanceArgs {
            get: false,
            dark: false,
            light: true,
            toggle: false,
        },
        &adapter,
    )
    .unwrap();
    assert_eq!(v["dark"], false);
}

#[test]
fn get_returns_current_state() {
    let adapter = MockSystemAdapter::default();
    let v = execute(
        AppearanceArgs {
            get: true,
            dark: false,
            light: false,
            toggle: false,
        },
        &adapter,
    )
    .unwrap();
    assert_eq!(v["dark"], false);
}

#[test]
fn toggle_flips_state() {
    let adapter = MockSystemAdapter::default();
    let v = execute(
        AppearanceArgs {
            get: false,
            dark: false,
            light: false,
            toggle: true,
        },
        &adapter,
    )
    .unwrap();
    assert_eq!(v["dark"], true);
}

#[test]
fn rejects_multiple_actions() {
    let adapter = MockSystemAdapter::default();
    let err = execute(
        AppearanceArgs {
            get: false,
            dark: true,
            light: true,
            toggle: false,
        },
        &adapter,
    )
    .unwrap_err();
    assert!(format!("{err:?}").to_lowercase().contains("invalid"));
}

#[test]
fn rejects_no_action() {
    let adapter = MockSystemAdapter::default();
    let err = execute(
        AppearanceArgs {
            get: false,
            dark: false,
            light: false,
            toggle: false,
        },
        &adapter,
    )
    .unwrap_err();
    assert!(format!("{err:?}").to_lowercase().contains("invalid"));
}
