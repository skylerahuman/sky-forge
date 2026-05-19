---
title: Rust Standards
---

# Rust Standards

SkyBolt is built as a Rust 2024 workspace using the toolchain pinned in `rust-toolchain.toml`.

Current standards and checks include:

- Workspace crates under `crates/*`.
- Shared dependencies declared in the root `Cargo.toml`.
- Formatting configured by `.rustfmt.toml`.
- Lints configured through `clippy.toml` and CI `RUSTFLAGS=-Dwarnings`.
- Snapshot testing with `insta` where used.
- CI command: `cargo test --workspace`.
