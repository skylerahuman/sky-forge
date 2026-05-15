---
title: Agent Authoring
---

# Agent Authoring

This guide explains how to define custom agents for Sky-Agent.

## What a custom agent needs

A custom agent should clearly describe:

- its purpose
- the tasks it should handle
- the tools it can use
- any limits or boundaries it should follow
- the user prompt context it expects

## File structure

Agents should live in the workspace’s agent directory and follow the existing frontmatter pattern used by built-in agents.

## Design goals

Custom agents should be:

- specific rather than vague
- bounded rather than all-purpose
- compatible with the current tool registry
- easy to reason about from the docs alone

## Compatibility

During the transition, built-in agent naming may still reflect Forge-era conventions in source files, but the docs should describe the Sky-Agent behavior model.
