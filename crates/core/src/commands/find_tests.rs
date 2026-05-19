use super::*;

fn node(name: Option<&str>, value: Option<&str>, description: Option<&str>) -> AccessibilityNode {
    AccessibilityNode {
        ref_id: Some("@e1".into()),
        role: "textfield".into(),
        name: name.map(String::from),
        value: value.map(String::from),
        description: description.map(String::from),
        hint: None,
        states: vec![],
        available_actions: vec![],
        bounds: None,
        children_count: None,
        children: vec![],
    }
}

#[test]
fn display_name_prefers_value_before_description() {
    let root = node(None, Some("current value"), Some("help text"));
    let query = FindQuery {
        role: None,
        name: None,
        value: None,
        text: None,
    };
    let mut matches = Vec::new();

    search_tree(&root, &query, &mut Vec::new(), &mut matches, None);

    assert_eq!(matches[0]["name"], "current value");
}

#[test]
fn search_tree_matches_text_across_fields() {
    let root = node(None, Some("Primary"), Some("Secondary"));
    let query = FindQuery {
        role: None,
        name: None,
        value: None,
        text: Some(search_text::normalize("secondary")),
    };
    let mut matches = Vec::new();

    search_tree(&root, &query, &mut Vec::new(), &mut matches, None);

    assert_eq!(matches.len(), 1);
}

#[test]
fn default_limit_caps_materialized_matches() {
    let root = AccessibilityNode {
        ref_id: None,
        role: "window".into(),
        name: None,
        value: None,
        description: None,
        hint: None,
        states: vec![],
        available_actions: vec![],
        bounds: None,
        children_count: None,
        children: (0..60)
            .map(|i| node(Some(&format!("Button {i}")), None, None))
            .collect(),
    };
    let query = FindQuery {
        role: None,
        name: None,
        value: None,
        text: Some(search_text::normalize("button")),
    };
    let mut matches = Vec::new();

    search_tree(
        &root,
        &query,
        &mut Vec::new(),
        &mut matches,
        Some(DEFAULT_LIMIT),
    );

    assert_eq!(matches.len(), DEFAULT_LIMIT);
}

#[test]
fn limit_conflicts_with_single_result_modes_for_batch_too() {
    let err = validate_find_mode(&FindArgs {
        app: None,
        role: None,
        name: None,
        value: None,
        text: None,
        count: false,
        first: true,
        last: false,
        nth: None,
        limit: Some(10),
    })
    .unwrap_err();

    assert_eq!(err.code(), "INVALID_ARGS");
}

#[test]
fn count_matches_does_not_build_result_json() {
    let root = AccessibilityNode {
        ref_id: None,
        role: "window".into(),
        name: None,
        value: None,
        description: None,
        hint: None,
        states: vec![],
        available_actions: vec![],
        bounds: None,
        children_count: None,
        children: vec![
            node(Some("Save"), None, None),
            node(Some("Cancel"), None, None),
        ],
    };
    let query = FindQuery {
        role: None,
        name: None,
        value: None,
        text: Some(search_text::normalize("a")),
    };

    assert_eq!(count_matches(&root, &query), 2);
}
