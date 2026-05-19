---
title: Memory
---

# Memory

SkyBolt currently stores and resumes conversation state through its existing conversation history system.

Implemented memory-related features include:

- Conversation IDs with `--conversation-id` / `--cid`.
- Conversation history and session management through `sky conversation`.
- JSON conversation execution with `--conversation <file>`.
- Logs through `sky logs`.
- Optional automatic dumps in JSON or HTML, depending on configuration.

This page describes current persistence behavior, not a long-term memory roadmap.
