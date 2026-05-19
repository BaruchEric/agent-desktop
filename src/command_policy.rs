use agent_desktop_core::{
    PermissionReport,
    error::{AdapterError, AppError, ErrorCode},
};

use crate::cli::Commands;

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
    let permission = policy_for(cmd);
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

fn requires_screen_recording(permission: PermissionNeed) -> bool {
    matches!(
        permission,
        PermissionNeed::ScreenRecording | PermissionNeed::AccessibilityAndScreenRecording
    )
}

#[cfg(test)]
#[path = "command_policy_tests.rs"]
mod tests;
