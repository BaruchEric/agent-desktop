//! Real-app e2e: the `batch` command runs a JSON array of commands through the
//! same policy + dispatch path as the CLI.
//!
//! Uses read-only sub-commands so the assertions are deterministic. The batch
//! payload is passed as a single positional JSON-array argument.
#![cfg(target_os = "macos")]

mod common;
use common::*;

#[test]
#[ignore = "drives the real adapter via batch dispatch"]
fn batch_runs_every_read_command() {
    if !bin_exists() {
        return;
    }
    let payload = r#"[{"command":"version"},{"command":"status"},{"command":"list-apps"}]"#;
    let v = run(&["batch", payload]);
    assert_eq!(v["ok"], true, "batch must succeed: {v}");

    let results = v["data"]["results"]
        .as_array()
        .unwrap_or_else(|| panic!("batch data.results must be an array: {v}"));
    assert_eq!(
        results.len(),
        3,
        "batch must return one result per command: {v}"
    );
    assert_eq!(
        results[0]["command"], "version",
        "results must echo command: {v}"
    );
    assert!(
        results.iter().all(|r| r["ok"].is_boolean()),
        "every batch result must carry an ok flag: {v}"
    );
}

#[test]
#[ignore = "drives the real adapter via batch dispatch"]
fn batch_without_stop_runs_all_despite_failure() {
    if !bin_exists() {
        return;
    }
    let payload = r#"[{"command":"version"},{"command":"click","args":{"ref_id":"@e99999","snapshot":"no-such-snapshot"}},{"command":"version"}]"#;
    let v = run(&["batch", payload]);

    let results = v["data"]["results"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        results.len(),
        3,
        "without --stop-on-error all three commands run: {v}"
    );
    assert_eq!(
        results[1]["ok"], false,
        "the middle command targets a bogus ref and must report failure: {v}"
    );
}

#[test]
#[ignore = "drives the real adapter via batch dispatch"]
fn batch_stop_on_error_halts_early() {
    if !bin_exists() {
        return;
    }
    let payload = r#"[{"command":"version"},{"command":"click","args":{"ref_id":"@e99999","snapshot":"no-such-snapshot"}},{"command":"version"}]"#;
    let v = run(&["batch", payload, "--stop-on-error"]);

    let results = v["data"]["results"].as_array().cloned().unwrap_or_default();
    assert!(
        results.len() <= 2,
        "--stop-on-error must halt before the third command runs: {v}"
    );
}
