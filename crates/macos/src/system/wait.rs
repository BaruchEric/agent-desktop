use agent_desktop_core::error::AdapterError;
use std::time::{Duration, Instant};

#[cfg(target_os = "macos")]
mod imp {
    use super::*;
    use crate::tree::surfaces::is_menu_open;

    const DEFAULT_MENU_TIMEOUT_MS: u64 = 750;
    const MAX_MENU_TIMEOUT_MS: u64 = 10_000;

    pub fn wait_for_menu(pid: i32, open: bool, timeout_ms: u64) -> Result<(), AdapterError> {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            if is_menu_open(pid) == open {
                return Ok(());
            }
            if Instant::now() >= deadline {
                let msg = if open {
                    format!("No context menu opened within {timeout_ms}ms")
                } else {
                    format!("Context menu did not close within {timeout_ms}ms")
                };
                return Err(AdapterError::timeout(msg));
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    pub fn menu_timeout_ms() -> u64 {
        std::env::var("AGENT_DESKTOP_MENU_TIMEOUT_MS")
            .ok()
            .and_then(|raw| raw.parse::<u64>().ok())
            .filter(|ms| *ms > 0)
            .map(|ms| ms.min(MAX_MENU_TIMEOUT_MS))
            .unwrap_or(DEFAULT_MENU_TIMEOUT_MS)
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    use super::*;

    pub fn wait_for_menu(_pid: i32, _open: bool, _timeout_ms: u64) -> Result<(), AdapterError> {
        Err(AdapterError::not_supported("wait_for_menu"))
    }

    pub fn menu_timeout_ms() -> u64 {
        750
    }
}

pub use imp::{menu_timeout_ms, wait_for_menu};
