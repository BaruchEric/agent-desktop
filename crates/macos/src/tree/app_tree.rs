use agent_desktop_core::{
    adapter::{SnapshotSurface, TreeOptions},
    error::AdapterError,
    node::AccessibilityNode,
};
use rustc_hash::FxHashSet;

use super::AXElement;
use super::build_context::TreeBuildContext;
use super::builder::build_subtree;
use super::element::element_for_pid;
use super::surfaces::{
    alert_for_pid, dock_root_for_pid, extras_menubar_for_pid, focused_surface_for_pid,
    menu_element_for_pid, menubar_for_pid, popover_for_pid, sheet_for_pid,
};

/// Builds an accessibility tree rooted at the app element for `pid`.
///
/// The surface field of `opts` selects which AX root element to use.
pub fn build_app_tree(pid: i32, opts: &TreeOptions) -> Result<AccessibilityNode, AdapterError> {
    let root = surface_root_for_pid(pid, opts.surface)?;
    build_tree_from_root(pid, &root, opts)
}

pub(crate) fn surface_root_for_pid(
    pid: i32,
    surface: SnapshotSurface,
) -> Result<AXElement, AdapterError> {
    match surface {
        SnapshotSurface::Window => Ok(element_for_pid(pid)),
        SnapshotSurface::Focused => focused_surface_for_pid(pid)
            .ok_or_else(|| AdapterError::internal("No focused surface found")),
        SnapshotSurface::Menu => menu_element_for_pid(pid)
            .ok_or_else(|| AdapterError::element_not_found("No open context menu")),
        SnapshotSurface::Menubar => {
            menubar_for_pid(pid).ok_or_else(|| AdapterError::element_not_found("No menu bar found"))
        }
        SnapshotSurface::ExtrasMenubar => extras_menubar_for_pid(pid)
            .ok_or_else(|| AdapterError::element_not_found("No extras menu bar found")),
        SnapshotSurface::Dock => dock_root_for_pid(pid)
            .ok_or_else(|| AdapterError::element_not_found("No dock list element found")),
        SnapshotSurface::Sheet => {
            sheet_for_pid(pid).ok_or_else(|| AdapterError::element_not_found("No open sheet"))
        }
        SnapshotSurface::Popover => popover_for_pid(pid)
            .ok_or_else(|| AdapterError::element_not_found("No visible popover")),
        SnapshotSurface::Alert => alert_for_pid(pid)
            .ok_or_else(|| AdapterError::element_not_found("No open alert or dialog")),
    }
}

pub(crate) fn build_tree_from_root(
    pid: i32,
    root: &AXElement,
    opts: &TreeOptions,
) -> Result<AccessibilityNode, AdapterError> {
    let mut ancestors = FxHashSet::default();
    let context = TreeBuildContext::for_pid(pid, opts.include_bounds);
    build_subtree(
        root,
        0,
        0,
        opts.max_depth,
        &mut ancestors,
        opts.skeleton,
        &context,
    )
    .ok_or_else(|| AdapterError::internal("Empty AX tree for surface"))
}
