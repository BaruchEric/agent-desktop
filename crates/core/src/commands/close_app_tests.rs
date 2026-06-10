use super::*;
use crate::adapter::PlatformAdapter;

struct ProtectiveAdapter;

impl PlatformAdapter for ProtectiveAdapter {
    fn is_protected_process(&self, identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("CriticalThing")
    }

    fn close_app(&self, _id: &str, _force: bool) -> Result<(), crate::error::AdapterError> {
        Ok(())
    }
}

#[test]
fn close_app_blocks_adapter_protected_process() {
    let err = execute(
        CloseAppArgs {
            app: "CriticalThing".into(),
            force: false,
        },
        &ProtectiveAdapter,
    )
    .unwrap_err();

    assert_eq!(err.code(), "INVALID_ARGS");
    assert!(err.to_string().contains("protected"));
}

#[test]
fn graceful_close_reports_request_without_claiming_termination() {
    let value = execute(
        CloseAppArgs {
            app: "TextEdit".into(),
            force: false,
        },
        &ProtectiveAdapter,
    )
    .unwrap();

    assert_eq!(value["app"], "TextEdit");
    assert_eq!(value["method"], "graceful");
    assert_eq!(value["requested"], true);
    assert!(
        value.get("closed").is_none(),
        "graceful close must not claim a termination it cannot guarantee"
    );
}

#[test]
fn forced_close_confirms_termination() {
    let value = execute(
        CloseAppArgs {
            app: "TextEdit".into(),
            force: true,
        },
        &ProtectiveAdapter,
    )
    .unwrap();

    assert_eq!(value["app"], "TextEdit");
    assert_eq!(value["method"], "force");
    assert_eq!(value["requested"], true);
    assert_eq!(value["closed"], true);
}
