---
title: Autonomy
---

# Autonomy

Sky-Agent should have a clear autonomy contract that changes by mode but stays consistent per agent.

## Goals

- ZSH mode should feel fast, terse, and iteration-friendly.
- Interactive mode should feel guided, collaborative, and stateful.
- The autonomy contract should describe when the agent may proceed and when it should ask for direction.

## Design principles

- avoid per-mode agent duplication
- keep the baseline contract explicit
- distinguish rapid shell iteration from guided interactive work
- make agent-specific autonomy easy to override when needed

## Compatibility

This guide defines the target contract for Sky-Agent and should stay aligned with the active tool and prompt behavior as it evolves.
