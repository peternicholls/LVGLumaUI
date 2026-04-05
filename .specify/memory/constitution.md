<!--
Sync Impact Report
Version change: unversioned template -> 1.0.0
Modified principles:
- Placeholder Principle 1 -> I. Compile-Time First Pipeline
- Placeholder Principle 2 -> II. Embedded-Safe LVGL Mapping
- Placeholder Principle 3 -> III. Deterministic, Readable Artifacts
- Placeholder Principle 4 -> IV. Strict Layer Isolation
- Placeholder Principle 5 -> V. Ratified Narrow Slices with Docs, Tests, and Fixtures
Added sections:
- Operational Constraints
- Workflow & Phase Gates
Removed sections:
- None
Templates requiring updates:
- ✅ .specify/templates/plan-template.md
- ✅ .specify/templates/spec-template.md
- ✅ .specify/templates/tasks-template.md
- ✅ docs/CONSTITUTION.md
- ✅ docs/intent.md
Follow-up TODOs:
- None
-->
# LVGLuamUI Constitution

## Core Principles

### I. Compile-Time First Pipeline
LumaUI MUST remain a staged compiler whose production path is authored source ->
parser -> semantic analysis -> IR -> generated LVGL C -> firmware build. No
feature may introduce device-runtime interpretation, browser-style mutation, or
hidden execution semantics. This preserves the repository's embedded-first
mission and prevents scope creep into runtime-engine work.

### II. Embedded-Safe LVGL Mapping
Every supported language construct MUST map cleanly to a named LVGL 9.x
primitive or API family before implementation is approved. Features that rely on
browser semantics, ambiguous backend inference, or LVGL 8.x compatibility MUST
be deferred until they are explicitly ratified in the project docs. This keeps
the language surface conservative, auditable, and honest about platform limits.

### III. Deterministic, Readable Artifacts
Equivalent inputs and compiler versions MUST produce stable, reviewable outputs,
including diagnostics, normalized models, and generated C. Generated code is a
deliverable, not an opaque artifact: symbol naming, formatting, emission order,
and escape-hatch boundaries MUST remain predictable and human-readable. This
ensures git diffs, snapshot tests, and firmware reviews stay trustworthy.

### IV. Strict Layer Isolation
The compiler crates MUST preserve stage boundaries: `compiler/` owns shared
infrastructure, `parser/` owns syntax, `semantic/` owns meaning resolution and
lowering, `ir/` owns the backend-facing canonical model,
`backend/lvgl_c/` owns code generation, and `cli/` owns orchestration. Cross-
layer shortcuts, adjacent-stage internals access, or backend-driven semantic
decisions are prohibited. This keeps changes local, testable, and reversible.

### V. Ratified Narrow Slices with Docs, Tests, and Fixtures
The project MUST prefer a narrow, ratified MVP slice over broad partial support.
Grammar, widget support, property support, and example claims MUST advance
phase-by-phase, and every meaningful compiler change MUST update the relevant
docs, fixtures, and tests together. This keeps the repo honest, prevents
speculative breadth, and preserves one coherent story across `docs/`,
`examples/`, `tests/`, and the codebase.

## Operational Constraints

- `LVGLuamUI` is the repository identity and `LumaUI` is the product identity.
	Planning, docs, and generated artifacts MUST keep those names distinct.
- Source truth MUST stay text-first. Authored source, diagnostics, generated C,
	fixtures, and diffs MUST remain meaningful in version control and CI. Binary
	or opaque intermediates MUST NOT become the committed pipeline contract.
- Generated output follows a hybrid ownership model. Compiler-owned regions MUST
	be overwritten on regeneration, user-owned escape-hatch regions MUST be
	clearly delimited and preserved, and manual edits outside escape hatches MUST
	be treated as invalid.
- The authored language grammar is provisional until ratified in
	`docs/LANGUAGE_SPEC.md`. Parser or backend work MUST NOT silently finalize
	syntax that the docs still mark as deferred.
- Unsupported or ambiguous mappings MUST be documented in `docs/LVGL_MAPPING.md`
	instead of being guessed in implementation.
- Hard out-of-scope areas remain full HTML/CSS semantics, DOM emulation,
	JavaScript or on-device scripting, browser rendering parity, WYSIWYG-first v1
	workflows, CSS cascade/specificity, and runtime binding evaluation on device.

## Workflow & Phase Gates

- All plans and task lists MUST identify the active phase and relevant exit gate
	from `docs/TASKS.md` and `docs/NEXT_STEPS.md` before implementation begins.
- A proposal that touches language shape MUST state whether the syntax or
	property surface is already ratified in `docs/LANGUAGE_SPEC.md`. If it is not,
	the work MUST stop at documentation and ratification tasks rather than moving
	ahead with implementation.
- A proposal that introduces a new widget, property, or layout rule MUST name
	the LVGL 9.x primitive or API family it maps to. If that mapping is unclear,
	the feature MUST be deferred and documented.
- Work that forces simultaneous redesign across parser, semantic, IR, and
	backend layers indicates that the slice is too large. The plan MUST reduce the
	slice before coding continues.
- Docs, examples, fixtures, and tests MUST be updated in the same change as any
	meaningful compiler behavior change. Generated C changes MUST carry snapshot or
	equivalent regression coverage.
- Review and planning artifacts MUST reject convenience-driven scope expansion,
	browser semantics, or backend cleverness that compensates for unresolved
	semantic rules.

## Governance

This constitution operationalizes the repository rules for Speckit workflows and
supersedes conflicting defaults in generated plan, spec, and task artifacts.
`docs/CONSTITUTION.md` is the narrative companion for the repo and MUST remain
materially aligned with this file.

Amendments MUST identify the changed section, explain why the change is needed,
and update any affected docs, fixtures, tests, and templates in the same change.
Amendments MUST also state the active phase and exit gate they preserve or
modify.

Versioning follows semantic versioning for governance changes:
- MAJOR for incompatible principle removals or redefinitions.
- MINOR for new principles, sections, or materially expanded obligations.
- PATCH for clarifications, wording improvements, and non-semantic refinements.

Compliance review is mandatory for every feature spec, implementation plan, task
list, and review that changes compiler behavior or project scope. Reviews MUST
check compile-time-first architecture, LVGL mapping clarity, determinism, layer
isolation, and docs/tests/fixtures alignment.

**Version**: 1.0.0 | **Ratified**: 2026-04-03 | **Last Amended**: 2026-04-05
