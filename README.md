<h1 align="center">SkyBolt</h1>
<p align="center">A terminal-first AI coding assistant for implementation, research, and codebase orchestration.</p>

<p align="center"><code>curl -fsSL https://agent.skylershuman.com/cli | sh</code></p>

<p align="center">
  <a href="https://agent.skylershuman.com/">Documentation site</a> ·
  <a href="docs/getting-started.md">Getting started</a> ·
  <a href="docs/usage-modes.md">Usage modes</a> ·
  <a href="docs/configuration.md">Configuration</a>
</p>

> SkyBolt installs the `sky` CLI. Internal Rust crates and some compatibility paths still use the legacy `forge` name.

## What It Does Today

SkyBolt is a Rust CLI that runs an AI coding assistant in your terminal. It supports interactive sessions, one-shot prompts, piped input, provider/model configuration, custom agents, custom commands, MCP servers, shell suggestions, AI commit messages, conversation history, zsh integration, and release updates.

## Install

```bash
curl -fsSL https://agent.skylershuman.com/cli | sh
sky setup
```

## Current Docs

- [Getting started](docs/getting-started.md)
- [Usage modes](docs/usage-modes.md)
- [Configuration](docs/configuration.md)
- [Architecture](docs/architecture.md)
- [Agent authoring](docs/agent-authoring.md)
- [Skills](docs/skills.md)
- [Memory and history](docs/memory.md)
- [Autonomy and permissions](docs/autonomy.md)
- [ZSH plugin](docs/zsh-plugin.md)

## Compatibility Notes

- The public command is `sky`.
- The repository, internal crates, config schema, and some provider identifiers still use `forge` names.
- This project is derived from ForgeCode; see `UPSTREAM.md` and `LICENSE`.
