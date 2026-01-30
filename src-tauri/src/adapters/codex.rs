use super::{CliAdapter, LineType, ParsedLine};
use crate::storage::models::CliType;
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

pub struct CodexAdapter {
    path: Option<String>,
}

impl CodexAdapter {
    pub fn new() -> Self {
        let path = which::which("codex")
            .ok()
            .map(|p| p.to_string_lossy().to_string());
        Self { path }
    }
}

#[async_trait]
impl CliAdapter for CodexAdapter {
    fn name(&self) -> &str {
        "Codex CLI"
    }

    fn cli_type(&self) -> CliType {
        CliType::Codex
    }

    fn is_installed(&self) -> bool {
        self.path.is_some()
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    async fn version(&self) -> Option<String> {
        let output = Command::new("codex")
            .arg("--version")
            .output()
            .await
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    fn build_command(&self, prompt: &str, working_dir: &Path) -> Command {
        let mut cmd = Command::new("codex");
        cmd.current_dir(working_dir)
            .arg("--quiet")
            .arg("--auto-edit")
            .arg("--full-auto")
            .arg(prompt)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    fn build_readonly_command(&self, prompt: &str, working_dir: &Path) -> Command {
        let mut cmd = Command::new("codex");
        cmd.current_dir(working_dir)
            .arg("--quiet")
            .arg(prompt)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    fn detect_completion(&self, output: &str, signal: &str) -> bool {
        // Codex output is plain text, direct detection
        output.contains(signal)
    }

    fn parse_output_line(&self, line: &str) -> ParsedLine {
        ParsedLine {
            content: line.to_string(),
            line_type: LineType::Text,
            is_assistant: true, // All Codex output is treated as assistant
        }
    }
}
