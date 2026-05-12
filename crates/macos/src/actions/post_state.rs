use agent_desktop_core::action::{Action, ElementState};

#[cfg(target_os = "macos")]
pub(crate) fn read_post_state(
    el: &crate::tree::AXElement,
    action: &Action,
) -> Option<ElementState> {
    let delay_ms = match action {
        Action::Click | Action::Toggle | Action::Check | Action::Uncheck | Action::TypeText(_) => {
            50
        }
        Action::SetValue(_) | Action::Clear | Action::Expand | Action::Collapse => 0,
        _ => return None,
    };
    if delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    }
    Some(read_element_state(el))
}

pub(crate) fn read_element_state(el: &crate::tree::AXElement) -> ElementState {
    let value = crate::tree::copy_value_typed(el);
    let role = crate::actions::ax_helpers::element_role(el).unwrap_or_default();
    let focused = crate::tree::element::copy_bool_attr(el, "AXFocused").unwrap_or(false);
    let enabled = crate::tree::element::copy_bool_attr(el, "AXEnabled").unwrap_or(true);
    let expanded = crate::tree::element::copy_bool_attr(el, "AXExpanded")
        .or_else(|| crate::tree::element::copy_bool_attr(el, "AXDisclosing"))
        .unwrap_or(false);
    let mut states = Vec::new();
    if focused {
        states.push("focused".into());
    }
    if !enabled {
        states.push("disabled".into());
    }
    if expanded {
        states.push("expanded".into());
    }
    if crate::tree::roles::is_toggleable_role(&role) && value_is_checked(value.as_deref()) {
        states.push("checked".into());
    }
    ElementState {
        role,
        states,
        value,
    }
}

fn value_is_checked(value: Option<&str>) -> bool {
    matches!(value, Some("1" | "true"))
}
