use agent_desktop_core::{
    PermissionReport,
    error::{AdapterError, AppError, ErrorCode},
};

use crate::cli::Commands;
use crate::dispatch_parse::{
    parse_direction, parse_get_property, parse_is_property, parse_mouse_button, parse_xy,
    parse_xy_opt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PermissionNeed {
    None,
    Accessibility,
    ScreenRecording,
    AccessibilityAndScreenRecording,
}

pub(crate) fn policy_for(cmd: &Commands) -> PermissionNeed {
    use PermissionNeed::{Accessibility, AccessibilityAndScreenRecording, None, ScreenRecording};
    match cmd {
        Commands::Version(_) | Commands::Skills(_) => None,
        Commands::Status | Commands::Permissions(_) => None,
        Commands::ListWindows(_) | Commands::ListApps(_) => None,
        Commands::ClipboardGet | Commands::ClipboardSet(_) | Commands::ClipboardClear => None,
        Commands::Batch(_) => None,

        Commands::Snapshot(_)
        | Commands::Find(_)
        | Commands::ListSurfaces(_)
        | Commands::Wait(_)
        | Commands::ListNotifications(_) => Accessibility,

        Commands::Screenshot(a) if a.app.is_some() || a.window_id.is_some() => {
            AccessibilityAndScreenRecording
        }
        Commands::Screenshot(_) => ScreenRecording,

        Commands::Get(_) | Commands::Is(_) => Accessibility,

        Commands::Click(_)
        | Commands::DoubleClick(_)
        | Commands::TripleClick(_)
        | Commands::RightClick(_)
        | Commands::SetValue(_)
        | Commands::Clear(_)
        | Commands::Select(_)
        | Commands::Toggle(_)
        | Commands::Check(_)
        | Commands::Uncheck(_)
        | Commands::Expand(_)
        | Commands::Collapse(_)
        | Commands::Scroll(_)
        | Commands::ScrollTo(_) => Accessibility,

        Commands::Type(_) => Accessibility,
        Commands::Focus(_) => Accessibility,
        Commands::Press(_) | Commands::KeyDown(_) | Commands::KeyUp(_) => Accessibility,
        Commands::Hover(_)
        | Commands::Drag(_)
        | Commands::MouseMove(_)
        | Commands::MouseClick(_)
        | Commands::MouseDown(_)
        | Commands::MouseUp(_) => Accessibility,

        Commands::Launch(_)
        | Commands::CloseApp(_)
        | Commands::FocusWindow(_)
        | Commands::ResizeWindow(_)
        | Commands::MoveWindow(_)
        | Commands::Minimize(_)
        | Commands::Maximize(_)
        | Commands::Restore(_)
        | Commands::DismissNotification(_)
        | Commands::DismissAllNotifications(_)
        | Commands::NotificationAction(_) => Accessibility,
    }
}

pub(crate) fn preflight(cmd: &Commands, report: &PermissionReport) -> Result<(), AppError> {
    validate_args(cmd)?;
    let permission = policy_for(cmd);
    if requires_accessibility(permission) && report.accessibility_denied() {
        let err = AdapterError::new(
            ErrorCode::PermDenied,
            "Accessibility permission not granted",
        )
        .with_suggestion(
            report
                .accessibility_suggestion()
                .unwrap_or("Grant Accessibility permission and retry"),
        );
        return Err(AppError::Adapter(err));
    }
    if requires_screen_recording(permission) && report.screen_recording_denied() {
        let err = AdapterError::new(
            ErrorCode::PermDenied,
            "Screen Recording permission not granted",
        )
        .with_suggestion(
            report
                .screen_recording_suggestion()
                .unwrap_or("Grant Screen Recording permission and retry"),
        );
        return Err(AppError::Adapter(err));
    }
    Ok(())
}

fn validate_args(cmd: &Commands) -> Result<(), AppError> {
    match cmd {
        Commands::Snapshot(args) => {
            if let Some(root) = &args.root {
                if args.surface != crate::cli_args::Surface::Window {
                    return Err(AppError::invalid_input(
                        "--root cannot be combined with --surface",
                    ));
                }
                validate_cli_ref_id(root)?;
            }
        }
        Commands::Get(args) => {
            validate_cli_ref_id(&args.ref_id)?;
            parse_get_property(&args.property)?;
        }
        Commands::Is(args) => {
            validate_cli_ref_id(&args.ref_id)?;
            parse_is_property(&args.property)?;
        }
        Commands::Click(args)
        | Commands::DoubleClick(args)
        | Commands::TripleClick(args)
        | Commands::RightClick(args)
        | Commands::Clear(args)
        | Commands::Focus(args)
        | Commands::Toggle(args)
        | Commands::Check(args)
        | Commands::Uncheck(args)
        | Commands::Expand(args)
        | Commands::Collapse(args)
        | Commands::ScrollTo(args) => {
            validate_cli_ref_id(&args.ref_id)?;
        }
        Commands::Type(args) => validate_cli_ref_id(&args.ref_id)?,
        Commands::SetValue(args) => validate_cli_ref_id(&args.ref_id)?,
        Commands::Select(args) => validate_cli_ref_id(&args.ref_id)?,
        Commands::Scroll(args) => {
            validate_cli_ref_id(&args.ref_id)?;
            parse_direction(&args.direction)?;
        }
        Commands::Hover(args) => {
            if let Some(ref_id) = &args.ref_id {
                validate_cli_ref_id(ref_id)?;
            }
            parse_xy_opt(args.xy.as_deref())?;
        }
        Commands::Drag(args) => {
            if let Some(ref_id) = &args.from {
                validate_cli_ref_id(ref_id)?;
            }
            if let Some(ref_id) = &args.to {
                validate_cli_ref_id(ref_id)?;
            }
            parse_xy_opt(args.from_xy.as_deref())?;
            parse_xy_opt(args.to_xy.as_deref())?;
        }
        Commands::MouseMove(args) => {
            parse_xy(&args.xy)?;
        }
        Commands::MouseClick(args) => {
            parse_xy(&args.xy)?;
            parse_mouse_button(&args.button)?;
        }
        Commands::MouseDown(args) | Commands::MouseUp(args) => {
            parse_xy(&args.xy)?;
            parse_mouse_button(&args.button)?;
        }
        Commands::Wait(args) => {
            if let Some(ref_id) = &args.element {
                validate_cli_ref_id(ref_id)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_cli_ref_id(ref_id: &str) -> Result<(), AppError> {
    let valid = ref_id.starts_with("@e")
        && ref_id.len() >= 3
        && ref_id.len() <= 12
        && ref_id[2..].chars().all(|c| c.is_ascii_digit())
        && ref_id[2..].parse::<u32>().is_ok_and(|n| n > 0);
    if valid {
        return Ok(());
    }
    Err(AppError::invalid_input(format!(
        "Invalid ref_id '{ref_id}': must match @e{{N}} where N is a positive integer"
    )))
}

fn requires_accessibility(permission: PermissionNeed) -> bool {
    matches!(
        permission,
        PermissionNeed::Accessibility | PermissionNeed::AccessibilityAndScreenRecording
    )
}

fn requires_screen_recording(permission: PermissionNeed) -> bool {
    matches!(
        permission,
        PermissionNeed::ScreenRecording | PermissionNeed::AccessibilityAndScreenRecording
    )
}

#[cfg(test)]
#[path = "command_policy_tests.rs"]
mod tests;
