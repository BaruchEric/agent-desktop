use crate::{
    adapter::PlatformAdapter, commands::external_run, error::AppError,
    system::external::ExternalKind,
};
use serde_json::Value;

pub struct OpenTargetArgs {
    pub target: String,
}

pub fn execute(args: OpenTargetArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    external_run::run(ExternalKind::OpenUrl, args.target, None, adapter)
}
