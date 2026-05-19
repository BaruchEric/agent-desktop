use super::*;
use crate::cli::Cli;
use crate::cli_args::ScreenshotArgs;
use agent_desktop_core::{PermissionReport, PermissionState};
use clap::CommandFactory;

#[test]
fn every_cli_subcommand_has_policy() {
    for subcommand in Cli::command().get_subcommands() {
        let name = subcommand.get_name();
        assert!(
            command_name_is_covered(name),
            "missing permission policy coverage for {name}"
        );
    }
}

fn command_name_is_covered(name: &str) -> bool {
    matches!(
        name,
        "snapshot"
            | "find"
            | "screenshot"
            | "get"
            | "is"
            | "click"
            | "double-click"
            | "triple-click"
            | "right-click"
            | "type"
            | "set-value"
            | "clear"
            | "focus"
            | "select"
            | "toggle"
            | "check"
            | "uncheck"
            | "expand"
            | "collapse"
            | "scroll"
            | "scroll-to"
            | "press"
            | "key-down"
            | "key-up"
            | "hover"
            | "drag"
            | "mouse-move"
            | "mouse-click"
            | "mouse-down"
            | "mouse-up"
            | "launch"
            | "close-app"
            | "list-windows"
            | "list-apps"
            | "focus-window"
            | "resize-window"
            | "move-window"
            | "minimize"
            | "maximize"
            | "restore"
            | "list-surfaces"
            | "list-notifications"
            | "dismiss-notification"
            | "dismiss-all-notifications"
            | "notification-action"
            | "clipboard-get"
            | "clipboard-set"
            | "clipboard-clear"
            | "wait"
            | "status"
            | "permissions"
            | "version"
            | "batch"
            | "skills"
    )
}

#[test]
fn unknown_permission_does_not_mask_platform_errors() {
    let report = PermissionReport::default();
    let command = Commands::Screenshot(ScreenshotArgs {
        app: None,
        window_id: None,
        output_path: None,
    });

    assert!(preflight(&command, &report).is_ok());
}

#[test]
fn screen_recording_denial_is_preflighted() {
    let report = PermissionReport {
        accessibility: PermissionState::Granted,
        screen_recording: PermissionState::Denied {
            suggestion: "grant screen recording".into(),
        },
        automation: PermissionState::NotRequired,
    };
    let command = Commands::Screenshot(ScreenshotArgs {
        app: None,
        window_id: None,
        output_path: None,
    });

    let err = preflight(&command, &report).expect_err("denied screen capture fails");

    assert_eq!(err.code(), "PERM_DENIED");
}

#[test]
fn accessibility_denial_does_not_preflight_ax_commands() {
    let report = PermissionReport {
        accessibility: PermissionState::Denied {
            suggestion: "grant accessibility".into(),
        },
        screen_recording: PermissionState::Granted,
        automation: PermissionState::NotRequired,
    };
    let command = Commands::Click(crate::cli_args::RefArgs {
        ref_id: "@e1".into(),
        snapshot_id: None,
    });

    preflight(&command, &report).expect("AX command should execute and report adapter result");
}
