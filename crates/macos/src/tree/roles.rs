/// Native AX role -> canonical role. Single source of truth for the macOS
/// role mapping, sorted by AX name for binary-search lookup. A conformance
/// test asserts every emitted role is in core's `CANONICAL_ROLES`, so the
/// cross-platform vocabulary cannot silently drift from what this adapter
/// emits. Windows/Linux adapters must ship the same table + test pair for
/// their native vocabularies (UIA control types, AT-SPI roles).
const AX_ROLE_MAP: &[(&str, &str)] = &[
    ("AXApplication", "application"),
    ("AXBrowser", "browser"),
    ("AXBusyIndicator", "progressbar"),
    ("AXButton", "button"),
    ("AXCell", "cell"),
    ("AXCheckBox", "checkbox"),
    ("AXColorWell", "colorwell"),
    ("AXColumn", "column"),
    ("AXComboBox", "combobox"),
    ("AXDateField", "datefield"),
    ("AXDialog", "dialog"),
    ("AXDisclosureTriangle", "disclosure"),
    ("AXDockItem", "dockitem"),
    ("AXDrawer", "drawer"),
    ("AXGenericElement", "group"),
    ("AXGrid", "grid"),
    ("AXGroup", "group"),
    ("AXHandle", "handle"),
    ("AXHelpTag", "helptag"),
    ("AXImage", "image"),
    ("AXIncrementor", "incrementor"),
    ("AXLayoutArea", "layoutitem"),
    ("AXLayoutItem", "layoutitem"),
    ("AXLevelIndicator", "levelindicator"),
    ("AXLink", "link"),
    ("AXList", "list"),
    ("AXMatte", "matte"),
    ("AXMenu", "menu"),
    ("AXMenuBar", "menu"),
    ("AXMenuBarItem", "menuitem"),
    ("AXMenuButton", "menubutton"),
    ("AXMenuItem", "menuitem"),
    ("AXOutline", "outline"),
    ("AXOutlineRow", "treeitem"),
    ("AXPopUpButton", "combobox"),
    ("AXPopover", "popover"),
    ("AXProgressIndicator", "progressbar"),
    ("AXRadioButton", "radiobutton"),
    ("AXRelevanceIndicator", "relevanceindicator"),
    ("AXRow", "treeitem"),
    ("AXRuler", "ruler"),
    ("AXRulerMarker", "rulermarker"),
    ("AXScrollArea", "scrollarea"),
    ("AXScrollBar", "scrollarea"),
    ("AXSearchField", "textfield"),
    ("AXSecureTextField", "textfield"),
    ("AXSheet", "sheet"),
    ("AXSlider", "slider"),
    ("AXSplitGroup", "splitter"),
    ("AXSplitter", "splitter"),
    ("AXStaticText", "statictext"),
    ("AXStepper", "incrementor"),
    ("AXSwitch", "switch"),
    ("AXTab", "tab"),
    ("AXTabGroup", "tab"),
    ("AXTable", "table"),
    ("AXTextArea", "textfield"),
    ("AXTextField", "textfield"),
    ("AXTimeField", "timefield"),
    ("AXToggle", "switch"),
    ("AXToolbar", "toolbar"),
    ("AXValueIndicator", "slider"),
    ("AXWebArea", "webarea"),
    ("AXWindow", "window"),
];

pub fn ax_role_to_str(ax_role: &str) -> &'static str {
    AX_ROLE_MAP
        .binary_search_by_key(&ax_role, |(ax, _)| *ax)
        .map(|idx| AX_ROLE_MAP[idx].1)
        .unwrap_or("unknown")
}

pub fn normalized_role_for_element(el: &crate::tree::AXElement, ax_role: Option<&str>) -> String {
    normalized_role_and_label(el, ax_role).0
}

pub fn normalized_role_and_label(
    el: &crate::tree::AXElement,
    ax_role: Option<&str>,
) -> (String, Option<String>) {
    let promoted_label = promoted_item_label(ax_role, el);
    let role = if promoted_label.is_some() {
        "cell"
    } else {
        ax_role.map(ax_role_to_str).unwrap_or("unknown")
    };
    (role.to_string(), promoted_label)
}

pub fn promoted_item_label(ax_role: Option<&str>, el: &crate::tree::AXElement) -> Option<String> {
    if ax_role != Some("AXGroup") {
        return None;
    }
    let children = crate::tree::element::child_attributes(ax_role)
        .iter()
        .find_map(|attr| {
            crate::tree::copy_ax_array(el, attr).filter(|children| !children.is_empty())
        })
        .unwrap_or_default();
    let has_icon = children
        .iter()
        .any(|child| crate::tree::copy_string_attr(child, "AXRole").as_deref() == Some("AXImage"));
    if !has_icon {
        return None;
    }
    children.iter().find_map(|child| {
        if crate::tree::copy_string_attr(child, "AXRole").as_deref() == Some("AXTextField") {
            crate::tree::copy_string_attr(child, "AXValue").filter(|value| !value.is_empty())
        } else {
            None
        }
    })
}

pub use agent_desktop_core::roles::is_toggleable_role;

#[cfg(test)]
mod tests {
    use super::*;
    use agent_desktop_core::roles::CANONICAL_ROLES;

    #[test]
    fn ax_role_map_is_sorted_and_unique_for_binary_search() {
        for window in AX_ROLE_MAP.windows(2) {
            assert!(
                window[0].0 < window[1].0,
                "AX_ROLE_MAP must stay sorted/unique: {} >= {}",
                window[0].0,
                window[1].0
            );
        }
    }

    #[test]
    fn every_emitted_role_is_in_core_canonical_vocabulary() {
        for (ax, canonical) in AX_ROLE_MAP {
            assert!(
                CANONICAL_ROLES.contains(canonical),
                "adapter emits '{canonical}' for {ax} but core CANONICAL_ROLES lacks it"
            );
        }
        assert!(
            CANONICAL_ROLES.contains(&"unknown"),
            "fallback role must be canonical"
        );
        assert!(
            CANONICAL_ROLES.contains(&"cell"),
            "promoted-item synthesized role must be canonical"
        );
    }

    #[test]
    fn lookup_maps_known_roles_and_falls_back_to_unknown() {
        assert_eq!(ax_role_to_str("AXTextArea"), "textfield");
        assert_eq!(ax_role_to_str("AXPopUpButton"), "combobox");
        assert_eq!(ax_role_to_str("AXWindow"), "window");
        assert_eq!(ax_role_to_str("AXSomeVendorCustom"), "unknown");
    }
}
