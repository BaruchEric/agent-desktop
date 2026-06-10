use crate::{adapter::PlatformAdapter, error::AppError};
use serde_json::{Value, json};

pub struct CloseAppArgs {
    pub app: String,
    pub force: bool,
}

pub fn execute(args: CloseAppArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    if adapter.is_protected_process(&args.app) {
        return Err(AppError::invalid_input(format!(
            "'{}' is a protected system process and cannot be closed",
            args.app
        )));
    }
    adapter.close_app(&args.app, args.force)?;
    Ok(json!({ "app": args.app, "closed": true }))
}

#[cfg(test)]
#[path = "close_app_tests.rs"]
mod tests;
