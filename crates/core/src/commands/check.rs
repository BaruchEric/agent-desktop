use crate::{
    action::{Action, ActionRequest},
    adapter::PlatformAdapter,
    commands::helpers::{execute_ref_action, RefArgs},
    error::AppError,
};
use serde_json::Value;

pub fn execute(args: RefArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    execute_ref_action(args, adapter, ActionRequest::headless(Action::Check))
}
