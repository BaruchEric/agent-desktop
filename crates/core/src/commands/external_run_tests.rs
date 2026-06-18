use super::*;
use crate::system::external::ExternalKind;
use crate::system::test_support::MockSystemAdapter;

#[test]
fn disabled_returns_policy_denied() {
    let adapter = MockSystemAdapter::default();
    let err =
        run_with_gate(false, ExternalKind::Shell, "echo hi".into(), None, &adapter).unwrap_err();
    assert!(
        format!("{err:?}").contains("POLICY_DENIED") || format!("{err:?}").contains("PolicyDenied")
    );
    assert!(adapter.system.last_external().is_none());
}

#[test]
fn enabled_delegates_and_returns_result() {
    let adapter = MockSystemAdapter::default();
    let v = run_with_gate(
        true,
        ExternalKind::Shell,
        "echo hi".into(),
        Some(1000),
        &adapter,
    )
    .unwrap();
    assert_eq!(v["exit_code"], 0);
    let req = adapter.system.last_external().unwrap();
    assert_eq!(req.kind, ExternalKind::Shell);
}
