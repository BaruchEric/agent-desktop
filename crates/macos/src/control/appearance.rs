use agent_desktop_core::{
    error::{AdapterError, ErrorCode},
    system::appearance::{AppearanceRequest, AppearanceState},
};
use std::process::Command;
use std::time::Duration;

const OSA_TIMEOUT: Duration = Duration::from_secs(10);

fn run_osa(script: &str) -> Result<String, AdapterError> {
    let mut cmd = Command::new("osascript");
    cmd.arg("-e").arg(script);
    let out = crate::system::process::run_with_timeout(&mut cmd, "osascript", OSA_TIMEOUT)?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        let code = if stderr.contains("Not authorized") || stderr.contains("-1743") {
            ErrorCode::PermDenied
        } else {
            ErrorCode::ActionFailed
        };
        return Err(AdapterError::new(code, "osascript failed")
            .with_platform_detail(stderr)
            .with_suggestion(
                "Grant Automation permission for your terminal in System Settings > Privacy & Security > Automation",
            ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn read_dark() -> Result<bool, AdapterError> {
    let v = run_osa(
        "tell application \"System Events\" to tell appearance preferences to get dark mode",
    )?;
    Ok(v == "true")
}

fn set_and_read(value_expr: &str) -> Result<bool, AdapterError> {
    let script = format!(
        "tell application \"System Events\"\ntell appearance preferences\nset dark mode to {value_expr}\nreturn dark mode\nend tell\nend tell"
    );
    Ok(run_osa(&script)? == "true")
}

pub fn handle(req: AppearanceRequest) -> Result<AppearanceState, AdapterError> {
    let dark = match req {
        AppearanceRequest::Get => read_dark()?,
        AppearanceRequest::SetDark(v) => set_and_read(if v { "true" } else { "false" })?,
        AppearanceRequest::Toggle => set_and_read("not dark mode")?,
    };
    Ok(AppearanceState { dark })
}
