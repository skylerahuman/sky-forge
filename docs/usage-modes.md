---
title: Usage Modes
---

# Usage Modes

SkyBolt currently supports these usage modes:

- Interactive terminal sessions with `sky`.
- Single-turn execution with `sky --prompt "..."`.
- Piped input, for example `cat prompt.md | sky`.
- Directory-scoped runs with `sky --directory <path>`.
- Sandbox worktrees with `sky --sandbox <name>`.
- JSON conversation execution through `sky --conversation <file>`.
- Natural-language shell suggestions with `sky suggest "..."`.
- AI-generated commit messages through `sky commit`.
- Interactive selectors through `sky select`.

The CLI also exposes commands for agents, providers, models, config, conversations, MCP servers, workspace operations, logs, updates, VS Code integration, and ZSH integration.
