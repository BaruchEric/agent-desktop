use crate::{
    adapter::{PlatformAdapter, SnapshotSurface, TreeOptions, WindowFilter},
    context::CommandContext,
    error::{AdapterError, AppError, ErrorCode},
    node::{AccessibilityNode, WindowInfo},
    ref_alloc::{self, RefAllocConfig},
    refs::RefMap,
    refs_store::RefStore,
};

pub struct SnapshotResult {
    pub tree: AccessibilityNode,
    pub refmap: RefMap,
    pub window: WindowInfo,
    pub snapshot_id: Option<String>,
}

pub fn build(
    adapter: &dyn PlatformAdapter,
    opts: &TreeOptions,
    app_name: Option<&str>,
    window_id: Option<&str>,
) -> Result<SnapshotResult, AppError> {
    let filter = WindowFilter {
        focused_only: app_name.is_none() && window_id.is_none(),
        app: app_name.map(str::to_string),
    };

    let windows = adapter.list_windows(&filter)?;

    if let Some(wid) = window_id {
        let window = windows.into_iter().find(|w| w.id == wid).ok_or_else(|| {
            AppError::Adapter(
                AdapterError::new(
                    ErrorCode::WindowNotFound,
                    format!("No window with id {wid}"),
                )
                .with_suggestion("Run 'list-windows' to see available window IDs."),
            )
        })?;
        return snapshot_from_window(adapter, &window, opts);
    }

    if let Some(app) = app_name {
        let window = windows
            .iter()
            .find(|w| w.app.eq_ignore_ascii_case(app) && w.is_focused)
            .cloned()
            .or_else(|| {
                windows
                    .into_iter()
                    .find(|w| w.app.eq_ignore_ascii_case(app))
            });
        return match window {
            Some(w) => snapshot_from_window(adapter, &w, opts),
            None => {
                let pid = adapter
                    .list_apps()?
                    .into_iter()
                    .find(|a| a.name.eq_ignore_ascii_case(app))
                    .map(|a| a.pid)
                    .ok_or_else(|| {
                        AppError::Adapter(
                            AdapterError::new(
                                ErrorCode::AppNotFound,
                                format!("No window or process found for app '{app}'"),
                            )
                            .with_suggestion(
                                "Verify the app is running. Use 'list-apps' to see running applications.",
                            ),
                        )
                    })?;
                snapshot_from_app(adapter, pid, app, opts)
            }
        };
    }

    let window = windows.into_iter().find(|w| w.is_focused).ok_or_else(|| {
        AppError::Adapter(
            AdapterError::new(ErrorCode::WindowNotFound, "No focused window found")
                .with_suggestion(
                    "Use --app to specify an application, or click a window to focus it.",
                ),
        )
    })?;
    snapshot_from_window(adapter, &window, opts)
}

fn snapshot_from_window(
    adapter: &dyn PlatformAdapter,
    window: &WindowInfo,
    opts: &TreeOptions,
) -> Result<SnapshotResult, AppError> {
    let raw_tree = adapter.get_tree(window, &opts.with_ref_identity_bounds())?;
    Ok(finalize_snapshot(raw_tree, window.clone(), opts))
}

fn snapshot_from_app(
    adapter: &dyn PlatformAdapter,
    pid: i32,
    app: &str,
    opts: &TreeOptions,
) -> Result<SnapshotResult, AppError> {
    let raw_tree = adapter.get_app_tree(pid, &opts.with_ref_identity_bounds())?;
    let synthetic_window = WindowInfo {
        id: format!("app-{pid}"),
        title: app.to_string(),
        app: app.to_string(),
        pid,
        bounds: None,
        is_focused: false,
    };
    Ok(finalize_snapshot(raw_tree, synthetic_window, opts))
}

fn finalize_snapshot(
    raw_tree: AccessibilityNode,
    window: WindowInfo,
    opts: &TreeOptions,
) -> SnapshotResult {
    let mut refmap = RefMap::new();
    let config = RefAllocConfig {
        include_bounds: opts.include_bounds,
        interactive_only: opts.interactive_only,
        compact: opts.compact,
        pid: window.pid,
        source_app: Some(window.app.as_str()),
        source_window_id: Some(window.id.as_str()),
        source_window_title: Some(window.title.as_str()),
        source_surface: opts.surface,
        root_ref_id: None,
        path_prefix: &[],
    };
    let mut tree = ref_alloc::allocate_refs(raw_tree, &mut refmap, &config);
    crate::hints::add_structural_hints(&mut tree);
    SnapshotResult {
        tree,
        refmap,
        window,
        snapshot_id: None,
    }
}

#[cfg(test)]
pub fn run(
    adapter: &dyn PlatformAdapter,
    opts: &TreeOptions,
    app_name: Option<&str>,
    window_id: Option<&str>,
) -> Result<SnapshotResult, AppError> {
    run_with_context(
        adapter,
        opts,
        app_name,
        window_id,
        &CommandContext::default(),
    )
}

pub fn run_with_context(
    adapter: &dyn PlatformAdapter,
    opts: &TreeOptions,
    app_name: Option<&str>,
    window_id: Option<&str>,
    context: &CommandContext,
) -> Result<SnapshotResult, AppError> {
    let mut result = build(adapter, opts, app_name, window_id)?;
    let store = RefStore::for_session(context.session_id())?;
    let snapshot_id = store.save_new_snapshot(&result.refmap)?;
    result.snapshot_id = Some(snapshot_id);
    context.trace_lazy(
        "snapshot.saved",
        || serde_json::json!({ "snapshot_id": result.snapshot_id, "ref_count": result.refmap.len() }),
    )?;
    Ok(result)
}

pub fn append_surface_refs(
    adapter: &dyn PlatformAdapter,
    pid: i32,
    source_app: Option<&str>,
    surface: SnapshotSurface,
) -> Result<Option<AccessibilityNode>, AppError> {
    append_surface_refs_with_context(
        adapter,
        pid,
        source_app,
        surface,
        &CommandContext::default(),
    )
}

pub fn append_surface_refs_with_context(
    adapter: &dyn PlatformAdapter,
    pid: i32,
    source_app: Option<&str>,
    surface: SnapshotSurface,
    context: &CommandContext,
) -> Result<Option<AccessibilityNode>, AppError> {
    let opts = TreeOptions {
        surface,
        interactive_only: true,
        ..Default::default()
    };

    let (raw_tree, window_id, window_title) = {
        let filter = WindowFilter {
            focused_only: false,
            app: None,
        };
        let windows = adapter.list_windows(&filter)?;
        if let Some(window) = windows.into_iter().find(|w| w.pid == pid) {
            let raw = adapter.get_tree(&window, &opts.with_ref_identity_bounds())?;
            let wid = window.id.clone();
            let wtitle = window.title.clone();
            (raw, wid, wtitle)
        } else {
            let raw = adapter.get_app_tree(pid, &opts.with_ref_identity_bounds())?;
            let synthetic_id = format!("app-{pid}");
            let synthetic_title = source_app.unwrap_or("").to_string();
            (raw, synthetic_id, synthetic_title)
        }
    };

    let store = RefStore::for_session(context.session_id())?;
    let mut refmap = store.load_latest()?;
    let config = RefAllocConfig {
        include_bounds: false,
        interactive_only: true,
        compact: false,
        pid,
        source_app,
        source_window_id: Some(window_id.as_str()),
        source_window_title: Some(window_title.as_str()),
        source_surface: surface,
        root_ref_id: None,
        path_prefix: &[],
    };
    let tree = ref_alloc::allocate_refs(raw_tree, &mut refmap, &config);
    if let Some(id) = store.latest_snapshot_id() {
        store.save_existing_snapshot(&id, &refmap)?;
    } else {
        store.save_new_snapshot(&refmap)?;
    }
    Ok(Some(tree))
}

#[cfg(test)]
#[path = "snapshot_tests.rs"]
mod tests;
