use super::{CliAdapter, LineType, ParsedLine};
use crate::storage::models::CliType;
use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

pub struct ClaudeCodeAdapter {
    path: Option<String>,
}

#[derive(Deserialize)]
struct StreamJsonLine {
    #[serde(rename = "type")]
    msg_type: Option<String>,
    content: Option<String>,
    role: Option<String>,
}

impl ClaudeCodeAdapter {
    pub fn new() -> Self {
        let path = which::which("claude")
            .ok()
            .map(|p| p.to_string_lossy().to_string());
        Self { path }
    }
}

#[async_trait]
impl CliAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &str {
        "Claude Code"
    }

    fn cli_type(&self) -> CliType {
        CliType::Claude
    }

    fn is_installed(&self) -> bool {
        self.path.is_some()
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    async fn version(&self) -> Option<String> {
        let output = Command::new("claude")
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
        let mut cmd = Command::new("claude");
        cmd.current_dir(working_dir)
            .arg("--print")
            .arg(prompt)
            .arg("--dangerously-skip-permissions")
            .arg("--output-format")
            .arg("stream-json")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    fn build_readonly_command(&self, prompt: &str, working_dir: &Path) -> Command {
        let mut cmd = Command::new("claude");
        cmd.current_dir(working_dir)
            .arg("--print")
            .arg(prompt)
            .arg("--output-format")
            .arg("text")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    fn detect_completion(&self, output: &str, signal: &str) -> bool {
        // Parse JSON lines and check only assistant content
        for line in output.lines() {
            if let Ok(parsed) = serde_json::from_str::<StreamJsonLine>(line) {
                if parsed.role.as_deref() == Some("assistant") {
                    if let Some(content) = &parsed.content {
                        if content.contains(signal) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn parse_output_line(&self, line: &str) -> ParsedLine {
        if let Ok(parsed) = serde_json::from_str::<StreamJsonLine>(line) {
            ParsedLine {
                content: parsed.content.unwrap_or_default(),
                line_type: LineType::Json,
                is_assistant: parsed.role.as_deref() == Some("assistant"),
            }
        } else {
            ParsedLine {
                content: line.to_string(),
                line_type: LineType::Text,
                is_assistant: false,
            }
        }
    }
}
