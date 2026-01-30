use super::{CliAdapter, CommandOptions, LineType, ParsedLine};
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

    fn exec_args(prompt: &str, options: CommandOptions) -> Vec<String> {
        let mut args = vec!["exec".to_string(), "--full-auto".to_string()];
        if options.skip_git_repo_check {
            args.push("--skip-git-repo-check".to_string());
        }
        args.push(prompt.to_string());
        args
    }

    fn readonly_args(prompt: &str, options: CommandOptions) -> Vec<String> {
        let mut args = vec![
            "exec".to_string(),
            "--sandbox".to_string(),
            "read-only".to_string(),
        ];
        if options.skip_git_repo_check {
            args.push("--skip-git-repo-check".to_string());
        }
        args.push(prompt.to_string());
        args
    }

    fn build_exec_command(
        &self,
        prompt: &str,
        working_dir: &Path,
        readonly: bool,
        options: CommandOptions,
    ) -> Command {
        let mut cmd = Command::new("codex");
        let args = if readonly {
            Self::readonly_args(prompt, options)
        } else {
            Self::exec_args(prompt, options)
        };
        cmd.current_dir(working_dir)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
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

    fn build_command(&self, prompt: &str, working_dir: &Path, options: CommandOptions) -> Command {
        self.build_exec_command(prompt, working_dir, false, options)
    }

    fn build_readonly_command(
        &self,
        prompt: &str,
        working_dir: &Path,
        options: CommandOptions,
    ) -> Command {
        self.build_exec_command(prompt, working_dir, true, options)
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

#[cfg(test)]
mod tests {
    use super::CodexAdapter;
    use super::CommandOptions;

    #[test]
    fn exec_args_include_exec_and_full_auto() {
        let args = CodexAdapter::exec_args("hello", CommandOptions::default());
        assert_eq!(args, vec!["exec", "--full-auto", "hello"]);
    }

    #[test]
    fn readonly_args_use_read_only_sandbox() {
        let args = CodexAdapter::readonly_args("hello", CommandOptions::default());
        assert_eq!(args, vec!["exec", "--sandbox", "read-only", "hello"]);
    }

    #[test]
    fn exec_args_include_skip_git_repo_check() {
        let args = CodexAdapter::exec_args(
            "hello",
            CommandOptions {
                skip_git_repo_check: true,
            },
        );
        assert_eq!(
            args,
            vec!["exec", "--full-auto", "--skip-git-repo-check", "hello"]
        );
    }

    #[test]
    fn readonly_args_include_skip_git_repo_check() {
        let args = CodexAdapter::readonly_args(
            "hello",
            CommandOptions {
                skip_git_repo_check: true,
            },
        );
        assert_eq!(
            args,
            vec!["exec", "--sandbox", "read-only", "--skip-git-repo-check", "hello"]
        );
    }
}
