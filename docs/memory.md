---
title: Memory
---

# Memory

Sky-Agent should use a native memory model that keeps user, project, and workspace context separate.

## Memory scopes

- **Global user memory**: durable preferences that follow the user
- **Project memory**: notes that belong to a specific repository or workspace
- **Conversation memory**: short-lived context for the current thread
- **Workspace context**: indexed source material that is retrieved when relevant

## Design goals

Memory should be:

- local-first
- explicit about scope
- safe across repositories
- easy to inspect and reason about

## Compatibility

This guide describes the target memory model that Sky-Agent should evolve toward, rather than assuming a shared Claude-style memory source.
