use agent_desktop_core::error::AdapterError;

/// Error for a chain deadline expiring mid-increment. Unlike a plain step
/// "skip", expiry can leave the control at a half-applied value, so the
/// error must be TIMEOUT (not ACTION_FAILED) and must carry the observed
/// state — the caller cannot read post-state on the error path.
pub(crate) fn increment_deadline_error(start: f64, current: f64, target: f64) -> AdapterError {
    AdapterError::timeout("Chain deadline expired while stepping the value toward the target")
        .with_suggestion(
            "Re-read the element value before retrying; increase the timeout or AGENT_DESKTOP_CHAIN_TIMEOUT_MS for slow controls.",
        )
        .with_details(serde_json::json!({
            "value_before": start,
            "value_at_timeout": current,
            "target": target,
            "mutated": (current - start).abs() >= f64::EPSILON,
        }))
}

pub(crate) fn bool_write_had_effect(attr: &str, expected: bool, observed: Option<bool>) -> bool {
    !matches!(
        attr,
        "AXExpanded" | "AXDisclosing" | "AXSelected" | "AXFocused"
    ) || observed == Some(expected)
}

pub(crate) fn dynamic_write_had_effect(
    attr: &str,
    role: Option<&str>,
    expected: &str,
    observed: Option<&str>,
) -> bool {
    if attr != "AXValue" || role == Some("AXSecureTextField") {
        return true;
    }
    observed == Some(expected) || numbers_match(expected, observed)
}

/// Numeric controls report their value back in their own format (a slider
/// set to `50` reads back as `50.00`), so compare numerically when both
/// sides parse as numbers.
fn numbers_match(expected: &str, observed: Option<&str>) -> bool {
    match (
        expected.parse::<f64>(),
        observed.and_then(|o| o.parse::<f64>().ok()),
    ) {
        (Ok(a), Some(b)) => (a - b).abs() < f64::EPSILON,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{bool_write_had_effect, dynamic_write_had_effect, increment_deadline_error};

    #[test]
    fn increment_deadline_error_is_timeout_and_reports_partial_mutation() {
        let err = increment_deadline_error(0.0, 37.0, 80.0);

        assert_eq!(err.code, agent_desktop_core::error::ErrorCode::Timeout);
        let details = err.details.expect("details must carry the observed state");
        assert_eq!(details["value_before"], 0.0);
        assert_eq!(details["value_at_timeout"], 37.0);
        assert_eq!(details["target"], 80.0);
        assert_eq!(details["mutated"], true);
        assert!(err.suggestion.is_some());
    }

    #[test]
    fn increment_deadline_error_reports_unmutated_state() {
        let err = increment_deadline_error(5.0, 5.0, 9.0);

        assert_eq!(err.details.unwrap()["mutated"], false);
    }

    #[test]
    fn ax_value_write_requires_readback_match() {
        assert!(!dynamic_write_had_effect(
            "AXValue",
            Some("AXTextField"),
            "",
            Some("unchanged")
        ));
        assert!(dynamic_write_had_effect(
            "AXValue",
            Some("AXTextField"),
            "",
            Some("")
        ));
    }

    #[test]
    fn non_value_and_secure_writes_trust_ax_success() {
        assert!(dynamic_write_had_effect(
            "AXSelected",
            Some("AXCheckBox"),
            "true",
            None
        ));
        assert!(dynamic_write_had_effect(
            "AXValue",
            Some("AXSecureTextField"),
            "secret",
            None
        ));
    }

    #[test]
    fn bool_state_writes_require_readback_match_for_stateful_attrs() {
        assert!(bool_write_had_effect("AXExpanded", true, Some(true)));
        assert!(!bool_write_had_effect("AXExpanded", true, Some(false)));
        assert!(!bool_write_had_effect("AXExpanded", false, None));
        assert!(bool_write_had_effect("AXFoo", true, None));
    }

    #[test]
    fn numeric_value_write_matches_reformatted_readback() {
        assert!(dynamic_write_had_effect(
            "AXValue",
            Some("AXSlider"),
            "50",
            Some("50.00")
        ));
        assert!(dynamic_write_had_effect(
            "AXValue",
            Some("AXIncrementor"),
            "3",
            Some("3")
        ));
        assert!(!dynamic_write_had_effect(
            "AXValue",
            Some("AXSlider"),
            "50",
            Some("12.00")
        ));
    }
}
