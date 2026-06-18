use crate::{adapter::PlatformAdapter, error::AppError, system::appearance::AppearanceRequest};
use serde_json::{Value, json};

pub struct AppearanceArgs {
    pub get: bool,
    pub dark: bool,
    pub light: bool,
    pub toggle: bool,
}

pub fn execute(args: AppearanceArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    let selected = [args.get, args.dark, args.light, args.toggle]
        .iter()
        .filter(|x| **x)
        .count();
    if selected != 1 {
        return Err(AppError::invalid_input(
            "appearance needs exactly one of --get/--dark/--light/--toggle",
        ));
    }
    let req = if args.get {
        AppearanceRequest::Get
    } else if args.dark {
        AppearanceRequest::SetDark(true)
    } else if args.light {
        AppearanceRequest::SetDark(false)
    } else {
        AppearanceRequest::Toggle
    };
    let state = adapter.system().appearance(req)?;
    Ok(json!({ "dark": state.dark }))
}

#[cfg(test)]
#[path = "appearance_tests.rs"]
mod tests;
