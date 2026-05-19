---
title: Agent Authoring
---

# Agent Authoring

SkyBolt currently supports custom agents loaded from repository or user configuration.

Agents define behavior such as model/provider selection, system instructions, tools, and permissions. Bundled agents live under `crates/sky_repo/src/agents/`, and project-local agents can be added through the existing Forge-compatible configuration paths.

The CLI exposes agent management through `sky agent` and listing through `sky list agent`.
