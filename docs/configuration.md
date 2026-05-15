---
title: Configuration
---

# Configuration

Sky-Agent is designed to be configuration-first.

## What that means

The project should prefer explicit, documented settings over hidden defaults or prompt-only behavior. When a feature can be configured, it should be represented in configuration and reflected in the schema.

## Topics

- Provider setup and credentials
- Default model selection
- Session overrides
- Workspace and semantic search settings
- Prompt and agent configuration
- User, project, and workspace precedence

## Precedence

In practice, configuration should be readable in layers:

1. Built-in defaults
2. User-level settings
3. Project-level settings
4. Session overrides or explicit command input

## Compatibility

Legacy Forge/ForgeCode configuration keys remain documented only where they are still required during the transition.
