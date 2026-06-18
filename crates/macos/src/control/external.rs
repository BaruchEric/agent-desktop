use agent_desktop_core::{
    error::AdapterError,
    system::external::{
        DEFAULT_EXTERNAL_TIMEOUT_MS, ExternalKind, ExternalRequest, ExternalResult,
    },
};
use std::process::Command;
use std::time::{Duration, Instant};

fn build(req: &ExternalRequest) -> Command {
    match req.kind {
        ExternalKind::Shell => {
            let mut c = Command::new("sh");
            c.arg("-c").arg(&req.payload);
            c
        }
        ExternalKind::AppleScript => {
            let mut c = Command::new("osascript");
            c.arg("-e").arg(&req.payload);
            c
        }
        ExternalKind::Jxa => {
            let mut c = Command::new("osascript");
            c.arg("-l").arg("JavaScript").arg("-e").arg(&req.payload);
            c
        }
        ExternalKind::OpenUrl | ExternalKind::OpenPath => {
            let mut c = Command::new("open");
            c.arg(&req.payload);
            c
        }
    }
}

pub fn handle(req: ExternalRequest) -> Result<ExternalResult, AdapterError> {
    let timeout_ms = req.timeout_ms.unwrap_or(DEFAULT_EXTERNAL_TIMEOUT_MS);
    let mut command = build(&req);
    let start = Instant::now();
    let output = crate::system::process::run_with_timeout(
        &mut command,
        req.kind.as_str(),
        Duration::from_millis(timeout_ms),
    )?;
    Ok(ExternalResult {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        duration_ms: start.elapsed().as_millis() as u64,
    })
}
