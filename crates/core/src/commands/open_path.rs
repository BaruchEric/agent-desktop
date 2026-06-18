use crate::{
    adapter::PlatformAdapter,
    commands::{external_run, open_url::OpenTargetArgs},
    error::AppError,
    system::external::ExternalKind,
};
use serde_json::Value;

pub fn execute(args: OpenTargetArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    external_run::run(ExternalKind::OpenPath, args.target, None, adapter)
}
