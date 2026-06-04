use agent_desktop_core::{
    action_request::ActionRequest,
    action_result::ActionResult,
    adapter::{LiveElement, NativeHandle, PlatformAdapter, SnapshotSurface},
    element_state::ElementState,
    error::AdapterError,
    node::Rect,
    refs::RefEntry,
};
use std::sync::atomic::{AtomicU32, Ordering};

#[path = "../../tests/conformance/ref_action_contract.rs"]
mod ref_action_contract;

struct ContractAdapter {
    live_bounds: Option<Rect>,
    dispatches: AtomicU32,
}

impl PlatformAdapter for ContractAdapter {
    fn resolve_element_strict(&self, _entry: &RefEntry) -> Result<NativeHandle, AdapterError> {
        Ok(NativeHandle::null())
    }

    fn get_live_element(&self, _handle: &NativeHandle) -> Result<LiveElement, AdapterError> {
        Ok(LiveElement {
            state: Some(ElementState {
                role: "button".into(),
                states: vec![],
                value: None,
            }),
            bounds: self.live_bounds,
            available_actions: Some(vec!["Click".into()]),
        })
    }

    fn execute_action(
        &self,
        _handle: &NativeHandle,
        _request: ActionRequest,
    ) -> Result<ActionResult, AdapterError> {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        Ok(ActionResult::new("click"))
    }
}

fn entry(bounds: Rect) -> RefEntry {
    RefEntry {
        pid: 1,
        role: "button".into(),
        name: Some("OK".into()),
        value: None,
        description: None,
        states: vec![],
        bounds: Some(bounds),
        bounds_hash: Some(bounds.bounds_hash()),
        available_actions: vec!["Click".into()],
        source_app: None,
        source_window_id: None,
        source_window_title: None,
        source_surface: SnapshotSurface::Window,
        root_ref: None,
        path_is_absolute: true,
        path: Default::default(),
    }
}

#[test]
fn adapter_contract_dispatches_when_live_identity_moved() {
    let snapshot_bounds = Rect {
        x: 1.0,
        y: 1.0,
        width: 20.0,
        height: 20.0,
    };
    let adapter = ContractAdapter {
        live_bounds: Some(Rect {
            x: 100.0,
            y: 100.0,
            width: 20.0,
            height: 20.0,
        }),
        dispatches: AtomicU32::new(0),
    };

    let result = ref_action_contract::run_click_command(&adapter, entry(snapshot_bounds)).unwrap();

    assert_eq!(result["action"], "click");
    assert_eq!(adapter.dispatches.load(Ordering::SeqCst), 1);
}

#[test]
fn adapter_contract_dispatches_when_live_identity_is_stable() {
    let bounds = Rect {
        x: 1.0,
        y: 1.0,
        width: 20.0,
        height: 20.0,
    };
    let adapter = ContractAdapter {
        live_bounds: Some(bounds),
        dispatches: AtomicU32::new(0),
    };

    let result = ref_action_contract::run_click_command(&adapter, entry(bounds)).unwrap();

    assert_eq!(result["action"], "click");
    assert_eq!(adapter.dispatches.load(Ordering::SeqCst), 1);
}
