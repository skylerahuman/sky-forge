---
title: Rust Standards
---

# Rust Standards

Sky-Agent uses a strict but practical Rust style that favors clarity, explicit boundaries, and source-backed behavior.

## Core expectations

- write rustdoc for public APIs
- prefer explicit config and domain boundaries
- keep services focused and composable
- use `anyhow::Result` at service boundaries and `thiserror` for domain errors
- avoid unnecessary trait objects and oversized generic APIs
- keep tests close to the source they cover
- keep prompt and template changes snapshot-friendly

## Documentation and config

When behavior changes, update the relevant docs and keep schema-backed configuration aligned with the Rust types.
