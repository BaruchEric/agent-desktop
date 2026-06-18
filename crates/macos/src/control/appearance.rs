use agent_desktop_core::{
    error::{AdapterError, ErrorCode},
    system::appearance::{AppearanceRequest, AppearanceState},
};
use std::process::Command;

fn run_osa(script: &str) -> Result<String, AdapterError> {
    let out = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| {
            AdapterError::new(ErrorCode::ActionFailed, "osascript spawn failed")
                .with_platform_detail(e.to_string())
        })?;
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

fn set_dark(on: bool) -> Result<(), AdapterError> {
    let script = format!(
        "tell application \"System Events\" to tell appearance preferences to set dark mode to {on}"
    );
    run_osa(&script).map(|_| ())
}

pub fn handle(req: AppearanceRequest) -> Result<AppearanceState, AdapterError> {
    match req {
        AppearanceRequest::Get => {}
        AppearanceRequest::SetDark(v) => set_dark(v)?,
        AppearanceRequest::Toggle => {
            let cur = read_dark()?;
            set_dark(!cur)?;
        }
    }
    Ok(AppearanceState { dark: read_dark()? })
}
