use super::*;
use crate::system::external::ExternalKind;
use crate::system::test_support::MockSystemAdapter;

#[test]
fn run_with_gate_forwards_each_kind() {
    for kind in [
        ExternalKind::Shell,
        ExternalKind::AppleScript,
        ExternalKind::Jxa,
        ExternalKind::OpenUrl,
        ExternalKind::OpenPath,
    ] {
        let adapter = MockSystemAdapter::default();
        run_with_gate(true, kind, "payload".into(), None, &adapter).unwrap();
        assert_eq!(adapter.system.last_external().unwrap().kind, kind);
    }
}
