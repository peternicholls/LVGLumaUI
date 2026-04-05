# AGENTS.md

## Project Intent

LumaUI is a clean-room, declarative UI compiler for LVGL.

It is web-inspired, but it is not a browser, not a DOM runtime, and not a full HTML/CSS implementation.

This repository exists to build a disciplined compiler pipeline that turns authored UI source into deterministic, readable LVGL C.

## Current Phase

This repo is in the first implementation pass.

That means:

- architecture is being hardened
- examples and fixtures matter
- compiler stages are being scaffolded
- the authored language is not frozen yet

Do not prematurely lock in grammar or broaden scope just because it feels natural.

## Non-Negotiable Rules

- Preserve compile-time-first architecture.
- Keep the parser, semantic layer, IR, and backend separated.
- Prefer deterministic output over clever abstractions.
- Keep generated C stable, readable, and easy to diff.
- Avoid browser semantics that do not map cleanly to LVGL.
- Treat docs, examples, and tests as first-class source material.
- Do not imitate proprietary LVGL tooling formats or terminology.

## Current Language Rule

Do not finalize the authored language grammar in this phase.

You may:

- improve provisional examples
- add lexer/parser scaffolding
- document constraints and decision criteria
- add tests around provisional parsing infrastructure

You may not:

- claim the grammar is final
- add broad CSS semantics
- introduce browser-style layout behavior
- over-design selectors, cascade, or runtime binding behavior

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

Command orchestration and user-facing diagnostics only.

Do not collapse these layers together for convenience.

## Determinism Expectations

- Sort discovered inputs.
- Preserve stable iteration order.
- Avoid unordered output from hash-based collections unless explicitly normalized.
- Keep generated symbol naming reproducible.
- Keep snapshot output formatting fixed.

## How To Approach Tasks

1. Start from `.specify/memory/constitution.md`, then the docs and current milestone in `docs/TASKS.md`.
2. Prefer the smallest change that improves the pipeline honestly.
3. If a feature would force language decisions that have not been ratified yet, document the constraint and defer the feature.
4. Update the relevant docs when behavior or scope changes.
5. Add or update fixtures and tests with every meaningful compiler change.

## Tests and Fixtures

When adding parser work:

- add parser-focused unit tests
- use small fixtures
- prefer explicit diagnostics over silent fallback

When adding semantic work:

- add validation tests
- cover both success and failure shapes

When adding backend work:

- add snapshot tests for generated C
- keep snapshots readable and intentionally ordered

Examples under `examples/` should act as golden fixtures, not marketing fluff.

## Scope Control

Defer or reject:

- JavaScript runtime concepts
- DOM mutation models
- full CSS selector engines
- dynamic template evaluation on-device
- preview/editor features before compiler maturity
- asset-pipeline complexity that is not yet specified

## Definition of a Good Change

A good change in this repo:

- sharpens a stage boundary
- improves determinism
- makes diagnostics clearer
- strengthens docs and fixtures
- adds tests that lock in intended behavior

A bad change:

- smuggles in browser behavior
- over-promises unimplemented language features
- hides uncertainty instead of isolating it
- couples unrelated compiler stages together
