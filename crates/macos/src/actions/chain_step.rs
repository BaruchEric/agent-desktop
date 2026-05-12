use agent_desktop_core::{action::MouseButton, error::AdapterError};

use crate::{actions::discovery::ElementCaps, tree::AXElement};

pub(crate) enum ChainStep {
    Action(&'static str),
    SetBool {
        attr: &'static str,
        value: bool,
    },
    SetDynamic {
        attr: &'static str,
    },
    FocusThenSetDynamic {
        attr: &'static str,
    },
    FocusThenClearByKeyboard,
    ChildActions {
        actions: &'static [&'static str],
        limit: usize,
    },
    AncestorActions {
        actions: &'static [&'static str],
        limit: usize,
    },
    Custom {
        label: &'static str,
        func: fn(&AXElement, &ElementCaps) -> Result<bool, AdapterError>,
    },
    CGClick {
        button: MouseButton,
        count: u32,
    },
}
