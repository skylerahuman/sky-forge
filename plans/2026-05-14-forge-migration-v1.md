# ForgeCode Personal Fork Migration

## Objective

Migrate the ForgeCode source to a personal GitHub repository under `skyler-shuman`, fix the truncated ZSH plugin output bug, and establish a `curl | sh` install pipeline backed by GitHub Actions. The upstream LLM-filter sync strategy is documented at the end.

---

## Assumptions

- **Binary name**: Keep as `forge` — the ZSH plugin already supports `FORGE_BIN` override, and all existing muscle memory and alias chains stay intact.
- **Repo name**: `sky-forge` — short, personal, thematic. The internal binary and ZSH plugin remain `forge`. Repo can be renamed later with zero code impact.
- **Apache 2.0 compliance**: Retain `LICENSE` file and Tailcall copyright notice. Add a separate notice in the repo root crediting the upstream. Mark modified files.
- **Initial scope**: No Rust changes in Phase 1. The truncated output fix and autosuggestions fix are ZSH-only. Rust changes are deferred to later phases.
- **Install URL pattern**: `curl -fsSL https://raw.githubusercontent.com/skyler-shuman/sky-forge/main/install.sh | sh` — hosted as a raw GitHub file, no external hosting required.

---

## Phase 1 — Repo Creation and Initial Setup

### Implementation Plan

- [ ] Task 1. Create a new **public** repository at `github.com/skyler-shuman/sky-forge`. Initialize with no default README (will be replaced). Set description: "Personal AI pair programmer — based on ForgeCode (Apache 2.0)."
- [ ] Task 2. Add the current `forgecode` directory contents as the initial commit. The upstream remote `tailcallhq/forgecode` is tracked but no history is carried over — start clean from the current HEAD snapshot. This avoids pulling in upstream PR history and bounty automation.
- [ ] Task 3. Add a `UPSTREAM.md` file in the repo root stating: "This project is a derivative of [ForgeCode](https://github.com/tailcallhq/forgecode) by Tailcall, licensed under Apache 2.0. Original copyright 2025 Tailcall." This satisfies Apache 2.0 attribution without modifying `LICENSE`.
- [ ] Task 4. Create two branches: `main` (your working branch with all custom code) and `upstream-snapshot` (a bare mirror of tailcallhq/forgecode HEAD, updated manually). `upstream-snapshot` is never merged into `main` — it exists only as a reference point for the LLM-filter diff workflow.
- [ ] Task 5. Remove or stub out the following Tailcall-specific automation from `.github/`:
  - `workflows/bounty.yml` — delete entirely
  - `workflows/stale.yml` — delete or disable (personal repo doesn't need auto-stale)
  - `scripts/bounty/` — delete entirely
  - `.github/contribution.md` — replace with a personal note
  Keep: `ci.yml`, `release.yml`, `release-drafter.yml`, `labels.yml`, `autofix.yml`, `dependabot.yml`
- [ ] Task 6. Remove references to `antinomyhq/npm-code-forge`, `antinomyhq/npm-forgecode`, and `antinomyhq/homebrew-code-forge` from `release.yml` — strip the `npm_release` and `homebrew_release` jobs entirely. The release pipeline will produce only GitHub Release binary uploads.

---

## Phase 2 — Truncated Output Bug Fix

Two separate bugs from the same class (ZLE widget coordination). Both are ZSH-only changes — no Rust required.

### Bug A: Output overwritten after `_forge_exec_interactive` (root cause of the cut-off)

**File**: `shell-plugin/lib/helpers.zsh:108-115`

**Root cause** (from issue #3144): After `_forge_exec_interactive` writes forge output directly to `/dev/tty`, `zle reset-prompt` redraws the prompt at ZLE's tracked cursor position — which is **above** the forge output. The prompt overwrites the response, making it appear cut off or vanished.

**Fix**: Replace `zle reset-prompt` with `zle .accept-line` in `_forge_reset()`. `zle .accept-line` submits the now-empty buffer normally, prints a newline, and draws a fresh prompt at the **bottom** of the output.

- [ ] Task 7. In `shell-plugin/lib/helpers.zsh`, replace the body of `_forge_reset()` (lines 108-115):

  Change from:
  ```zsh
  function _forge_reset() {
    BUFFER=""
    CURSOR=0
    zle -I
    zle reset-prompt
  }
  ```

  Change to:
  ```zsh
  function _forge_reset() {
    BUFFER=""
    CURSOR=0
    zle .accept-line
  }
  ```

  The `zle -I` call is also removed — it was invalidating the line before `reset-prompt`, which contributed to the overwrite artifact and is not needed when using `.accept-line`.

### Bug B: zsh-autosuggestions ghost text not cleared on Enter (issue #3243, open PR #3244)

**File**: `shell-plugin/lib/bindings.zsh`

**Root cause**: `forge-accept-line` is defined and bound after `zsh-autosuggestions` has already run its `_zsh_autosuggest_bind_widgets` pass, so the widget is never registered in `ZSH_AUTOSUGGEST_CLEAR_WIDGETS`. The ghost text `POSTDISPLAY` is never cleared, leaving stale inline suggestions overlapping the output.

- [ ] Task 8. At the end of `shell-plugin/lib/bindings.zsh` (after the existing `bindkey` lines), add the following block:

  ```zsh
  # Coordinate with zsh-autosuggestions: register forge-accept-line so POSTDISPLAY
  # is cleared when Enter is pressed, preventing ghost text from overlapping output.
  if typeset -f _zsh_autosuggest_bind_widgets >/dev/null 2>&1; then
    if [[ ${ZSH_AUTOSUGGEST_CLEAR_WIDGETS[(Ie)forge-accept-line]} -eq 0 ]]; then
      ZSH_AUTOSUGGEST_CLEAR_WIDGETS+=(forge-accept-line)
      _zsh_autosuggest_bind_widgets
    fi
  fi
  ```

  This is a no-op when `zsh-autosuggestions` is not loaded.

### Verification Criteria

- After a `:` prompt completes, the full AI response is visible and not overwritten by the prompt.
- With `zsh-autosuggestions` active, pressing Enter clears gray ghost text before the command executes.
- Behavior is unchanged when `zsh-autosuggestions` is not installed.
- No regression in non-`:command` buffer (normal `accept-line` path) behavior.

---

## Phase 3 — CI/Release Pipeline Adaptation

### What needs to change

The existing `release.yml` and `ci.yml` reference Tailcall secrets (`POSTHOG_API_SECRET`, `NPM_ACCESS`, `NPM_TOKEN`, `HOMEBREW_ACCESS`) and external repos. These must be removed or replaced.

**Note**: Both workflow files carry a `DO NOT EDIT BY HAND` warning — they are generated from `build.rs` in `crates/forge_ci/`. For the personal fork, the simplest approach is to convert these to hand-maintained YAML files and remove the `gh-workflows` generation chain, since we don't need Tailcall's multi-channel release infrastructure.

### Implementation Plan

- [ ] Task 9. Remove the `DO NOT EDIT` header comment from `ci.yml` and `release.yml` — they are now hand-maintained in this fork.
- [ ] Task 10. In `ci.yml`, strip the `build` job's `cargo-llvm-cov` step and replace it with a simpler `cargo test --workspace` — coverage tooling is Tailcall infrastructure that requires their secrets. Keep the `zsh_rprompt_perf` job as-is.
- [ ] Task 11. In `ci.yml`, remove the `POSTHOG_API_SECRET` env var from the `build_release` and `build_release_pr` jobs. The PostHog telemetry is compiled out when `POSTHOG_API_SECRET` is empty — the binary still builds correctly.
- [ ] Task 12. In `release.yml`, delete the `npm_release` and `homebrew_release` jobs entirely. The workflow should contain only `build_release`.
- [ ] Task 13. In `release.yml`, remove `POSTHOG_API_SECRET` from the `Build Binary` step env block.
- [ ] Task 14. Add a GitHub Actions secret `GITHUB_TOKEN` is already provided automatically — no additional secrets are required for the stripped-down pipeline.
- [ ] Task 15. Update the release trigger to also fire on `workflow_dispatch` (manual trigger) in addition to published releases — allows triggering a release build without going through GitHub's UI release flow.

### Verification Criteria

- Pushing to `main` triggers CI and produces a draft release with binaries attached for all 9 targets.
- Publishing a GitHub Release triggers `release.yml` and uploads binaries.
- No workflow fails due to missing secrets.
- The compiled binary runs and responds to `forge --version`.

---

## Phase 4 — Install Script

The install script must: detect OS and architecture, download the appropriate binary from GitHub Releases, place it in `~/.local/bin` (or `/usr/local/bin` with sudo fallback), and print next steps.

### Implementation Plan

- [ ] Task 16. Create `install.sh` in the repo root. The script should:
  1. Detect OS (`uname -s`): `Linux`, `Darwin`, `Windows_NT`
  2. Detect arch (`uname -m`): `x86_64`, `aarch64`/`arm64`
  3. Map to binary name pattern: `forge-{arch}-{os}` (e.g., `forge-x86_64-unknown-linux-musl` for Linux x86_64 musl)
  4. Determine the latest release tag via GitHub API: `https://api.github.com/repos/skyler-shuman/sky-forge/releases/latest`
  5. Construct the download URL: `https://github.com/skyler-shuman/sky-forge/releases/download/{tag}/{binary_name}`
  6. Download with `curl -fSL` to a temp file, verify it is non-empty
  7. Install to `$HOME/.local/bin/forge` (create directory if absent), set executable bit
  8. Print: "forge installed to ~/.local/bin/forge — run 'forge setup' to install the ZSH plugin"
  9. Detect if `~/.local/bin` is not in `PATH` and print a warning with the fix command
- [ ] Task 17. Default to the `musl` Linux variant (statically linked, works on all Linux distributions without glibc version constraints). Add a `--gnu` flag for users who explicitly want the GNU variant.
- [ ] Task 18. Add a `--version <tag>` flag to `install.sh` to allow pinning to a specific release (useful for the upstream-filter sync workflow described below).
- [ ] Task 19. Verify the install script works end-to-end in a clean environment by testing against the first real release binary.

### Invocation

```bash
curl -fsSL https://raw.githubusercontent.com/skyler-shuman/sky-forge/main/install.sh | sh
```

Or with pinning:
```bash
curl -fsSL https://raw.githubusercontent.com/skyler-shuman/sky-forge/main/install.sh | sh -s -- --version v1.0.0
```

---

## Upstream Sync Strategy (Reference)

This is not a Phase — it is the ongoing maintenance workflow after the fork is live.

**When to sync**: Weekly, or before starting any new feature work.

**Workflow**:

1. `git fetch` the `upstream-snapshot` branch against `tailcallhq/forgecode main`
2. Run `git log upstream-snapshot ^main --oneline` to get the delta commits
3. Feed each commit's diff + message to an LLM with this prompt template:

   > Classify this ForgeCode upstream commit. Return JSON: `{"category": "bug_fix|feature|refactor|chore|security", "worth_incorporating": true|false, "touches_custom_files": true|false, "summary": "...", "risk": "low|medium|high"}`. Flag anything touching auth, HTTP, dependency versions, or shell plugin files as high risk regardless of category.

4. Cherry-pick commits classified as `worth_incorporating: true` onto `main` one at a time
5. Commit any cherry-picks with a `upstream:` prefix in the message for traceability
6. Skip commits classified `worth_incorporating: false` — update `upstream-snapshot` regardless

**Custom commit prefix convention** (for clean LLM classification of your own commits vs upstream):
- `custom:` — personal modification
- `fix:` — bug fix (including the truncated output fix)
- `upstream:` — cherry-picked from upstream

---

## Execution Order

| Phase | Scope | Blocking dependencies |
|---|---|---|
| 1 — Repo setup | GitHub, CI/CD config files | None — do first |
| 2 — ZSH bug fix | 2 ZSH files only | Phase 1 must be complete (need the repo) |
| 3 — Pipeline adaptation | CI/CD YAML files | Phase 1 |
| 4 — Install script | New `install.sh` | Phase 3 (needs a release to download) |

Phases 2 and 3 can be done in parallel once Phase 1 is complete.
