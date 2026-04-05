# Implementation Plan: Brownfield MVP Compiler Slice

**Branch**: `001-brownfield-spec` integration branch with one child branch per phase | **Date**: 2026-04-05 | **Spec**: `/specs/001-brownfield-spec/spec.md`
**Input**: Feature specification from `/specs/001-brownfield-spec/spec.md`

## Summary

Ratify the narrowest honest LumaUI language slice for `examples/minimal`, then use that ratified subset to drive one gated implementation path from parser work through semantic validation, canonical IR, and deterministic LVGL C generation. The plan keeps Phase 1 centered on documentation and parser completion, treats semantic, IR, and backend work as downstream gates rather than simultaneous redesign, and keeps bindings, broader widgets, and browser-like semantics explicitly deferred.

Delivery follows a phase-branch workflow: `001-brownfield-spec` remains the integration branch, each phase is implemented on its own child branch, each child branch lands by PR into `001-brownfield-spec`, and the next phase branch is cut only after the previous phase merge completes.

## Technical Context

**Language/Version**: Rust 2021 workspace on the stable toolchain; authored `.lui` and `.lus` sources remain provisional until ratified  
**Primary Dependencies**: `anyhow`, `clap` with derive, `serde` with derive, `toml`; LVGL 9.x C API family as the backend target  
**Storage**: Filesystem only (`lumaui.toml`, `ui/screens/*.lui`, `ui/styles/*.lus`, generated `generated/ui/*.c` and `*.h`)  
**Testing**: `cargo test`, parser unit tests, semantic validation tests, backend snapshot tests, CLI validation and build smoke checks against `examples/minimal`, plus deterministic diagnostic and logging assertions where command observability is part of the contract  
**Target Platform**: Host-side Rust CLI for developer machines and CI; generated output targets embedded firmware projects using LVGL 9.x  
**Project Type**: Multi-crate compiler workspace with a CLI frontend  
**Performance Goals**: Deterministic parsing, stable diagnostics, and repeatable generated output take priority over raw throughput; the normative example should validate and build fast enough for tight local iteration and CI use  
**Constraints**: Compile-time-only workflow, no runtime interpretation, no browser semantics, strict stage isolation, explicit LVGL mapping only, hybrid ownership model for generated output, bindings rejected in the first slice, and deterministic stage-scoped logging that does not pollute generated artifacts or stable diagnostics  
**Scale/Scope**: One normative minimal project, two authored source kinds, five ratified widgets (`Screen`, `Column`, `Row`, `Text`, `Button`), class and id selectors, a tiny style-property subset, named event handler references, and one deterministic backend slice

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] The proposal preserves the compile-time-only path and does not introduce runtime interpretation, DOM-style mutation, or browser-only semantics.
- [x] Every new widget, property, event surface, or layout rule maps cleanly to a named LVGL 9.x primitive or API family, or is explicitly deferred in `docs/LVGL_MAPPING.md`.
- [x] The impacted crates respect stage boundaries (`compiler/`, `parser/`, `semantic/`, `ir/`, `backend/lvgl_c/`, `cli/`). If multiple stages must be redesigned at once, the slice has been reduced first.
- [x] Any syntax or property-surface change is already ratified in `docs/LANGUAGE_SPEC.md`, or this plan stops at ratification work instead of implementing unratified grammar.
- [x] The active phase and exit gate from `docs/TASKS.md` and `docs/NEXT_STEPS.md` are identified and preserved.
- [x] Required updates to docs, examples, fixtures, and tests or snapshots are listed explicitly in this plan.

**Gate Result**: Pass. The plan stays within the documented Phase 0 to Phase 1 transition: it ratifies the MVP language surface first, keeps parser work as the immediate implementation focus, and treats semantic, IR, backend, and fixture stabilization as ordered downstream gates instead of concurrent redesign.

## Project Structure

### Documentation (this feature)

```text
specs/001-brownfield-spec/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── cli-commands.md
└── tasks.md
```

### Source Code (repository root)

```text
cli/
compiler/
parser/
semantic/
ir/
backend/lvgl_c/
docs/
examples/
tests/
```

**Structure Decision**: Phase 1 implementation primarily touches `docs/`, `examples/minimal`, `parser/`, `compiler/`, and parser-focused tests because those are the components required to ratify grammar and parse real authored input. `semantic/`, `ir/`, `backend/lvgl_c/`, and `cli/` are in scope only to define stable contracts and later gates; they should not be redesigned during parser completion. Broadening more than one adjacent stage at once is treated as a signal that the language slice is too large.

## Phase 0 Research Summary

Research findings are recorded in `research.md`. The resulting decisions that drive this plan are:

1. Use `examples/minimal` as the sole normative fixture for the first ratified slice.
2. Ratify an XML-like markup subset aligned with existing starter files and examples instead of inventing a second syntax before parser completion.
3. Limit the first supported widget set to `Screen`, `Column`, `Row`, `Text`, and `Button`.
4. Limit the first supported selector and property surface to class and id selectors plus `padding`, `background-color`, `text-color`, `width`, and `height`.
5. Allow named event handler references in the first slice and explicitly reject bindings.
6. Keep semantic responsibility focused on duplicate ids, supported-surface validation, event-reference representation, and lowering to canonical IR.
7. Keep backend work constrained to explicit LVGL 9.x mappings already documented for the ratified subset.
8. Distinguish normative and aspirational fixtures across docs, examples, tests, and snapshots before expanding the surface.

## Phase 1 Design Outputs

- `data-model.md` defines the domain entities and relationships that the parser, semantic layer, IR, CLI, and backend must agree on for the first slice.
- `contracts/cli-commands.md` defines the user-facing CLI behaviors for `doctor`, `validate`, `build`, `preview`, and `init` across the active brownfield phase.
- `quickstart.md` defines the implementation and verification flow for moving the minimal example through ratification, parsing, semantic validation, backend generation, and stability checks.

## Implementation Strategy

**Engineering quality rule**: Code-bearing tasks follow test-driven development, keep helpers and modules single-purpose, and treat logging and diagnostics as intentional user-facing behavior rather than temporary debugging output.

**Documentation quality rule**: Documentation changes travel with behavior changes. Normative docs, example READMEs, fixture labels, and operator guidance must use consistent terminology, clearly separate ratified behavior from deferred work, and stay concise enough for reviewers to verify against code and tests.

**Decision governance rule**: The agent should research and document stage-shaping choices before implementation locks them in. Supporting material should include options, pros and cons, relevant practices, implementation developments, risks, and open questions. Final direction is deferred until the developer reviews and signs off.

**Branch governance rule**: Phase-gated execution is also a Git workflow rule. Contributors work on one phase branch at a time, target `001-brownfield-spec` as the PR base for every phase branch, and avoid stacking later-phase implementation on unmerged earlier-phase work.

## Research and Sign-Off Expectations

The following categories require explicit developer review before they are considered settled:

- authored-language ratification decisions
- shared AST, semantic, IR, or diagnostic contract changes that affect downstream stages
- backend ownership-boundary conventions and generated-file policies
- observability conventions that change operator-facing command behavior
- preview/runtime direction beyond the current compiler-first scope

For these decisions, the agent should produce or update supporting documents in the feature packet and align repository docs only after the developer confirms the chosen direction.

### Workstream 1: Ratify the MVP Language Surface

**Goal**: Turn the provisional language guidance into an explicit accepted-and-rejected contract for the first thin slice.

**Primary changes**:

- Update `docs/LANGUAGE_SPEC.md` from provisional guidance to a ratified MVP subset for the first slice.
- Keep `docs/LVGL_MAPPING.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, and `README.md` aligned with the ratified scope and explicit deferrals.
- Align `examples/minimal` to the ratified subset and clearly label `examples/dashboard` or other broader fixtures as aspirational if they remain ahead of support.
- Keep terminology and support-status wording aligned across all ratification docs so reviewers can compare them without interpretation drift.

**Exit condition**: A reviewer can tell exactly what the parser must accept, what it must reject, which event reference form is valid, and that bindings remain out of scope.

**Sign-off requirement**: The supporting research and tradeoff material for the ratified slice has been reviewed and approved by the developer.

### Workstream 2: Complete the Parser Gate

**Goal**: Parse the ratified minimal subset into real AST structures for markup and style documents.

**Primary changes**:

- Extend `parser/` from tokenization scaffolding to real parsing for markup and style documents in the ratified slice.
- Keep parser output aligned with the current AST types or evolve those types narrowly where the ratified subset requires stronger representation.
- Use `compiler/` diagnostics to produce source-located syntax errors with actionable messages.
- Add parser tests that cover valid minimal fixtures and invalid constructs such as malformed syntax, unsupported widgets, unsupported selectors, and binding syntax.

**Exit condition**: `lumaui validate examples/minimal` parses real authored syntax end to end, and parser tests cover both success and failure cases with spans.

### Workstream 3: Complete the Semantic Gate and Canonical IR Contract

**Goal**: Validate the parsed subset and lower it into a backend-ready canonical IR.

**Primary changes**:

- Expand `semantic/` from duplicate-id checks to supported widget validation, supported property validation, event-reference validation, and explicit binding rejection.
- Refine `ir/` only as needed to represent the canonical first-slice widget tree, normalized style information, and event metadata without syntax-specific ambiguity.
- Add semantic tests that cover duplicate ids, unsupported widgets, unsupported properties, out-of-scope bindings, successful lowering for the minimal example, and deterministic validation/logging behavior where exposed through the CLI contract.

**Exit condition**: The semantic layer rejects unsupported constructs clearly and produces a canonical IR for the normative example without leaking parser quirks downstream.

**Sign-off requirement**: Any contract changes that materially affect downstream lowering or backend assumptions have been discussed and approved by the developer before they are treated as stable.

### Workstream 4: Complete the Backend Gate

**Goal**: Generate deterministic, readable LVGL C from real semantic IR for the minimal example.

**Primary changes**:

- Connect `cli build` to the real parsed and semantically validated pipeline.
- Restrict `backend/lvgl_c/` to the explicitly mapped first-slice widgets and properties.
- Emit stable file names, stable symbol naming, stable widget creation order, and the documented hybrid ownership model boundaries.
- Replace or supplement synthetic backend-only snapshots with frontend-driven snapshots from `examples/minimal`.
- Add stage-scoped logging hooks and coverage so build progress and failure paths are observable without destabilizing snapshots or generated files.

**Exit condition**: The minimal example builds into stable `.c` and `.h` files with no manual translation and no backend guessing.

**Sign-off requirement**: Generated-file ownership boundaries, build-path observability behavior, and LVGL mapping conventions used by the slice have been reviewed and approved by the developer.

### Workstream 5: Normative Fixture and Stability Alignment

**Goal**: Make the repository tell one consistent story about what is supported now versus later.

**Primary changes**:

- Mark normative versus aspirational fixtures consistently in docs, examples, tests, and snapshot expectations.
- Keep `examples/minimal` as the trust anchor for current-phase success.
- Treat `examples/dashboard` and any broader fixtures as expected-fail or aspirational until the supported slice expands.
- Expand regression coverage only around implemented behavior and known failure cases.
- Use regressions to drive refactoring when duplication, ambiguous ownership, or leaky stage boundaries appear during implementation.
- Keep contributor-facing guidance concise and current so the repository can be understood without relying on historical context outside the docs.

**Exit condition**: Maintainers can review docs, examples, tests, and snapshots in one pass and immediately tell what is expected to pass today.

## File and Module Impact Map

| Area | Why It Is In Scope | Why Adjacent Areas Stay Narrow |
|------|--------------------|--------------------------------|
| `docs/LANGUAGE_SPEC.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, `docs/LVGL_MAPPING.md`, `README.md` | These documents define the ratified contract, phase ordering, and LVGL mapping discipline. | They describe the slice and gates; they do not justify implementing broader syntax or later-phase features immediately. |
| `examples/minimal` | This is the normative fixture and the only required passing authored project for the slice. | It remains intentionally small so parser and semantic work can stabilize before surface expansion. |
| `parser/` | Phase 1's main implementation focus is real AST construction and syntax diagnostics. | Parser work should not absorb semantic or backend responsibilities. |
| `compiler/` | Diagnostics and project loading must support parser and CLI workflows. | The crate remains shared infrastructure and should not take on semantic or backend logic. |
| `semantic/` | Required for duplicate-id checks, supported-surface validation, and lowering once parsing is stable. | It should consume stable AST rather than drive grammar design. |
| `ir/` | Needed to hold the canonical backend-facing model for the minimal slice. | It stays minimal and backend-oriented; it should not mirror raw syntax one-to-one. |
| `backend/lvgl_c/` | Required for one honest end-to-end LVGL C path and deterministic snapshots. | Backend support is limited to explicitly mapped constructs already ratified. |
| `cli/` | `validate` and `build` are the external entrypoints that prove the slice works. | CLI behavior should orchestrate stages, not contain business logic from any stage. |
| `tests/` and snapshots | These lock in success and failure behavior for the ratified slice. | Coverage should stay aligned to the implemented subset and not speculate beyond it. |

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Grammar scope creep during parser work | Parser, semantic, IR, and backend all start moving at once | Keep `examples/minimal` as the only normative target and reject additions that require multi-stage redesign. |
| Provisional examples drift from the ratified contract | Docs and fixtures tell conflicting stories | Treat docs plus the normative fixture as the source of truth and mark broader fixtures as aspirational until supported. |
| Backend semantics become guessy | Generated C appears to support behavior the language has not ratified | Require explicit LVGL mapping before implementation and defer ambiguous properties or widgets. |
| Determinism regressions | Snapshot churn and low trust in generated output | Preserve sorted discovery, stable iteration order, stable naming, and fixed formatting at every stage. |

## Phase 2 Readiness

This planning pass stops before task generation, but the repository is considered ready for `/speckit.tasks` when all of the following are true:

1. The ratified first-slice syntax and deferrals are explicit in the docs.
2. `examples/minimal` is aligned to that ratified subset.
3. Parser, semantic, IR, backend, and CLI responsibilities are separated cleanly enough to assign tasks without ambiguity.
4. Normative versus aspirational fixture status is visible to maintainers.

## Complexity Tracking

No constitution violations are expected for this plan. The design intentionally narrows scope rather than justifying unratified grammar work or multi-stage redesign.
