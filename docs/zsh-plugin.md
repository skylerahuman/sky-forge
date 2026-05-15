---
title: ZSH Plugin
---

# ZSH Plugin

The ZSH plugin provides `:`-prefixed commands for fast, terminal-native interaction with Sky-Agent.

## Core ideas

- `:` sends a prompt to the active agent.
- `:agent` switches the active agent.
- `:@` file tagging attaches files to the current prompt.
- `:help` should stay focused on shell workflow, session switching, and configuration behavior.

## Workflow expectations

The ZSH mode should favor:

- short prompts
- quick follow-up turns
- minimal conversation overhead
- direct access to the current working directory and selected files

## Compatibility

The shell workflow still uses the existing `forge` CLI during the transition, so examples and command names may remain legacy-compatible until the rename is complete.
