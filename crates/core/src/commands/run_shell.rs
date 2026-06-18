use crate::{
    adapter::PlatformAdapter, commands::external_run, error::AppError,
    system::external::ExternalKind,
};
use serde_json::Value;

pub struct RunScriptArgs {
    pub script: String,
    pub timeout: Option<u64>,
}

pub fn execute(args: RunScriptArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    external_run::run(ExternalKind::Shell, args.script, args.timeout, adapter)
}
