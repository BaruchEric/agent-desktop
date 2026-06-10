/// Interactive roles that receive refs during snapshot allocation.
///
/// Each entry must be produced by at least one platform adapter's native-to-canonical
/// role mapping. Read-only roles (statictext, image) and container roles (group, list,
/// table) stay out. Platform-private extensions live in the adapter, not here.
pub const INTERACTIVE_ROLES: &[&str] = &[
    "button",
    "cell",
    "checkbox",
    "colorwell",
    "combobox",
    "dockitem",
    "incrementor",
    "link",
    "menubutton",
    "menuitem",
    "radiobutton",
    "slider",
    "switch",
    "tab",
    "textfield",
    "treeitem",
];

/// Every role the platform adapters can emit, sorted. This is the
/// cross-platform vocabulary contract: each adapter's role-mapping
/// conformance test asserts every role it emits is listed here (see the
/// macOS `AX_ROLE_MAP` tests; Windows/Linux adapters must ship the same
/// table + test pair). Role queries are validated against this list so an
/// impossible role fails loudly instead of silently matching nothing.
pub const CANONICAL_ROLES: &[&str] = &[
    "application",
    "browser",
    "button",
    "cell",
    "checkbox",
    "colorwell",
    "column",
    "combobox",
    "datefield",
    "dialog",
    "disclosure",
    "dockitem",
    "drawer",
    "grid",
    "group",
    "handle",
    "helptag",
    "image",
    "incrementor",
    "layoutitem",
    "levelindicator",
    "link",
    "list",
    "matte",
    "menu",
    "menubutton",
    "menuitem",
    "outline",
    "popover",
    "progressbar",
    "radiobutton",
    "relevanceindicator",
    "ruler",
    "rulermarker",
    "scrollarea",
    "sheet",
    "slider",
    "splitter",
    "statictext",
    "switch",
    "tab",
    "table",
    "textfield",
    "timefield",
    "toolbar",
    "treeitem",
    "unknown",
    "webarea",
    "window",
];

/// Resolves a caller-supplied role to the canonical vocabulary,
/// case-insensitively. Common text-input aliases (`textarea`, `textbox`,
/// `searchfield`) normalize to `textfield`, matching how the platform
/// adapters map native text roles. Returns `None` for roles no adapter
/// can ever emit.
pub fn canonical_role(role: &str) -> Option<&'static str> {
    let normalized = role.trim().to_ascii_lowercase();
    let aliased = match normalized.as_str() {
        "textarea" | "textbox" | "searchfield" => "textfield",
        other => other,
    };
    CANONICAL_ROLES.iter().copied().find(|c| *c == aliased)
}

/// Returns true when `role` is in [`INTERACTIVE_ROLES`].
pub fn is_interactive_role(role: &str) -> bool {
    INTERACTIVE_ROLES.contains(&role)
}

/// Returns true for roles whose checked/unchecked state can be queried and set.
pub fn is_toggleable_role(role: &str) -> bool {
    matches!(role, "checkbox" | "switch" | "radiobutton")
}

/// Returns true for roles that carry an expanded/collapsed surface state.
pub fn is_expandable_role(role: &str) -> bool {
    matches!(role, "combobox" | "menubutton" | "treeitem")
}

/// Returns true for roles whose `value` changes during normal interaction and
/// must not be treated as stable ref identity.
pub fn is_mutable_value_role(role: &str) -> bool {
    matches!(role, "combobox" | "incrementor" | "slider" | "textfield")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interactive_roles_are_sorted_and_unique() {
        let mut sorted = INTERACTIVE_ROLES.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.as_slice(), INTERACTIVE_ROLES);
    }

    #[test]
    fn canonical_roles_are_sorted_and_unique() {
        let mut sorted = CANONICAL_ROLES.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.as_slice(), CANONICAL_ROLES);
    }

    #[test]
    fn interactive_roles_are_a_subset_of_canonical() {
        for role in INTERACTIVE_ROLES {
            assert!(
                CANONICAL_ROLES.contains(role),
                "interactive role {role} missing from CANONICAL_ROLES"
            );
        }
    }

    #[test]
    fn canonical_role_resolves_text_input_aliases() {
        assert_eq!(canonical_role("textarea"), Some("textfield"));
        assert_eq!(canonical_role("textbox"), Some("textfield"));
        assert_eq!(canonical_role("searchfield"), Some("textfield"));
    }

    #[test]
    fn canonical_role_is_case_insensitive_and_trimmed() {
        assert_eq!(canonical_role("Button"), Some("button"));
        assert_eq!(canonical_role(" TEXTAREA "), Some("textfield"));
    }

    #[test]
    fn canonical_role_rejects_unknown_roles() {
        assert_eq!(canonical_role("buttn"), None);
        assert_eq!(canonical_role(""), None);
    }

    #[test]
    fn toggleable_roles_are_a_subset_of_interactive() {
        for role in ["checkbox", "switch", "radiobutton"] {
            assert!(is_toggleable_role(role));
            assert!(is_interactive_role(role));
        }
        assert!(!is_toggleable_role("button"));
        assert!(!is_toggleable_role("textfield"));
    }

    #[test]
    fn expandable_roles_are_a_subset_of_interactive() {
        for role in ["combobox", "menubutton", "treeitem"] {
            assert!(is_expandable_role(role));
            assert!(is_interactive_role(role));
        }
        assert!(!is_expandable_role("button"));
        assert!(!is_expandable_role("checkbox"));
        assert!(!is_expandable_role("disclosure"));
    }

    #[test]
    fn every_expandable_role_is_interactive() {
        for role in ["combobox", "menubutton", "treeitem"] {
            assert!(
                is_expandable_role(role),
                "{role} expected expandable for subset check"
            );
            assert!(
                INTERACTIVE_ROLES.contains(&role),
                "expandable role {role} missing from INTERACTIVE_ROLES"
            );
        }
    }

    #[test]
    fn every_toggleable_role_is_interactive() {
        for role in ["checkbox", "switch", "radiobutton"] {
            assert!(is_toggleable_role(role));
            assert!(
                INTERACTIVE_ROLES.contains(&role),
                "toggleable role {role} missing from INTERACTIVE_ROLES"
            );
        }
    }

    #[test]
    fn read_only_roles_are_never_interactive() {
        for role in ["statictext", "image", "group", "list", "table"] {
            assert!(!is_interactive_role(role));
        }
    }

    #[test]
    fn mutable_value_roles_are_interactive() {
        for role in ["combobox", "incrementor", "slider", "textfield"] {
            assert!(is_mutable_value_role(role));
            assert!(is_interactive_role(role));
        }
        assert!(!is_mutable_value_role("cell"));
        assert!(!is_mutable_value_role("button"));
    }
}
