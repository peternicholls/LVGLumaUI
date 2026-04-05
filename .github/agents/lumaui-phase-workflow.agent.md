---
description: Use the LumaUI phase-gated workflow for ratified-slice planning, decision-brief gating, and narrow compiler-stage work
---

# LumaUI Phase Workflow

Use this workflow for repository-shaped LumaUI work. Keep the slice narrow and preserve strict stage boundaries.

## Use This Agent For

- choosing the next task from the active phase packet
- deciding whether a request is docs-only, decision-only, review-only, or allowed to change code
- deciding whether a change needs a decision brief before implementation
- keeping `docs/`, `examples/`, `tests/`, snapshots, and code synchronized
- preserving strict boundaries across `parser`, `semantic`, `ir`, `backend/lvgl_c`, and `cli`

## Required Reading Order

Read these before acting:

1. `README.md`
2. `docs/TASKS.md`
3. `docs/NEXT_STEPS.md`
4. `docs/ARCHITECTURE.md`
5. `docs/LANGUAGE_SPEC.md`
6. `docs/LVGL_MAPPING.md`
7. the active feature packet under `specs/<feature-id>/`

## Execution Rules

1. State the active phase and exit gate before implementation.
2. Confirm the construct is already ratified; if not, stop at documentation or decision work.
3. If the task changes language surface, LVGL mapping policy, generated ownership conventions, diagnostics policy, or shared downstream contracts, write or update a decision brief under `specs/<feature-id>/decisions/` using `docs/DECISION_BRIEF_TEMPLATE.md` before implementation.
4. For code-bearing work, add or update the failing test, fixture, snapshot, or CLI assertion first.
5. Make the smallest stage-local implementation that satisfies the task.
6. Update docs, examples, fixtures, tests, and snapshots together whenever behavior changes.
7. If a change forces simultaneous redesign across parser, semantic, IR, and backend, reduce the slice instead of pushing through it.

## Verification Defaults

- Use `scripts/lumaui-phase-check.sh` for the standard verification bundle.
- Use `--skip-build` when the task does not touch the build path.
- Use `--require-build` once end-to-end build behavior is expected to work for the slice.

## Governance

- Use `.specify/memory/constitution.md` as the governing workflow authority.
- Treat `docs/archive/CONSTITUTION.md` as archival context only.
- In final reports, include changed files, simplifications made, and remaining risks or verification gaps.
