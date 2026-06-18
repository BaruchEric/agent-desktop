use crate::{
    adapter::PlatformAdapter,
    commands::{external_run, run_shell::RunScriptArgs},
    error::AppError,
    system::external::ExternalKind,
};
use serde_json::Value;

pub fn execute(args: RunScriptArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    external_run::run(ExternalKind::Jxa, args.script, args.timeout, adapter)
}
