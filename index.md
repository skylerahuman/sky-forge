---
title: SkyBolt
---

# SkyBolt

SkyBolt is a terminal-first AI coding assistant. It currently ships as the `sky` CLI and keeps some legacy `forge` internals for compatibility.

## Install

```bash
curl -fsSL https://agent.skylershuman.com/cli | sh
sky setup
```

The installer downloads release binaries from [`skylerahuman/sky-forge`](https://github.com/skylerahuman/sky-forge).

## Current Functionality

- Interactive terminal chat for coding work.
- One-shot prompts with `sky --prompt` and piped stdin.
- Provider, model, and authentication management.
- Custom agents, custom commands, and skills loaded from the repository or user config.
- Built-in tools for file edits, search, shell commands, web fetches, todos, and task delegation.
- MCP server configuration and execution.
- Conversation history, resume support, logs, and JSON/HTML dumps.
- ZSH integration, shell suggestions, and release updates.

## Docs

- [Getting Started]({{ '/getting-started.html' | relative_url }})
- [Usage Modes]({{ '/usage-modes.html' | relative_url }})
- [Configuration]({{ '/configuration.html' | relative_url }})
- [Architecture]({{ '/architecture.html' | relative_url }})
- [Agent Authoring]({{ '/agent-authoring.html' | relative_url }})
- [Skills]({{ '/skills.html' | relative_url }})
- [Memory]({{ '/memory.html' | relative_url }})
- [Autonomy]({{ '/autonomy.html' | relative_url }})
- [ZSH Plugin]({{ '/zsh-plugin.html' | relative_url }})
