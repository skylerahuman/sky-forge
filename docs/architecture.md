---
title: Architecture
---

# Architecture

This guide explains how Sky-Agent is organized and how work moves through the system.

## What to look for

- crate responsibilities and boundaries
- prompt assembly and system context rendering
- conversation flow from user input to tool output
- persistence and workspace indexing
- the difference between stable context and volatile runtime context

## Current implementation anchors

The current system prompt path is a useful entry point when reading the codebase. It assembles the system context in stages and separates stable information from rendered runtime content.

As this guide grows, it should stay aligned with the code that currently renders the system prompt, loads configuration, and dispatches tools.
