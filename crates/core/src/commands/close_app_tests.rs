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
fn close_app_allows_ordinary_process_via_adapter() {
    let value = execute(
        CloseAppArgs {
            app: "TextEdit".into(),
            force: false,
        },
        &ProtectiveAdapter,
    )
    .unwrap();

    assert_eq!(value["closed"], true);
    assert_eq!(value["app"], "TextEdit");
}
