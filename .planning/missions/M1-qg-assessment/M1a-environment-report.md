# M1a — Environment & Pipeline Assessment (physics-tree)

**Mission:** M1 (quantum-gravity-programme) — sub-mission M1a
**Agent:** general-purpose, model sonnet
**Date:** 2026-08-15
**Scope:** Build health, run path, authoring-pipeline static assessment, revival blockers. Read-only + repo-local builds only. No commits, no pushes, no changes outside `.planning/missions/M1-qg-assessment/` (`target/` excepted).

---

## 1. Build health — VERDICT: CLEAN

`cargo check --workspace` (5-crate workspace: domain, db, server, app, simulation) — **exit 0, no errors.** Only pre-existing warnings: ~36 dead-code warnings in `app` (unused fetch helpers, likely superseded by newer data-loading pattern) and 5 dead-field warnings in `simulation` (`canvas_width`/`canvas_height` never read across all 5 simulations). None are new; all are cosmetic.

`cargo build --workspace` — **exit 0**, 2.17s (mostly incremental off the check). Native binaries build cleanly including `target/debug/{validate,ingest,server}`, which already exist and are current.

Toolchain: `rustc 1.94.0`, `cargo 1.94.0`, `rust-toolchain.toml` pins `stable` + `wasm32-unknown-unknown` — both installed via rustup. `cargo-leptos 0.3.5` is installed and callable (`cargo leptos --version`).

No `DATABASE_URL` was needed at compile time — confirmed the codebase uses **zero** compile-time-checked `sqlx::query!`/`query_as!` macros (a deliberate v1.1 decision recorded in STATE.md: "graph_repo.rs switched to dynamic sqlx::query API... query_as! macro fails to compile when PhysicsNode gains a new field without DATABASE_URL"). This means the build is DB-agnostic and will stay green even if Postgres is down.

**Frontend deps:** `package.json` declares only npm dependencies (sigma, graphology, katex, mathjs) and one devDependency (esbuild) — **no `scripts` block**. `node_modules/` is already populated and `npm install --dry-run` reports "up to date in 93ms" — no drift. The JS bundles consumed by the app (`public/js/sigma_bundle.js`, `toc_bundle.js`, `mathjs_bundle.js`) are pre-built, checked-in artifacts; there is no documented rebuild script (no justfile, no npm script, no build.rs hook) — rebuilding them (if ever needed) would require manually reconstructing whatever ad hoc `esbuild` invocation produced them, which is undocumented. Not a blocker for running the app as-is.

## 2. Run path

**No README.md, no justfile, no scripts/ directory exist in this repo.** The run sequence below is reconstructed from `Cargo.toml`'s `[[workspace.metadata.leptos]]` block, `crates/server/src/main.rs`, `.env`, and `Dockerfile` — not documented anywhere as a single source of truth. This absence is itself worth fixing on revival (see §4).

**Dev run sequence (reconstructed, not executed by this assessment):**
```
# 1. Postgres must be reachable at DATABASE_URL (see below — already true today)
# 2. From repo root:
cargo leptos watch
# serves at http://127.0.0.1:3001 (site-addr in Cargo.toml; NOT 3000 — Dockerfile/prod
# uses 3000, several stale port refs to 3030 exist in .claude/settings.local.json from
# old debug sessions — 3001 is the authoritative dev value)
```
`crates/server/src/main.rs` loads `.env` via `dotenvy`, requires `DATABASE_URL` (panics if unset), connects via `db::create_pool`, and separately runs `tower_sessions_sqlx_store::PostgresStore::migrate()` at startup for its own session table (not tracked in `migrations/`). Schema migrations themselves are **not** auto-run by the server — they'd need `sqlx migrate run --source migrations` (as CI does) if the DB weren't already current.

**Postgres status — checked read-only, DO NOT START/CREATE performed:**
- `pg_isready` → accepting connections (`/run/postgresql:5432`)
- `systemctl status postgresql` → **already active**, running since 14:34:21 today (system service, not a project-specific instance)
- `.env` → `DATABASE_URL=postgres://postgres:postgres@localhost:5432/physics_tree`
- `psql -l` confirms the `physics_tree` database **already exists**
- `\dt` shows all 10 expected tables present (`nodes`, `node_phases`, `edges`, `users`, `progress`, `xp_events`, `user_streaks`, `engagement_events`, `user_phase_progress`, `_sqlx_migrations`)
- `_sqlx_migrations` shows **all 11/11 migrations applied**, matching 1:1 with the 11 files in `migrations/` (versions 20260318000001 → 20260329000001) — nothing pending
- Data present: 34 rows in `nodes`, 22 in `node_phases`, 2 in `users` — this is a live, populated dev database, not an empty shell

**Bottom line: the DB prerequisite that would normally block a revival session is already satisfied on this machine.** No `sqlx migrate run` needed, no DB creation needed. (Caveat: this state is specific to this machine/session — collation-version warnings on every psql connection, `"database was created using collation version 2.43, but the OS provides 2.44"`, are cosmetic but should be fixed once with `ALTER DATABASE ... REFRESH COLLATION VERSION` to stop the noise.)

**Prod/Docker path** (`Dockerfile`, not exercised): two-stage build, `cargo leptos build --release` in a `rust:1.85-bookworm` builder, runtime on `debian:bookworm-slim`, serves on `0.0.0.0:3000`, healthcheck against `/api/health`.

## 3. Authoring pipeline (`tools/authoring/`) — STATIC ASSESSMENT ONLY, NOT EXECUTED

**Invocation contract:** `cd tools && python -m authoring {generate <spec.yaml> | preview <slug> | approve <slug> --branch <b> | gate <slug> | calibrate}`. Packaged as a flat-layout setuptools project (`pyproject.toml`, Python ≥3.12), with a committed `.venv` (uv-managed, Python 3.13.12) and `uv.lock`.

**Architecture:** 4 agents (Author, Physics Reviewer, Pedagogy Reviewer, Student Simulator) orchestrated in `pipeline.py`, each calling `claude_agent_sdk.query()` with per-agent `ClaudeAgentOptions(model=...)`. Author writes files directly (`allowed_tools=["Write","Read"]`, `permission_mode="acceptEdits"`) into a staging dir; reviewers/simulator are read-only (`allowed_tools=[]`). Flow per `run_generate`: Author → {Physics Reviewer ∥ Pedagogy Reviewer} → Student Simulator → if FAIL and `revision_round < max_revision_rounds` (config: **1**) → Author revision → repeat once → stop. Max 8 model calls per node (1 author + 3 review-round agents ×2 rounds), minimum 4 if it passes first try. Two independent, human-gated CLI steps (`preview`, `approve`) stand between any AI output and `content/` — `approve` is explicitly "the ONLY code path that writes to content/" per its own docstring, confirmed by inspection.

**Credentials:** No `ANTHROPIC_API_KEY` or `api_key` handling anywhere in the package (grepped clean) — entirely delegated to `claude_agent_sdk`'s default credential resolution (env var or authenticated `claude` CLI session). System has `/usr/bin/claude` (Claude Code CLI, v2.1.220) installed, but that doesn't guarantee the SDK will use it non-interactively for a scripted `query()` call — this needs verifying at actual run time, not assumed.

### Two independent hard blockers found (confirmed by static inspection, not execution)

**(a) Pinned model is fully retired, not merely stale.** `pipeline_config.yaml` pins all four agents to `claude-sonnet-4-20250514`. Web search confirms **Anthropic retired `claude-sonnet-4-20250514` on 2026-06-15** — two months before this assessment (2026-08-15) — with **no grace period**; every API call against it now errors. Recommended migration target per Anthropic's own guidance is `claude-sonnet-4-6` (Feb 2026), though the currently-live frontier model should be re-checked at revival time rather than hardcoded again. **A model bump means:** editing all 4 model strings in `pipeline_config.yaml` (config.py's `AgentConfig` defaults also hardcode the same stale ID as a fallback — same edit needed there), then **re-running Phase 13's calibration** (`python -m authoring calibrate`) against the gold set, since TPR/TNR=1.0 was measured against output from the old model and a different model's content style/quality could shift gate accuracy.

**(b) `claude-agent-sdk` is imported but never declared as a dependency, and is not installed.** `agents/author.py`, `reviewer.py`, `student.py`, and `pipeline.py` all `from claude_agent_sdk import query, ClaudeAgentOptions`. `pyproject.toml` declares only `pyyaml>=6.0` (+ `pytest` under `[dev]`); `authoring.egg-info/requires.txt` confirms the same; `uv.lock` has zero references to `claude`; and `.venv/bin/python -c "import claude_agent_sdk"` **fails with `ModuleNotFoundError`**. The committed `.venv` was never capable of running `generate` — this gap predates the retirement in (a) and looks like it was never caught because `generate` was likely last exercised in an environment with a manually pip-installed SDK outside the tracked lockfile (or never actually run at all — Phase 12/13 commit history shows the pipeline scaffold, gate module, and gold-fixture calibration landing, but no evidence in `.planning/` of a real `generate` invocation succeeding). Fix: add `claude-agent-sdk` (confirmed real PyPI package, requires Python ≥3.10, ships its own CLI in the wheel) to `pyproject.toml` and `uv sync`.

**Not blocked:** the `gate` and `calibrate` subcommands (`quality_gate.py`, `calibrate.py`) do **not** import `claude_agent_sdk` — they're pure-Python/pyyaml, wrapping the Rust `validate` binary and reading the pre-existing gold fixtures. These two subcommands are plausibly runnable today without any API dependency at all (not verified by execution, per the no-spend constraint, but the import graph is clean).

**Cost estimate (illustrative, not measured):** Based on the kinematics pilot node's actual size (3,386 words across 7 phases + node.yaml ≈ 4,500–5,000 output tokens) and prompt sizes (author system prompt 502 lines ≈ 6,000 tokens; reviewer/simulator prompts 185–237 lines ≈ 2,300–2,500 tokens each), a full no-revision generation round is roughly 28–30k input / 9–10k output tokens across the 4 calls; a worst-case single-revision round roughly doubles that (≈60k input / ≈20k output). At Sonnet-4-class launch pricing this lands in the tens-of-cents-per-node range — genuinely cheap — but this figure uses stale pricing assumptions for a now-retired model and should be re-derived against whatever model and current published rate is actually configured before Phase 14 runs at any volume.

## 4. Revival blockers

Ranked by whether they'd stop a working session outright vs. just create friction:

1. **[RESOLVED since `.gardener-notes.md` was written]** The dangling `.claude/agents/wiki-librarian.md` symlink (gardener finding F-01) and the tracked-but-should-be-gitignored `.gardener-notes.md` / `physics_tree.db` (F-03) are **already fixed** — confirmed by `git log` (`7752373 chore: drop the dead wiki-librarian symlink, ignore custodial output`) and by direct inspection: `.claude/agents/` now holds only 33 valid `gsd-*` symlinks (all resolve), and `.gitignore` now lists both `.gardener-notes.md` and `physics_tree.db`, with `git ls-files` confirming neither is tracked. `.gardener-notes.md` itself is otherwise stale (last generated 2026-07-30, references a state that's since moved on) — worth a fresh `/gardener --repo` pass rather than trusting it as current.

2. **[Real, hard blocker for AI content authoring specifically]** The two issues in §3 — retired model ID and missing SDK dependency — must both be fixed before `python -m authoring generate` can run at all. This does **not** block using physics-tree as a learning platform on the existing (hand-authored kinematics) content; it only blocks Phase 14 ("AI Pilot Nodes") and any future AI-assisted authoring.

3. **[Documentation gap, not a code blocker]** No README/justfile/scripts anywhere means the run sequence in §2 had to be reverse-engineered from `Cargo.toml` leptos metadata + `main.rs` + `.env`. `.gardener-notes.md`'s own standing recommendation (F-04 proposal) is to defer adding a `CLAUDE.md`/README until a revival signal — this assessment IS that signal. A minimal README or justfile with the `cargo leptos watch` sequence, the DATABASE_URL contract, and the authoring-pipeline invocation would remove real friction for any future session (agentic or human).

4. **[Minor, cosmetic]** Postgres collation-version mismatch warning on every connection (OS collation library upgraded since DB creation) — harmless today, fixable with one `ALTER DATABASE ... REFRESH COLLATION VERSION` per DB, not urgent.

5. **[Minor, planning-doc drift]** `.planning/ROADMAP.md`'s checklist marks `- [ ] Phase 9: Database & Ingest` unchecked in the phase list, but the same file's Progress table and `.planning/STATE.md` both say Phase 9 is complete (all 3 of its plans show `[x]`), and Phase 13 (which depends on 9 and 12) is verified complete. Cosmetic doc-sync bug, not a functional blocker — flagging so a revival session doesn't get confused re-litigating whether Phase 9 is actually done (it is).

6. **Phase 14 definition** (for reference, unstarted, `TBD` plans): goal is "the full authoring pipeline is validated end-to-end with 3-5 pilot nodes spanning EQF 2, EQF 3-4, and EQF 5, with at least 2 produced via the AI pipeline and approved through human review." Depends on Phase 11 (done) + Phase 13 (done). Requirements PILOT-02/03/04 all currently `Pending`. This is exactly the phase blocked by item 2 above — everything else it depends on is already green.

## 5. Summary verdict

The **application itself is closer to runnable today than the "idle since April" framing suggested**: build is clean, Postgres is already up with all migrations applied and real data loaded, frontend deps are current, and the one blocking hygiene issue recorded in the last gardener run has already been fixed. The **AI authoring pipeline is the actual blocker for Phase 14** specifically, via two compounding, independently-confirmed issues: a fully retired model ID (hard API error, not just staleness) and a genuinely missing package dependency that likely means `generate` has never successfully run in the committed environment. Neither blocks using the existing hand-authored kinematics content as a learning platform right now.
