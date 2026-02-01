# Handy - Claude Code CLI Integration

Offline speech-to-text (Tauri: Rust + React/TypeScript). Adding `claude -p` as post-processing provider.

## Build Commands
```bash
bun install                    # Install dependencies
bun run tauri dev              # Dev mode (compiles Rust + React)
bun run tauri build            # Production build
```

## Code Style
- Follow existing project conventions
- Rust: standard formatting, error handling with Result
- TypeScript/React: existing component patterns

## Workflow Rules
- IMPORTANT: Never lose original text - always fallback on error
- Minimize changes to existing files (merge-friendly)
- Test with Czech and English text

## Task Reference
See @.claude/rules/claude-cli-integration.md for detailed implementation instructions.
