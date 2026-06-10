use agent_desktop_core::{
    adapter::NativeHandle,
    error::{AdapterError, ErrorCode},
    refs::RefEntry,
};
use std::time::{Duration, Instant};

#[cfg(target_os = "macos")]
use super::resolve_deadline::sleep_before_retry;
#[cfg(target_os = "macos")]
use super::resolve_identity::has_meaningful_identity;
#[cfg(target_os = "macos")]
use super::resolve_roots::{
    candidate_roots, path_candidate_roots, source_window_number, source_window_scope_required,
};
#[cfg(target_os = "macos")]
use super::resolve_search::{find_entry_by_path, find_entry_in_roots};

#[cfg(target_os = "macos")]
pub fn resolve_element_impl(entry: &RefEntry) -> Result<NativeHandle, AdapterError> {
    resolve_element_with_timeout(entry, Duration::from_secs(5))
}

#[cfg(target_os = "macos")]
pub fn resolve_element_with_timeout(
    entry: &RefEntry,
    timeout: Duration,
) -> Result<NativeHandle, AdapterError> {
    let (resolve_depth, attempts) = (50, 4);
    let deadline = Instant::now() + timeout;
    for attempt in 0..attempts {
        if can_use_path_fast_path(entry) {
            let path_roots = path_candidate_roots(entry, deadline)?;
            let scope_verified = path_roots.scope_verified;
            match find_entry_by_path(&path_roots.roots, entry, scope_verified, deadline) {
                Ok(handle) => {
                    return Ok(handle);
                }
                Err(err) if is_retryable_resolution_error(&err) => {}
                Err(err) => return Err(err),
            }
            if requires_scoped_path_resolution(entry) {
                if attempt + 1 < attempts {
                    sleep_before_retry(deadline);
                }
                continue;
            }
        }
        if !can_use_broad_search(entry) {
            if attempt + 1 < attempts {
                sleep_before_retry(deadline);
            }
            continue;
        }
        let roots = candidate_roots(entry, deadline)?;
        let scope_verified = roots.scope_verified;
        match find_entry_in_roots(&roots.roots, entry, resolve_depth, scope_verified, deadline) {
            Ok(handle) => {
                return Ok(handle);
            }
            Err(err) if is_retryable_resolution_error(&err) => {}
            Err(err) => return Err(err),
        }

        if attempt + 1 < attempts {
            sleep_before_retry(deadline);
        }
    }

    Err(AdapterError::new(
        ErrorCode::StaleRef,
        format!(
            "Element not found: role={}, name={:?}, description={:?}",
            entry.role,
            entry.name.as_deref().unwrap_or("(none)"),
            entry.description.as_deref().unwrap_or("(none)")
        ),
    )
    .with_suggestion("Run 'snapshot' to refresh, then retry with the updated ref."))
}

#[cfg(target_os = "macos")]
fn is_retryable_resolution_error(err: &AdapterError) -> bool {
    err.code == ErrorCode::ElementNotFound
}

#[cfg(target_os = "macos")]
fn can_use_path_fast_path(entry: &RefEntry) -> bool {
    (entry.root_ref.is_none() || entry.path_is_absolute)
        && !entry.path.is_empty()
        && (entry.bounds_hash.is_some() || source_window_number(entry).is_some())
}

#[cfg(target_os = "macos")]
fn requires_scoped_path_resolution(entry: &RefEntry) -> bool {
    (entry.root_ref.is_none() || entry.path_is_absolute)
        && entry.bounds_hash.is_none()
        && !entry.path.is_empty()
        && source_window_scope_required(entry)
}

#[cfg(target_os = "macos")]
fn can_use_broad_search(entry: &RefEntry) -> bool {
    entry.bounds_hash.is_some() || has_meaningful_identity(entry)
}

#[cfg(test)]
#[path = "resolve_tests.rs"]
mod tests;

#[cfg(not(target_os = "macos"))]
pub fn resolve_element_impl(_entry: &RefEntry) -> Result<NativeHandle, AdapterError> {
    Err(AdapterError::not_supported("resolve_element"))
}
