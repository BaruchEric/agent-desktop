use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalKind {
    Shell,
    AppleScript,
    Jxa,
    OpenUrl,
    OpenPath,
}

impl ExternalKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExternalKind::Shell => "shell",
            ExternalKind::AppleScript => "applescript",
            ExternalKind::Jxa => "jxa",
            ExternalKind::OpenUrl => "open-url",
            ExternalKind::OpenPath => "open-path",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalRequest {
    pub kind: ExternalKind,
    pub payload: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_result_serializes_flat() {
        let r = ExternalResult {
            exit_code: 0,
            stdout: "hi\n".into(),
            stderr: String::new(),
            duration_ms: 12,
        };
        let v = serde_json::to_value(&r).unwrap();
        assert_eq!(v["exit_code"], 0);
        assert_eq!(v["stdout"], "hi\n");
        assert_eq!(v["stderr"], "");
        assert_eq!(v["duration_ms"], 12);
    }
}
