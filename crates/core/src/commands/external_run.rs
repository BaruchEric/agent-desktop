use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    adapter::PlatformAdapter,
    error::{AdapterError, AppError, ErrorCode},
    refs::home_dir,
    system::external::{ExternalKind, ExternalRequest},
};
use serde_json::{Value, json};

pub const EXEC_ENV: &str = "AGENT_DESKTOP_ENABLE_EXEC";

pub(crate) fn exec_enabled() -> bool {
    std::env::var(EXEC_ENV)
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false)
}

pub(crate) fn run(
    kind: ExternalKind,
    payload: String,
    timeout_ms: Option<u64>,
    adapter: &dyn PlatformAdapter,
) -> Result<Value, AppError> {
    let enabled = exec_enabled();
    if enabled {
        audit(kind, &payload);
    }
    run_with_gate(enabled, kind, payload, timeout_ms, adapter)
}

pub(crate) fn run_with_gate(
    enabled: bool,
    kind: ExternalKind,
    payload: String,
    timeout_ms: Option<u64>,
    adapter: &dyn PlatformAdapter,
) -> Result<Value, AppError> {
    if !enabled {
        return Err(AppError::Adapter(
            AdapterError::new(ErrorCode::PolicyDenied, "External execution is disabled")
                .with_suggestion(
                    "Set AGENT_DESKTOP_ENABLE_EXEC=1 to enable run-shell/run-applescript/run-jxa/open-url/open-path",
                ),
        ));
    }
    let result = adapter.system().run_external(ExternalRequest {
        kind,
        payload,
        timeout_ms,
    })?;
    Ok(json!({
        "exit_code": result.exit_code,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "duration_ms": result.duration_ms,
    }))
}

fn audit(kind: ExternalKind, payload: &str) {
    let Some(home) = home_dir() else { return };
    let dir = home.join(".agent-desktop");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let preview: String = payload.chars().take(120).collect();
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("exec_audit.log"))
    {
        let _ = writeln!(f, "{}\t{}\t{}", ts, kind.as_str(), preview);
    }
}

#[cfg(test)]
#[path = "external_run_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "external_kind_tests.rs"]
mod kind_tests;
