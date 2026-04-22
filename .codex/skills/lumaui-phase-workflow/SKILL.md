---
name: lumaui-phase-workflow
description: Use for LumaUI phase-gated compiler work, ratified-slice planning, decision-brief gating, and keeping docs, examples, fixtures, tests, and snapshots aligned. Trigger when choosing the next LumaUI task, changing language or LVGL mapping policy, or deciding whether work should stop at docs versus proceed into implementation.
---

# LumaUI Phase Workflow

Use this skill for repository-shaped LumaUI work. It is for the judgment-heavy part of the workflow, not for blindly replaying shell commands.

## Use This Skill For

- choosing the next task from the active phase packet
- deciding whether a request is docs-only, review-only, or allowed to change code
- deciding whether a change needs a decision brief before implementation
- keeping `docs/`, `examples/`, `tests/`, snapshots, and code synchronized
- preserving strict stage boundaries across `parser`, `semantic`, `ir`, `backend/lvgl_c`, and `cli`

## Keep In The Agent Loop

These steps need repository judgment and should remain skill-driven:

1. Read the active workflow docs before acting:
   - `README.md`
   - `docs/TASKS.md`
   - `docs/NEXT_STEPS.md`
   - `docs/ARCHITECTURE.md`
   - `docs/LANGUAGE_SPEC.md`
   - `docs/LVGL_MAPPING.md`
   - the active feature packet under `specs/<feature-id>/`
2. State the active phase and exit gate before implementation.
3. Decide whether the task is:
   - ratification or wording only
   - decision material only
   - shared-contract work
   - code-bearing story work
   - verification or errata only
4. Stop at docs and decision material when the requested change is not yet ratified.
5. Keep the slice narrow. If a change forces simultaneous redesign across parser, semantic, IR, and backend, reduce the slice instead of pushing through it.
6. Update docs, examples, fixtures, tests, and snapshots together whenever behavior changes.

## Prefer Automation For

Use scripts for mechanical repetition:

- routine verification commands
- deterministic repo-health checks
- predictable scaffolding from stable templates

Use `scripts/lumaui-phase-check.sh` instead of reconstructing the standard verification bundle by hand.

## Decision Brief Triggers

Write or update a decision brief under `specs/<feature-id>/decisions/` using `docs/DECISION_BRIEF_TEMPLATE.md` before implementation when the task does any of the following:

- changes the ratified language surface
- adds or widens widget, property, selector, layout, binding, or event support
- changes LVGL mapping policy or generated-file ownership conventions
- changes diagnostics or observability policy in a way that shapes operator-facing command behavior
- changes shared contracts that downstream stages will rely on

Use `.specify/memory/constitution.md` as the governing workflow authority. Treat `docs/archive/CONSTITUTION.md` as archival context only.

## Default Execution Sequence

1. Confirm the task fits the current phase and exit gate.
2. Confirm the construct is already ratified; if not, stop at documentation or decision work.
3. For code-bearing work, add or update the failing test, fixture, snapshot, or CLI assertion first.
4. Make the smallest stage-local implementation that satisfies the task.
5. Run `scripts/lumaui-phase-check.sh` with the right flags, then run any task-specific verification not covered by the script.
6. In the final report, always include:
   - changed files
   - simplifications made
   - remaining risks or verification gaps

## Verification Defaults

- Docs-only work: run targeted consistency checks and any relevant markdown diff review.
- Parser, semantic, IR, backend, or CLI work: run the phase check script plus targeted crate tests.
- Use `--skip-build` when the task does not touch the build path.
- Use `--require-build` once end-to-end build behavior is expected to work for the slice.
