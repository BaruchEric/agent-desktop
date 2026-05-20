use crate::{
    adapter::{PlatformAdapter, WindowFilter},
    error::{AdapterError, AppError, ErrorCode},
    node::WindowInfo,
};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

const FOCUS_SETTLE_TIMEOUT_MS: u64 = 750;
const FOCUS_POLL_INTERVAL_MS: u64 = 50;
const FOCUS_CONFIRMATIONS: u8 = 2;

pub struct FocusWindowArgs {
    pub window_id: Option<String>,
    pub app: Option<String>,
    pub title: Option<String>,
}

pub fn execute(args: FocusWindowArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    let filter = WindowFilter {
        focused_only: false,
        app: args.app.clone(),
    };
    let windows = adapter.list_windows(&filter)?;

    let window = if let Some(id) = &args.window_id {
        windows.into_iter().find(|w| &w.id == id)
    } else if let Some(title) = &args.title {
        windows
            .into_iter()
            .find(|w| w.title.contains(title.as_str()))
    } else if let Some(app) = &args.app {
        windows
            .into_iter()
            .find(|w| w.app.eq_ignore_ascii_case(app))
    } else {
        return Err(AppError::invalid_input(
            "Provide --window-id, --app, or --title to identify the window",
        ));
    };

    let window = window.ok_or_else(|| {
        AppError::Adapter(
            crate::error::AdapterError::new(
                crate::error::ErrorCode::WindowNotFound,
                "No matching window found",
            )
            .with_suggestion("Run 'list-windows' to see available windows and their IDs."),
        )
    })?;

    let window_id = window.id.clone();
    adapter.focus_window(&window)?;
    let focused = wait_for_focused_window(adapter, &window_id, args.app)?;
    Ok(json!({ "focused": focused }))
}

fn wait_for_focused_window(
    adapter: &dyn PlatformAdapter,
    window_id: &str,
    app: Option<String>,
) -> Result<WindowInfo, AppError> {
    wait_for_focused_window_with_poll_interval(
        adapter,
        window_id,
        app.as_deref(),
        Duration::from_millis(FOCUS_POLL_INTERVAL_MS),
    )
}

fn wait_for_focused_window_with_poll_interval(
    adapter: &dyn PlatformAdapter,
    window_id: &str,
    app: Option<&str>,
    poll_interval: Duration,
) -> Result<WindowInfo, AppError> {
    let deadline = Instant::now() + Duration::from_millis(FOCUS_SETTLE_TIMEOUT_MS);
    let mut confirmations = 0;
    loop {
        match observed_focused_window(adapter, app)? {
            Some(window) if window.id == window_id => {
                confirmations += 1;
                if confirmations >= FOCUS_CONFIRMATIONS {
                    return Ok(window);
                }
            }
            _ => {
                confirmations = 0;
            }
        }

        if Instant::now() >= deadline {
            return Err(AppError::Adapter(
                AdapterError::new(
                    ErrorCode::ActionFailed,
                    "Window focus did not settle on the requested window",
                )
                .with_suggestion("Run 'list-windows' to refresh window IDs, then retry."),
            ));
        }

        if !poll_interval.is_zero() {
            std::thread::sleep(poll_interval);
        }
    }
}

fn observed_focused_window(
    adapter: &dyn PlatformAdapter,
    app: Option<&str>,
) -> Result<Option<WindowInfo>, AppError> {
    match adapter.focused_window() {
        Ok(window) => Ok(window),
        Err(err) if err.code == ErrorCode::PlatformNotSupported => adapter
            .list_windows(&WindowFilter {
                focused_only: true,
                app: app.map(str::to_string),
            })
            .map(|windows| windows.into_iter().next())
            .map_err(AppError::Adapter),
        Err(err) => Err(AppError::Adapter(err)),
    }
}

#[cfg(test)]
#[path = "focus_window_tests.rs"]
mod tests;
