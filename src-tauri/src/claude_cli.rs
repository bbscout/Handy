use log::{debug, warn};
use std::process::Command;
use std::time::Duration;

/// Default timeout for Claude CLI operations (30 seconds)
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default model for Claude CLI (fastest and cheapest)
pub const DEFAULT_CLAUDE_MODEL: &str = "haiku";

/// Available Claude CLI models
pub const CLAUDE_CLI_MODELS: &[(&str, &str)] = &[
    ("haiku", "Haiku (fastest, cheapest)"),
    ("sonnet", "Sonnet (balanced)"),
    ("opus", "Opus (most capable)"),
];

/// Process text using Claude Code CLI (`claude -p`).
/// Returns Ok(processed_text) on success, or Err with error message.
/// IMPORTANT: On error, caller should fall back to original text.
pub fn process_with_claude_cli(text: &str, prompt: &str, model: &str) -> Result<String, String> {
    if text.trim().is_empty() {
        return Ok(text.to_string());
    }

    let full_prompt = format!("{}\n\nText:\n{}", prompt, text);
    let model_to_use = if model.is_empty() { DEFAULT_CLAUDE_MODEL } else { model };

    debug!(
        "Calling Claude CLI with model '{}', prompt length: {} chars",
        model_to_use,
        full_prompt.len()
    );

    // Use spawn + wait_with_output for timeout support
    let mut child = Command::new("claude")
        .arg("--model")
        .arg(model_to_use)
        .arg("-p")
        .arg(&full_prompt)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn Claude CLI: {}", e))?;

    // Wait with timeout
    let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let start = std::time::Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                // Process has exited
                let output = child
                    .wait_with_output()
                    .map_err(|e| format!("Failed to read Claude CLI output: {}", e))?;

                if status.success() {
                    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if result.is_empty() {
                        debug!("Claude CLI returned empty response, using original text");
                        return Ok(text.to_string());
                    }
                    debug!("Claude CLI succeeded, output length: {} chars", result.len());
                    return Ok(result);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Claude CLI failed: {}", stderr));
                }
            }
            Ok(None) => {
                // Process still running
                if start.elapsed() > timeout {
                    // Kill the process
                    let _ = child.kill();
                    return Err(format!(
                        "Claude CLI timed out after {} seconds",
                        DEFAULT_TIMEOUT_SECS
                    ));
                }
                // Sleep briefly before checking again
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(format!("Error waiting for Claude CLI: {}", e));
            }
        }
    }
}

/// Check if Claude Code CLI is available and working.
/// Returns true if `claude --version` succeeds.
pub fn is_claude_cli_available() -> bool {
    match Command::new("claude").arg("--version").output() {
        Ok(output) => {
            let available = output.status.success();
            if available {
                let version = String::from_utf8_lossy(&output.stdout);
                debug!("Claude CLI available: {}", version.trim());
            } else {
                debug!("Claude CLI not available (exit code: {:?})", output.status.code());
            }
            available
        }
        Err(e) => {
            warn!("Claude CLI not found: {}", e);
            false
        }
    }
}

/// Get list of available Claude CLI models for frontend
pub fn get_claude_cli_models() -> Vec<(String, String)> {
    CLAUDE_CLI_MODELS
        .iter()
        .map(|(id, label)| (id.to_string(), label.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text_returns_empty() {
        let result = process_with_claude_cli("", "Fix grammar", "haiku");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_whitespace_text_returns_whitespace() {
        let result = process_with_claude_cli("   ", "Fix grammar", "haiku");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "   ");
    }
}
