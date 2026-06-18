use crate::{
    adapter::PlatformAdapter,
    error::{AdapterError, AppError, ErrorCode},
};
use serde_json::{Value, json};

pub struct MenuArgs {
    pub app: Option<String>,
    pub path: Option<String>,
    pub list: bool,
}

pub(crate) fn parse_path(raw: &str) -> Vec<String> {
    raw.split('>')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn execute(args: MenuArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    let pid = resolve_pid(args.app.as_deref(), adapter)?;
    if args.list {
        let paths = adapter.list_menu_paths(pid)?;
        return Ok(json!({ "paths": paths }));
    }
    if let Some(p) = args.path {
        let segments = parse_path(&p);
        if segments.is_empty() {
            return Err(AppError::invalid_input("menu --path must be non-empty"));
        }
        adapter.select_menu_path(pid, &segments)?;
        return Ok(json!({ "action": "menu", "path": segments }));
    }
    Err(AppError::invalid_input(
        "Provide --path \"A > B\" or --list",
    ))
}

fn resolve_pid(app: Option<&str>, adapter: &dyn PlatformAdapter) -> Result<i32, AppError> {
    let app = app.ok_or_else(|| AppError::invalid_input("menu --path requires --app"))?;
    let apps = adapter.list_apps()?;
    apps.into_iter()
        .find(|a| a.name.eq_ignore_ascii_case(app))
        .map(|a| a.pid)
        .ok_or_else(|| {
            AppError::Adapter(AdapterError::new(
                ErrorCode::AppNotFound,
                format!("Application '{app}' is not running"),
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::parse_path;

    #[test]
    fn splits_and_trims_segments() {
        assert_eq!(
            parse_path("Format > Make Plain Text"),
            vec!["Format".to_string(), "Make Plain Text".to_string()]
        );
    }

    #[test]
    fn single_segment_is_kept() {
        assert_eq!(parse_path("File"), vec!["File".to_string()]);
    }
}
