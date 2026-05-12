use super::{child_attributes, redact_secure_value};

#[test]
fn test_browser_children_use_columns() {
    assert_eq!(
        child_attributes(Some("AXBrowser")),
        ["AXColumns", "AXContents"]
    );
}

#[test]
fn test_default_children_follow_fallback_order() {
    assert_eq!(
        child_attributes(Some("AXGroup")),
        ["AXChildren", "AXContents", "AXChildrenInNavigationOrder"]
    );
}

#[test]
fn test_secure_text_value_is_redacted() {
    assert_eq!(
        redact_secure_value(Some("AXSecureTextField"), Some("secret".into())),
        None
    );
    assert_eq!(
        redact_secure_value(Some("AXTextField"), Some("visible".into())),
        Some("visible".into())
    );
}
