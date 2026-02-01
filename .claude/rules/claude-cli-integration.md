# Claude CLI Integration Task

## Goal
Add Claude Code CLI (`claude -p`) as post-processing provider alternative to OpenAI API.

## Prerequisites
Repo must be forked and cloned first:
```bash
gh repo fork cjpais/Handy --clone && cd Handy
bun install
mkdir -p src-tauri/resources/models
curl -o src-tauri/resources/models/silero_vad_v4.onnx https://blob.handy.computer/silero_vad_v4.onnx
```

## Discovery Phase
Search for existing post-processing:
```bash
rg -i "postprocess|post_process" --type rust --type ts
rg -i "openai|provider" --type rust --type ts
rg -i "experimental" --type rust --type ts
```

Find: settings structure, API call location, text flow.

## Implementation

### Rust Provider
```rust
use std::process::Command;

pub fn process_with_claude_cli(text: &str, prompt: &str) -> Result<String, String> {
    if text.trim().is_empty() {
        return Ok(text.to_string());
    }

    let output = Command::new("claude")
        .arg("-p")
        .arg(format!("{}\n\nText:\n{}", prompt, text))
        .output()
        .map_err(|e| format!("Claude CLI error: {}", e))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(if result.is_empty() { text.to_string() } else { result })
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn is_claude_cli_available() -> bool {
    Command::new("claude")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
```

### UI Changes
- Add "Claude Code CLI" to provider dropdown
- Show textarea for system prompt when selected
- Add "Test CLI" button (calls `claude --version`)
- Info text: "Requires Claude Code CLI with Pro/Max subscription"
- Hide API key field when CLI selected

### Default System Prompt (Czech)
```
Oprav gramatiku, interpunkci a velka pismena v tomto prepisu z speech-to-text.
Zachovej vyznam a anglicismy. Vrat POUZE opraveny text.
```

## Error Handling Rules
- NEVER lose original text
- On CLI failure: return original text + log warning
- Detect CLI unavailability and inform user
- Implement timeout (default 30s)

## Testing Checklist
- [ ] Various text lengths
- [ ] Fallback when CLI unavailable
- [ ] Czech and English text
- [ ] Windows/macOS/Linux if possible
