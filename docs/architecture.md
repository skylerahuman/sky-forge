---
title: Architecture
---

# Architecture

SkyBolt is a Rust workspace. The current public binary is `sky`; internal crates still use `forge_*` names.

Important crates:

- `forge_main`: CLI entrypoint, terminal UI, command parsing, shell integration, logs, updates, and VS Code integration.
- `forge_app`: core application layer for agents, orchestration, tool execution, MCP execution, prompts, retries, git/workspace handling, and truncation.
- `forge_api`: public API facade over app/domain/config types.
- `forge_domain`: domain models, providers, tools, policies, conversations, and request/response structures.
- `forge_config`: configuration loading, merging, validation, and schema-related types.
- `forge_services`: local and remote service integrations, including context/workspace support.
- `forge_repo`: bundled agents, skills, providers, fixtures, and repository-backed resources.
- `forge_display`, `forge_select`, `forge_spinner`, and stream crates: terminal rendering and interactive UI support.

Release workflows build platform-specific `sky-*` binaries from the Rust workspace.
