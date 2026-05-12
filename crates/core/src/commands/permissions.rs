use crate::{adapter::PlatformAdapter, error::AppError, PermissionReport};
use serde_json::{json, Value};

pub struct PermissionsArgs {
    pub request: bool,
}

pub fn execute_with_report(
    args: PermissionsArgs,
    adapter: &dyn PlatformAdapter,
    report: &PermissionReport,
) -> Result<Value, AppError> {
    let report = if args.request {
        adapter.request_permissions()
    } else {
        report.clone()
    };
    Ok(render(report))
}

fn render(report: PermissionReport) -> Value {
    json!({
        "accessibility": report.accessibility,
        "screen_recording": report.screen_recording,
        "automation": report.automation
    })
}
