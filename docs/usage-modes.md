---
title: Usage Modes
---

# Usage Modes

Sky-Agent supports three ways to work.

## Interactive mode

Use interactive mode when you want to guide a longer task and let the agent keep track of the path to completion.

Best for:

- multi-step implementation work
- debugging with back-and-forth steering
- tasks where you want the agent to stay conversational

## One-shot mode

Use one-shot mode when you want a single request answered quickly.

Best for:

- small edits
- direct questions
- scripted terminal workflows

## ZSH plugin mode

Use ZSH plugin mode when you want a very fast terminal-native loop.

Best for:

- rapid iteration from the shell
- `:`-prefixed commands
- attaching files or switching agents without leaving the terminal

See the dedicated [ZSH Plugin]({{ '/zsh-plugin.html' | relative_url }}) guide for the command reference.
