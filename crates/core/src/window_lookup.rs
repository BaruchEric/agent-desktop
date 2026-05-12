use crate::{
    adapter::{PlatformAdapter, WindowFilter},
    error::AppError,
    node::WindowInfo,
};

pub(crate) fn find_window_for_pid(
    pid: i32,
    adapter: &dyn PlatformAdapter,
) -> Result<WindowInfo, AppError> {
    let filter = WindowFilter {
        focused_only: false,
        app: None,
    };
    let windows = adapter.list_windows(&filter)?;
    let mut candidates: Vec<_> = windows.into_iter().filter(|w| w.pid == pid).collect();
    if candidates.is_empty() {
        return Err(AppError::invalid_input(
            "No window found for this application",
        ));
    }
    let selected = candidates
        .iter()
        .position(|w| w.is_focused)
        .unwrap_or_default();
    Ok(candidates.swap_remove(selected))
}
