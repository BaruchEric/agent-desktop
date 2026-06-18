use super::*;
use crate::system::test_support::MockSystemAdapter;

fn args() -> VolumeArgs {
    VolumeArgs {
        get: false,
        set: None,
        up: false,
        down: false,
        mute: false,
        unmute: false,
        step: 5,
    }
}

#[test]
fn set_volume_returns_state() {
    let adapter = MockSystemAdapter::default();
    let v = execute(
        VolumeArgs {
            set: Some(40),
            ..args()
        },
        &adapter,
    )
    .unwrap();
    assert_eq!(v["output_volume"], 40);
}

#[test]
fn rejects_no_action() {
    let adapter = MockSystemAdapter::default();
    let err = execute(args(), &adapter).unwrap_err();
    assert!(
        format!("{err:?}").contains("INVALID_ARGS") || format!("{err:?}").contains("InvalidArgs")
    );
}

#[test]
fn rejects_out_of_range() {
    let adapter = MockSystemAdapter::default();
    let err = execute(
        VolumeArgs {
            set: Some(150),
            ..args()
        },
        &adapter,
    )
    .unwrap_err();
    assert!(format!("{err:?}").to_lowercase().contains("invalid"));
}
