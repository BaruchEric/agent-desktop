use agent_desktop_core::{
    error::{AdapterError, ErrorCode},
    system::external::{ExternalKind, ExternalRequest, ExternalResult},
};
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const DEFAULT_EXTERNAL_TIMEOUT_MS: u64 = 30_000;

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
    let start = Instant::now();
    let mut child = build(&req)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            AdapterError::new(ErrorCode::ActionFailed, "spawn failed")
                .with_platform_detail(e.to_string())
        })?;

    let timeout_ms = req.timeout_ms.unwrap_or(DEFAULT_EXTERNAL_TIMEOUT_MS);
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);

    loop {
        if let Some(status) = child.try_wait().map_err(|e| {
            AdapterError::new(ErrorCode::ActionFailed, "wait failed")
                .with_platform_detail(e.to_string())
        })? {
            let mut stdout = String::new();
            let mut stderr = String::new();
            if let Some(mut o) = child.stdout.take() {
                let _ = o.read_to_string(&mut stdout);
            }
            if let Some(mut e) = child.stderr.take() {
                let _ = e.read_to_string(&mut stderr);
            }
            return Ok(ExternalResult {
                exit_code: status.code().unwrap_or(-1),
                stdout,
                stderr,
                duration_ms: start.elapsed().as_millis() as u64,
            });
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(AdapterError::timeout("external command timed out"));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
