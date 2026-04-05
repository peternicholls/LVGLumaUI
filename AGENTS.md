# AGENTS.md

## Product Name

The public product name is **Luma UI for LVGL**.
Use **LumaUI** as the short form.

## Project Intent

LumaUI is a clean-room, declarative UI compiler for LVGL.

It is web-inspired, but it is not a browser, not a DOM runtime, and not a full HTML/CSS implementation.

This repository exists to build a disciplined compiler pipeline that turns authored UI source into deterministic, readable LVGL C.

## Core Working Rules

- Preserve compile-time-first architecture.
- Keep parser, semantic, IR, backend, and CLI responsibilities clearly separated.
- Prefer deterministic output over clever abstractions.
- Keep generated C stable, readable, and easy to diff.
- Avoid browser semantics that do not map cleanly to LVGL.
- Treat docs, examples, fixtures, tests, and generated snapshots as first-class source material.
- Do not imitate proprietary LVGL tooling formats or terminology.

## Decision Workflow

Major language, architecture, mapping, and observability decisions are research-first and developer-approved.

When a task touches stage-shaping choices:

- prepare supporting material with options, pros/cons, relevant practices, implementation developments, risks, and open questions
- use `docs/DECISION_BRIEF_TEMPLATE.md` as the default format
- store feature-scoped decision briefs under `specs/<feature-id>/decisions/`
- defer final ratification until the developer explicitly signs off

Do not quietly finalize repository-shaping decisions in code alone.

## Current Phase Guidance

The repository is still in the early compiler build-out phase.

That means:

- architecture is still being hardened
- examples and fixtures matter
- stage contracts must remain explicit
- the authored language should not be widened casually

Do not broaden scope just because it feels natural.

## Architectural Boundaries

### `compiler/`

Shared infrastructure only.

### `parser/`

Syntax-facing work only.

### `semantic/`

Meaning resolution, validation, and lowering only.

### `ir/`

Backend-facing canonical model only.

### `backend/lvgl_c/`

LVGL code generation only.

### `cli/`

Command orchestration, diagnostics presentation, and operator-facing output only.

Do not collapse these layers together for convenience.

## Observability Rules

- Keep logging deterministic and stage-scoped.
- Keep diagnostics stable, actionable, and separate from progress output.
- Do not leak logging or tracing noise into generated artifacts.
- Let the CLI own operator-facing presentation of stage events.

## How To Approach Work

1. Start from the active docs, especially `README.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, and `docs/LANGUAGE_SPEC.md`.
2. Prefer the smallest honest change that improves the pipeline.
3. If a feature would force unratified language or contract decisions, document the constraint and defer the feature.
4. Update the relevant docs in the same change when behavior or support status changes.
5. Add or update tests, fixtures, or snapshots with every meaningful compiler change.

## Tests and Fixtures

When adding parser work:

- add parser-focused unit tests
- use small fixtures
- prefer explicit diagnostics over silent fallback

When adding semantic work:

- add validation tests
- cover both success and failure cases

When adding backend work:

- add snapshot tests for generated C
- keep snapshots readable and intentionally ordered

Examples under `examples/` should act as golden fixtures, not marketing fluff.

## Documentation Housekeeping

- Follow `docs/DOCUMENTATION_SCHEME.md` for file placement and naming.
- Use `README.md` as an entry point, not a dumping ground for every policy.
- Keep repository-wide policy in `docs/` and feature work in `specs/<feature-id>/`.
- Archive historical bootstrap material instead of leaving it in the active docs set.

## Scope Control

Defer or reject:

- JavaScript runtime concepts
- DOM mutation models
- full CSS selector engines
- dynamic template evaluation on-device
- preview/editor-first features before compiler maturity
- asset-pipeline complexity that is not yet specified

## Definition of a Good Change

A good change in this repo:

- sharpens a stage boundary
- improves determinism
- makes diagnostics clearer
- strengthens docs, fixtures, and tests
- adds evidence for intended behavior

A bad change:

- smuggles in browser behavior
- over-promises unimplemented language features
- hides uncertainty instead of isolating it
- couples unrelated compiler stages together