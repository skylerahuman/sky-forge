# ForgeCode Upgrade Roadmap

## Objective

Elevate ForgeCode into a fully capable, low-noise AI pair programmer by addressing four gaps in priority order: (1) eliminate verbose always-on file work tree injection, (2) add persistent out-of-the-box memory, (3) establish a clear autonomy contract, (4) fix the documentation. Extensibility improvements follow as Phase 2.

---

## Phase 1 — File Work Tree Verbosity (Lowest Hanging Fruit)

**Problem:** `<system_information>` with `file_list` and `workspace_extensions` is injected into every LLM call unconditionally. Source: `crates/forge_app/src/system_prompt.rs:116-129` + `templates/forge-partial-system-info.md`. This adds noise and token cost on every turn, and is redundant when semantic search is active.

**Approach:** Make the file list and extension stats conditional — suppress them when the workspace semantic index is active (since `sem_search` makes the raw listing redundant), and add a `forge.yaml` config flag as an opt-in override.

### Implementation Plan

- [ ] Task 1. Add a `show_file_list` boolean field (default: `true`) to `ForgeConfig` in `crates/forge_domain/src/forge_config.rs`. Add a `workspace_indexed` boolean field to `SystemContext` in `crates/forge_domain/src/system_context.rs:87-142`.
- [ ] Task 2. In `crates/forge_app/src/system_prompt.rs`, query whether the current workspace has been indexed (via `WorkspaceRepository` or equivalent status check) and populate `SystemContext::workspace_indexed` accordingly.
- [ ] Task 3. Update `templates/forge-partial-system-info.md` to wrap the `<file_list>` and `<workspace_extensions>` blocks in a Handlebars conditional: only render them when `show_file_list` is true AND `workspace_indexed` is false. This requires no change to agent definitions — the template gate handles it transparently.
- [ ] Task 4. Expose `show_file_list` in `forge.yaml` schema (`forge.schema.json`) so users can force-enable or force-disable it regardless of index state.
- [ ] Task 5. Update the agent system prompt templates (`crates/forge_repo/src/agents/forge.md`, `sage.md`, `muse.md`) to add a Handlebars conditional block — when `workspace_indexed` is true, add a note instructing the agent to prefer `sem_search` over file list scanning.

### Verification Criteria

- A session with `:sync` completed should produce a system prompt with no `<file_list>` or `<workspace_extensions>` blocks.
- A session without `:sync` should produce the full `<system_information>` block as before.
- Setting `show_file_list: true` in `forge.yaml` should force the block to appear even when indexed.
- Setting `show_file_list: false` should suppress it even when not indexed.
- Token count per turn should decrease measurably for indexed workspaces.

### Potential Risks and Mitigations

1. **Agents lose orientation in un-indexed repos** — Mitigation: default remains `true` when not indexed, so behavior is unchanged for new users who haven't run `:sync`.
2. **Workspace index status check adds latency** — Mitigation: the status check is a lightweight DB read already used by `:workspace-status`; cache it per session.

### Alternative Approaches

1. **Always suppress, on-demand tool** — Remove the file list from the system prompt entirely and expose it as a `list_directory` tool agents call on demand. Higher impact but risks agents missing context they currently rely on passively.
2. **Reduce depth only** — Keep the list but limit to directories only (strip files). Lower improvement than the conditional approach.

---

## Phase 2 — Out-of-the-Box Memory

**Problem:** No general-purpose memory primitive. `~/.forge/memory/` exists as user convention and is indexed by semantic search, but agents have no explicit instructions to read, query, or write it.

**Approach:** Add a `remember` built-in skill plus global AGENTS.md directives that instruct all agents to query `~/.forge/memory/` at session start and write learnings there. No new Rust code required for the baseline — this is a prompt/skill/config change.

### Implementation Plan

- [ ] Task 1. Create a global `~/forge/AGENTS.md` (if not already present) with directives instructing all agents to: (a) run `sem_search` against `~/.forge/memory/` at session start to load relevant user context, (b) write new persistent facts to `~/.forge/memory/` using a structured Markdown format with YAML frontmatter (`type`, `tldr`, priority fields), (c) never delete memory files without explicit user instruction.
- [ ] Task 2. Create a built-in `remember` skill at `crates/forge_embed/src/skills/remember/SKILL.md`. The skill's procedure should: accept a fact and a type (user preference, project context, reference), write a structured `.md` file to `~/.forge/memory/`, and confirm to the user. This follows the existing skill format with YAML frontmatter used by `create-skill` and `execute-plan`.
- [ ] Task 3. Update all three agent system prompts (`forge.md`, `sage.md`, `muse.md`) to add a Handlebars block (gated on `tool_names.sem_search`) instructing the agent to query `~/.forge/memory/` for user preferences and project context at the start of each conversation.
- [ ] Task 4. Add `~/.forge/memory/` initialization to `forge workspace init` — create the directory and a starter `user-preferences.md` template if neither exists, so the memory location is always ready for semantic indexing.
- [ ] Task 5. Add `~/.forge/memory/` to the default workspace sync scope so `:sync` always indexes it alongside the project directory.

### Verification Criteria

- After a user says "remember I prefer dark mode", the `remember` skill creates a file in `~/.forge/memory/` with correct frontmatter.
- On a subsequent fresh session, an agent answering a UI question references the dark mode preference without being told again.
- `forge workspace init` creates `~/.forge/memory/` if absent.
- `forge workspace sync` indexes `~/.forge/memory/` contents.

### Potential Risks and Mitigations

1. **Memory grows unbounded** — Mitigation: document a convention for archiving stale memories; add a `:memory-clean` command as a future enhancement.
2. **Agents hallucinate memory reads they didn't do** — Mitigation: the `sem_search` call is explicit in the agent prompt with output expected before the agent proceeds; reviewable in verbose mode.
3. **Privacy: `~/.forge/memory/` contains sensitive facts sent to the workspace server for indexing** — Mitigation: document this clearly; add a note in `forge workspace init` output; allow `FORGE_WORKSPACE_SERVER_URL=local` as a self-hosting path.

### Alternative Approaches

1. **Rust-layer `memory_write` / `memory_read` tool** — Implement dedicated tool calls instead of relying on existing `fs_write`/`sem_search`. Stronger guarantees but requires new Rust code and a full release cycle.
2. **SQLite key-value store** — Add a `MemoryRepository` alongside `ConversationRepository`. Most robust but highest effort.

---

## Phase 3 — Autonomy Contract

**Problem:** Agents lack a documented, user-facing contract defining what they will do autonomously versus what requires confirmation. Users cannot predict when Forge will modify files, run commands, or delete things without asking.

**Approach:** Define explicit autonomy tiers in AGENTS.md and agent system prompts, and expose them in `:info` output.

### Implementation Plan

- [ ] Task 1. Define three autonomy tiers in `~/forge/AGENTS.md` and document them in the README: **Autonomous** (agent proceeds without asking — read ops, writing new files, running read-only commands), **Confirm-first** (agent states intent and waits — deleting files, force-push, dropping DB tables, modifying CI/CD, sending messages), **Prohibited** (agent refuses — destructive operations outside project scope, credential exfiltration).
- [ ] Task 2. Update the `forge` agent system prompt (`crates/forge_repo/src/agents/forge.md`) to include an explicit "Autonomy Contract" section enumerating what it will do freely, what it will confirm first, and what it will refuse. Mirror this in `sage.md` (read-only, so "Autonomous for reads, Prohibited for writes") and `muse.md` (Prohibited for all file changes).
- [ ] Task 3. Add an `autonomy` field to `AgentDefinition` in `crates/forge_repo/src/agent_definition.rs` with values `full`, `confirm`, `readonly`. Expose this in `forge list agent` output and `:info` so users can see at a glance what the active agent's autonomy level is.
- [ ] Task 4. For the `forge` agent specifically, add a pre-flight check pattern in the system prompt: before any destructive action (file deletion, `git reset --hard`, force operations), emit a structured confirmation request and require an affirmative before proceeding.

### Verification Criteria

- `forge list agent` output includes an `autonomy` column.
- `:info` shows the active agent's autonomy tier.
- Asking the `forge` agent to delete a file triggers a confirmation prompt before execution.
- The `sage` agent refuses `fs_write` tool calls, citing its autonomy level.
- The `muse` agent refuses all code modification requests with a referral to `forge`.

### Potential Risks and Mitigations

1. **Overly aggressive confirmation breaks agentic flow** — Mitigation: keep the confirm-first list tight; only cover genuinely irreversible or high-blast-radius operations.
2. **System prompt length growth** — Mitigation: the autonomy contract is concise (10-15 lines); the Handlebars conditional approach means it only renders for agents that have the relevant tools.

### Alternative Approaches

1. **Tool-level guards in Rust** — Intercept destructive tool calls in the executor and inject confirmation logic server-side. Stronger guarantee but harder to customize per-agent.
2. **Sandbox mode as default** — Default `forge` to `--sandbox` (isolated worktree) and let users explicitly opt into direct-write mode. High safety but changes the core interaction model.

---

## Phase 4 — Documentation Overhaul

**Problem:** `docs/` has one internal style guide. README is 1,124 lines of config reference with no architecture explanation, no agent authoring guide, and no memory/skill development guide.

**Approach:** Restructure docs into purpose-specific guides. Don't expand the README — link to docs instead.

### Implementation Plan

- [ ] Task 1. Create `docs/architecture.md` covering: crate structure and responsibilities, the agent execution pipeline (event → template rendering → LLM → tool dispatch → response), the system prompt assembly pipeline (how `file_list`, `workspace_extensions`, agent body, and non-static context are composed), and the persistence layer (SQLite conversations, snapshots, workspace indexing).
- [ ] Task 2. Create `docs/agent-authoring.md` covering: the YAML frontmatter schema for custom agents, all `AgentDefinition` fields with examples, the Handlebars `tool_names` conditional pattern, user prompt template variables (`event`, `current_date`, `terminal_context`), and the override precedence (built-in → global → project-local).
- [ ] Task 3. Create `docs/skills.md` covering: the skill file format and YAML frontmatter, the built-in skills (`create-skill`, `execute-plan`, `github-pr-description`), how to write a custom skill with a worked example, and the override precedence for skills.
- [ ] Task 4. Create `docs/memory.md` covering: the `~/.forge/memory/` convention, the structured frontmatter format for memory files, how semantic search indexes and retrieves memory, the `remember` skill usage, and privacy considerations around workspace server indexing.
- [ ] Task 5. Trim README to a quickstart + feature overview, replacing the deep config reference sections with links to the relevant `docs/` files.

### Verification Criteria

- A new contributor can onboard to agent authoring using only `docs/agent-authoring.md` without reading source code.
- The architecture doc correctly describes the system prompt assembly pipeline as confirmed against source.
- README is under 400 lines after trimming.

### Potential Risks and Mitigations

1. **Docs drift from implementation** — Mitigation: add a CI step that checks referenced file paths in docs against the actual file tree.
2. **Trimming README breaks external links** — Mitigation: keep anchor IDs stable or add redirects where possible.

---

## Execution Order Summary

| Phase | Change | Effort | Impact |
|---|---|---|---|
| 1 | Suppress file work tree when workspace is indexed | Low (template + 1 config field) | High — immediate token reduction |
| 2 | Out-of-the-box memory via skill + global AGENTS.md | Low-Medium (no Rust for baseline) | High — persistent context across sessions |
| 3 | Autonomy contract in agent prompts + `AgentDefinition` | Medium (Rust field + prompt edits) | High — predictability and safety |
| 4 | Documentation overhaul | Medium (writing work) | Medium — discoverability and contributor onboarding |

Phase 1 can be executed independently. Phases 2 and 3 can proceed in parallel once Phase 1 is merged. Phase 4 should incorporate findings from Phases 1-3.
