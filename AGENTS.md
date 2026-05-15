# Sky-Agent Code Standards

This document contains guidelines and best practices for AI agents and contributors working with this codebase.

## Naming and Branding

- Use **Sky-Agent** for user-facing documentation, landing pages, and explanatory copy.
- Use legacy Forge/ForgeCode naming only when referring to existing commands, paths, compatibility notes, or migration details.
- Prefer clear compatibility language whenever the current CLI or on-disk structure still uses legacy names.

## Error Management

- Use `anyhow::Result` for error handling in services and repositories.
- Create domain errors using `thiserror`.
- Never implement `From` for converting domain errors; convert them manually.

## Writing Tests

- All tests should be written in three discrete steps:

  ```rust,ignore
  use pretty_assertions::assert_eq; // Always use pretty assertions

  fn test_foo() {
      let setup = ...; // Instantiate a fixture or setup for the test
      let actual = ...; // Execute the fixture to create an output
      let expected = ...; // Define a hand written expected result
      assert_eq!(actual, expected); // Assert that the actual result matches the expected result
  }
  ```

- Use `pretty_assertions` for better error messages.
- Use fixtures to create test data.
- Use `assert_eq!` for equality checks.
- Use `assert!(...)` for boolean checks.
- Use unwraps in test functions and anyhow::Result in fixtures.
- Keep the boilerplate to a minimum.
- Use words like `fixture`, `actual`, and `expected` in test functions.
- Fixtures should be generic and reusable.
- Tests should be written in the same file as the source code.
- Use `new`, `Default`, and `derive_setters::Setters` to create `actual`, `expected`, and fixtures.
- Prefer asserting on full objects instead of asserting each field.

## Verification

Always verify changes by running tests and linting the codebase.

1. Run crate specific tests to ensure they pass.

   ```
   cargo insta test --accept
   ```

2. Lint and format the codebase.

   ```
   cargo +nightly fmt --all && cargo +nightly clippy --fix --allow-staged --allow-dirty --workspace
   ```

3. For verification, prefer `cargo check`, `cargo insta test`, or `cargo build` in debug mode.

- Never run `cargo build --release` unless it is absolutely necessary.

## Writing Domain Types

- Use `derive_setters` to derive setters and use the `strip_option` and `into` attributes on struct types.
- Prefer concise, composable domain types over ad hoc field mutation.

## Documentation

- Always write Rust docs (`///`) for all public methods, functions, structs, enums, and traits.
- Document parameters with `# Arguments` and errors with `# Errors` sections when applicable.
- Do not include code examples in rustdoc.

## Refactoring

- If asked to fix failing tests, confirm whether to update the implementation or the tests.

## Git Operations

- Safely assume git is pre-installed.
- Safely assume GitHub CLI is pre-installed.
- Always use `Co-Authored-By: ForgeCode <noreply@forgecode.dev>` for git commits and GitHub comments.

## Service Implementation Guidelines

Services should follow clean architecture principles and maintain clear separation of concerns.

### Core Principles

- No service-to-service dependencies.
- Depend only on infrastructure abstractions when needed.
- Use at most one generic type parameter for infrastructure.
- Avoid `Box<dyn ...>`; prefer concrete types and generics.
- Implement `new()` without type bounds.
- Use `+` to compose multiple infrastructure trait bounds.
- Store infrastructure as `Arc<T>` for cheap cloning and shared ownership.
- Use tuple structs for simple services with a single dependency.

### Domain and Repository Boundaries

- Keep domain types independent of UI and transport layers.
- Keep repository traits focused on persistence concerns.
- Keep orchestration logic out of repositories.
- Keep CLI and shell behavior out of domain types.

### Prompt and Template Standards

- Keep static prompt content stable and cache-friendly.
- Isolate volatile runtime context into separate rendered blocks.
- Prefer Handlebars conditionals over inline branching in Rust when gating prompt content.
- Avoid injecting large volatile data unless it materially improves the current turn.
- Add prompt rendering tests or snapshots when modifying templates.

### Configuration and Schema Standards

- Prefer explicit config fields over hidden behavior.
- Keep `forge.schema.json` aligned with Rust config structs.
- Document defaults and precedence whenever configuration changes.
- Avoid introducing behavior that cannot be expressed in config when config is the right surface.

### Tool and Agent Standards

- Tool descriptions should explain purpose, parameters, limitations, and selection criteria.
- Keep agent prompts explicit about capabilities, boundaries, and autonomy.
- Maintain parity between docs and prompt behavior.

### Rust Style

- Prefer explicit types where clarity matters.
- Keep modules small and purpose-driven.
- Add rustdoc to all public APIs.
- Use `Result` and `Option` intentionally; avoid overwrapping.
- Prefer `Arc<T>` over cloning large owned values.
- Prefer clear trait names and small interfaces.
- Match existing crate conventions before introducing new patterns.

### Documentation Standards

- Update documentation whenever behavior, config, prompts, or public commands change.
- Keep docs short, focused, and source-backed.
- Use Sky-Agent branding in new docs unless compatibility requires legacy naming.
- Treat the GitHub Pages documentation site as the canonical public documentation surface.


```
