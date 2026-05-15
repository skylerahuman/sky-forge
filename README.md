<h1 align="center">Sky-Agent</h1>
<p align="center">A lightweight AI coding assistant for terminal-first development.</p>

<p align="center"><code>curl -fsSL https://forgecode.dev/cli | sh</code></p>

<p align="center">
  <a href="index.md">Documentation site</a> ·
  <a href="docs/getting-started.md">Getting started</a> ·
  <a href="docs/usage-modes.md">Usage modes</a> ·
  <a href="docs/configuration.md">Configuration</a>
</p>

> Sky-Agent currently ships with the existing `forge` CLI while the branding migration is in progress.

## What it is

Sky-Agent is a terminal-first coding assistant for rapid implementation, research, and orchestration.

## Quick links

- [Docs site](index.md)
- [Getting started](docs/getting-started.md)
- [Usage modes](docs/usage-modes.md)
- [ZSH plugin](docs/zsh-plugin.md)
- [Configuration](docs/configuration.md)
- [Architecture](docs/architecture.md)
- [Agent authoring](docs/agent-authoring.md)
- [Skills](docs/skills.md)
- [Memory](docs/memory.md)
- [Autonomy](docs/autonomy.md)
- [Rust standards](docs/rust-standards.md)

## Transition notes

- The current CLI command remains `forge` during the migration.
- Legacy Forge/ForgeCode terminology is retained only for compatibility, paths, or migration notes.
- The public documentation surface is the GitHub Pages site rooted at `index.md`.

When this limit is reached, Forge will:

- Ask you if you wish to continue
- If you respond with 'Yes', it will continue the conversation
- If you respond with 'No', it will end the conversation

</details>

---

<details>
<summary><strong>Model Context Protocol (MCP)</strong></summary>

The MCP feature allows AI agents to communicate with external tools and services. This implementation follows Anthropic's [Model Context Protocol](https://docs.anthropic.com/en/docs/claude-code/tutorials#set-up-model-context-protocol-mcp) design.

### MCP Configuration

Configure MCP servers using the CLI:

```bash
# List all MCP servers
forge mcp list

# Import a server from JSON
forge mcp import

# Show server configuration details
forge mcp show

# Remove a server
forge mcp remove

# Reload servers and rebuild caches
forge mcp reload
```

Or manually create a `.mcp.json` file with the following structure:

```json
{
  "mcpServers": {
    "server_name": {
      "command": "command_to_execute",
      "args": ["arg1", "arg2"],
      "env": { "ENV_VAR": "value" }
    },
    "another_server": {
      "url": "http://localhost:3000/events"
    }
  }
}
```

MCP configurations are read from two locations (project-local takes precedence):

1. **Project-local:** `.mcp.json` in your project directory
2. **Global:** `~/forge/.mcp.json`

### Example Use Cases

MCP can be used for various integrations:

- Web browser automation
- External API interactions
- Tool integration
- Custom service connections

### Usage in Multi-Agent Workflows

MCP tools can be used as part of multi-agent workflows, allowing specialized agents to interact with external systems as part of a collaborative problem-solving approach.

</details>

---

## Documentation

For comprehensive documentation on all features and capabilities, please visit the [documentation site](https://github.com/tailcallhq/forgecode/tree/main/docs).

---

## Installation

```bash
# YOLO
curl -fsSL https://forgecode.dev/cli | sh

# Package managers
nix run github:tailcallhq/forgecode # for latest dev branch
```

---

## Community

Join our vibrant Discord community to connect with other Forge users and contributors, get help with your projects, share ideas, and provide feedback!

[![Discord](https://img.shields.io/discord/1044859667798568962?style=for-the-badge&cacheSeconds=120&logo=discord)](https://discord.gg/kRZBPpkgwq)

---

## Support Us

Your support drives Forge's continued evolution! By starring our GitHub repository, you:

- Help others discover this powerful tool 🔍
- Motivate our development team 💪
- Enable us to prioritize new features 🛠️
- Strengthen our open-source community 🌱
