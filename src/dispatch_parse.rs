use agent_desktop_core::{
    action::{Direction, MouseButton},
    commands::{get, is_check},
    error::AppError,
};

pub(crate) fn parse_get_property(s: &str) -> Result<get::GetProperty, AppError> {
    match s {
        "text" => Ok(get::GetProperty::Text),
        "value" => Ok(get::GetProperty::Value),
        "title" => Ok(get::GetProperty::Title),
        "bounds" => Ok(get::GetProperty::Bounds),
        "role" => Ok(get::GetProperty::Role),
        "states" => Ok(get::GetProperty::States),
        other => Err(AppError::invalid_input(format!(
            "Unknown property '{other}'. Valid: text, value, title, bounds, role, states"
        ))),
    }
}

pub(crate) fn parse_is_property(s: &str) -> Result<is_check::IsProperty, AppError> {
    match s {
        "visible" => Ok(is_check::IsProperty::Visible),
        "enabled" => Ok(is_check::IsProperty::Enabled),
        "checked" => Ok(is_check::IsProperty::Checked),
        "focused" => Ok(is_check::IsProperty::Focused),
        "expanded" => Ok(is_check::IsProperty::Expanded),
        other => Err(AppError::invalid_input(format!(
            "Unknown property '{other}'. Valid: visible, enabled, checked, focused, expanded"
        ))),
    }
}

pub(crate) fn parse_direction(s: &str) -> Result<Direction, AppError> {
    match s {
        "up" => Ok(Direction::Up),
        "down" => Ok(Direction::Down),
        "left" => Ok(Direction::Left),
        "right" => Ok(Direction::Right),
        other => Err(AppError::invalid_input(format!(
            "Unknown direction '{other}'. Valid: up, down, left, right"
        ))),
    }
}

pub(crate) fn parse_mouse_button(s: &str) -> Result<MouseButton, AppError> {
    match s {
        "left" => Ok(MouseButton::Left),
        "right" => Ok(MouseButton::Right),
        "middle" => Ok(MouseButton::Middle),
        other => Err(AppError::invalid_input(format!(
            "Unknown button '{other}'. Valid: left, right, middle"
        ))),
    }
}

pub(crate) fn parse_xy(s: &str) -> Result<(f64, f64), AppError> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(AppError::invalid_input(format!(
            "Invalid coordinates '{s}'. Expected format: x,y (e.g., 500,300)"
        )));
    }
    let x: f64 = parts[0]
        .trim()
        .parse()
        .map_err(|_| AppError::invalid_input(format!("Invalid x coordinate: '{}'", parts[0])))?;
    let y: f64 = parts[1]
        .trim()
        .parse()
        .map_err(|_| AppError::invalid_input(format!("Invalid y coordinate: '{}'", parts[1])))?;
    Ok((x, y))
}

pub(crate) fn parse_xy_opt(s: Option<&str>) -> Result<Option<(f64, f64)>, AppError> {
    match s {
        Some(s) => parse_xy(s).map(Some),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_direction() {
        assert_eq!(
            parse_direction("sideways").unwrap_err().code(),
            "INVALID_ARGS"
        );
    }

    #[test]
    fn rejects_unknown_get_property() {
        match parse_get_property("placeholder") {
            Ok(_) => panic!("expected invalid get property"),
            Err(err) => assert_eq!(err.code(), "INVALID_ARGS"),
        }
    }

    #[test]
    fn rejects_unknown_is_property() {
        match parse_is_property("selected") {
            Ok(_) => panic!("expected invalid is property"),
            Err(err) => assert_eq!(err.code(), "INVALID_ARGS"),
        }
    }

    #[test]
    fn rejects_unknown_mouse_button() {
        assert_eq!(
            parse_mouse_button("primary").unwrap_err().code(),
            "INVALID_ARGS"
        );
    }

    #[test]
    fn parses_xy_with_whitespace() {
        assert_eq!(parse_xy(" 10.5, 20 ").unwrap(), (10.5, 20.0));
    }

    #[test]
    fn rejects_bad_xy_shape_and_numbers() {
        assert_eq!(parse_xy("10").unwrap_err().code(), "INVALID_ARGS");
        assert_eq!(parse_xy("x,20").unwrap_err().code(), "INVALID_ARGS");
        assert_eq!(parse_xy("10,y").unwrap_err().code(), "INVALID_ARGS");
    }
}
