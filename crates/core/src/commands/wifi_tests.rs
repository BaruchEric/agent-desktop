use super::*;
use crate::system::test_support::MockSystemAdapter;

#[test]
fn off_returns_power_false() {
    let adapter = MockSystemAdapter::default();
    let v = execute(
        WifiArgs {
            on: false,
            off: true,
            status: false,
        },
        &adapter,
    )
    .unwrap();
    assert_eq!(v["wifi_power"], false);
}

#[test]
fn rejects_no_action() {
    let adapter = MockSystemAdapter::default();
    let err = execute(
        WifiArgs {
            on: false,
            off: false,
            status: false,
        },
        &adapter,
    )
    .unwrap_err();
    assert!(format!("{err:?}").to_lowercase().contains("invalid"));
}
