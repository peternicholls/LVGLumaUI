# Feature Specification: Brownfield MVP Compiler Slice

**Feature Branch**: `001-brownfield-spec`  
**Created**: 2026-04-05  
**Status**: Draft  
**Input**: User description: "create a full specification. this is a brownfield project. all the relevant documentation for planning and intent is in the /docs/ directory. These should be referenced against as you build your SpecKit specify specification"

## Clarifications

### Session 2026-04-05

- Q: Should the first ratified slice include event references, bindings, both, or neither? -> A: Include named event handler references in the first slice, but defer bindings.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ratify the first supported language contract (Priority: P1)

As a LumaUI maintainer, I need the brownfield repository to define one narrow, explicit, and hierarchically consistent MVP authoring contract so the team can stop treating provisional docs and fixtures as if they were already ratified behavior.

**Why this priority**: The current repository is still at the Phase 0 / Phase 1 boundary. The next required step in the brownfield roadmap is to freeze the smallest supported language subset before parser, semantic, and backend work expand.

**Independent Test**: This can be fully tested by reviewing the brownfield spec and the docs package together and confirming that a reviewer can identify the governing document order, the current phase baseline, the exact first-slice supported surface, and the explicitly deferred surface without making undocumented assumptions.

**Acceptance Scenarios**:

1. **Given** the docs directory contains both governing rules and provisional guidance, **When** maintainers review the ratified brownfield spec, **Then** they can identify that `CONSTITUTION.md` is the governing authority, the repo is currently in Phase 0 with a Phase 1 language-freeze transition ahead, and the first thin slice is defined as an explicit supported contract rather than an inferred intent.
2. **Given** a provisional example, aspirational document, or fixture conflicts with the ratified thin slice, **When** maintainers decide whether the construct is in scope, **Then** the governing document hierarchy and the ratified slice take precedence and the conflicting construct is marked as deferred, rejected, or aspirational instead of silently accepted.
3. **Given** the first supported slice is approved, **When** a reviewer examines the language contract, **Then** they can see the exact supported widgets, selectors, identifiers, minimal style surface, event-reference policy, and binding deferral policy for the first end-to-end path.

---

### User Story 2 - Validate the normative thin slice (Priority: P2)

As an embedded developer using LumaUI, I need the repository's first normative example to validate against the ratified subset and to fail clearly when authored input exceeds that subset.

**Why this priority**: Validation is the first trustworthy proof that the project has moved beyond provisional scaffolding while still respecting the narrow-slice discipline required by the brownfield docs.

**Independent Test**: This can be fully tested by validating `examples/minimal` as the normative first slice and validating intentionally invalid or aspirational fixtures to confirm that accepted input succeeds and unsupported input fails with source-located guidance.

**Acceptance Scenarios**:

1. **Given** a project that stays within the ratified first slice, **When** the user validates it, **Then** the system accepts both screen and style sources and produces a consistent parsed representation suitable for downstream semantic work.
2. **Given** a project that contains malformed syntax, unknown widgets, unsupported properties, duplicate identifiers, unsupported bindings, or otherwise deferred constructs, **When** the user validates it, **Then** the system reports each issue with source context, clear remediation guidance, and no silent fallback.
3. **Given** the repository contains both normative and aspirational examples, **When** validation coverage is reviewed, **Then** maintainers can tell which fixtures are expected to pass in the current slice and which remain intentionally ahead of implementation.

---

### User Story 3 - Deliver one phase-gated end-to-end build path (Priority: P3)

As a firmware team integrating generated UI code, I need one minimal example to move through the ratified thin slice into deterministic, readable LVGL output so the repo proves one honest end-to-end path without pretending broader support already exists.

**Why this priority**: The brownfield roadmap consistently prefers one complete thin slice over broad partial support. The first end-to-end path is valuable only if it follows the documented gate order instead of skipping unresolved language or semantic decisions.

**Independent Test**: This can be fully tested by confirming that the normative minimal example progresses through the documented gate sequence, builds into reviewable generated artifacts, and produces stable output on repeated runs from the same input.

**Acceptance Scenarios**:

1. **Given** the language contract for the first slice has been ratified and the normative example stays within it, **When** the user builds that example, **Then** the system emits generated screen artifacts that match the documented LVGL mappings, preserve the documented hybrid ownership model, and require no manual translation.
2. **Given** the same input and compiler version are used in repeated builds, **When** output is regenerated, **Then** the generated artifacts remain stable enough for snapshot review and version-control diffs.
3. **Given** downstream implementation work has not yet satisfied an earlier phase gate, **When** maintainers plan the next task, **Then** the repo continues in the documented order rather than broadening the slice or skipping directly to preview, asset expansion, or wider widget support.

### Edge Cases

- What happens when `CONSTITUTION.md` and another planning document appear to disagree? The constitution remains the governing authority and the spec must treat the lower-priority document as subordinate guidance that needs alignment.
- What happens when `examples/minimal` uses a construct that is not explicitly ratified? The construct must be documented, either added to the supported first slice through explicit ratification or removed from the normative example.
- What happens when `examples/dashboard` or another aspirational fixture uses deferred widgets or properties? The fixture must be labeled aspirational or expected-fail until the supported slice expands to include those constructs.
- What happens when provisional `.lui` or `.lus` examples imply syntax that the ratified subset does not adopt? The ratified grammar wins and the provisional form must be updated, rejected, or clearly marked non-normative.
- What happens when a property or widget lacks a clean named LVGL mapping? The construct stays deferred and must not be normalized or emitted through guessed backend behavior.
- What happens when authored input includes a binding reference in the first slice? Validation must reject it as out of scope for the current ratified subset.
- What happens when duplicate identifiers appear across a screen tree? Validation must reject the project before lowering or code generation continues.
- What happens when a generated file contains user-owned escape-hatch content and regeneration occurs? Compiler-owned regions are replaced while clearly delimited user-owned regions remain preserved.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The brownfield specification MUST treat the docs directory as the authoritative source corpus for intent and planning, with `docs/CONSTITUTION.md` as the governing authority when any other document conflicts.
- **FR-002**: The feature MUST reflect the current repository baseline accurately: Phase 0 Foundation is substantially complete, the authored language remains provisional, and this brownfield slice exists to satisfy the entry assumptions and exit goals for Phase 1 rather than to claim later phases are already complete.
- **FR-003**: The feature MUST ratify the exact first-slice markup grammar and the exact first-slice style grammar needed for one honest end-to-end compiler path.
- **FR-004**: The ratified first slice MUST explicitly define the supported widget set, identifier and class support, selector surface, text usage, and minimal property surface for the normative minimal example.
- **FR-005**: The ratified first slice MUST explicitly define the accepted named event handler reference form for the MVP path and MUST explicitly defer bindings from the first slice, including how binding usage is rejected during validation.
- **FR-006**: The supported first slice MUST remain narrow enough that it does not force simultaneous redesign of parser, semantic, IR, and backend layers beyond the thin-slice sequencing described in the brownfield docs.
- **FR-007**: The feature MUST document the phase-gated delivery order for this slice: ratify language first, then parser work, then semantic validation and lowering, then one backend path, then fixture and stability alignment.
- **FR-008**: `examples/minimal` MUST be treated as the first normative fixture for the ratified slice, and every construct it uses MUST be either explicitly supported by the slice or removed from the normative path.
- **FR-009**: The system MUST allow users to validate a project that stays within the ratified first slice and return a success outcome only when both screen and style sources conform to the documented subset.
- **FR-010**: The system MUST reject malformed syntax, unknown widgets, unsupported properties, duplicate identifiers, binding usage, and other out-of-scope authored constructs with actionable diagnostics that identify source context and corrective direction.
- **FR-011**: The semantic contract for the first slice MUST include duplicate-identity checks, supported widget validation, supported property validation, explicit event reference representation, and lowering of valid input into a canonical IR free of syntax-specific ambiguity.
- **FR-012**: Users MUST be able to build at least one normative minimal example from authored source into generated LVGL screen artifacts without manual translation steps once the language, parser, and semantic gates for the slice are satisfied.
- **FR-013**: Generated output for the ratified slice MUST remain deterministic, readable, and compatible with the documented hybrid ownership model for compiler-owned and user-owned regions.
- **FR-014**: The repository MUST distinguish normative fixtures that are expected to pass in the current slice from aspirational fixtures that remain ahead of implementation, and that distinction MUST be visible in docs, examples, tests, and snapshots.
- **FR-015**: The feature MUST keep the supported language surface aligned across `docs/LANGUAGE_SPEC.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, `docs/LVGL_MAPPING.md`, `README.md`, normative examples, parser and semantic tests, and generated snapshots so the repository tells one consistent brownfield story.
- **FR-016**: The feature MUST preserve explicit deferrals for unratified grammar, browser-like semantics, preview workflows, asset-pipeline expansion, wider widget support, richer style complexity, and LVGL 8.x compatibility until later phases ratify them.

### Key Entities *(include if feature involves data)*

- **Brownfield Document Hierarchy**: The ordered set of repository documents used to govern this feature, with the constitution as the highest authority and the remaining planning documents as subordinate guides that must be kept aligned.
- **Ratified First Slice**: The explicitly supported authored-language subset for the first end-to-end path, including exact grammar rules, permitted widgets, selectors, identifiers, accepted event references, and the explicit exclusion of bindings.
- **Normative Fixture**: A repository example that is expected to validate or build successfully in the current ratified slice and therefore acts as a trusted acceptance artifact.
- **Aspirational Fixture**: A repository example that intentionally stays ahead of current implementation and must be clearly labeled so it does not imply unsupported behavior is already part of the slice.
- **Authored Project**: A LumaUI project composed of configuration, screen source files, and style source files that serves as the user-facing input to validation and build workflows.
- **Canonical IR**: The backend-facing representation of valid authored input after unsupported constructs, duplicate identities, and ambiguous declarations have been resolved or rejected.
- **Generated Screen Artifact**: The emitted LVGL C screen output owned by the compiler except for explicitly delimited user-owned escape-hatch regions.
- **Diagnostic Record**: A user-facing validation or build finding that identifies severity, source location, and corrective guidance for unsupported or invalid input.
- **Phase Gate**: A repository milestone condition that must be satisfied before the next layer of work broadens scope, preventing the brownfield project from skipping unresolved design decisions.

## Constitution Alignment *(mandatory)*

- **Active Phase**: The repository is currently in Phase 0: Foundation, with `docs/LANGUAGE_SPEC.md` describing the work as a Phase 0 / Phase 1 boundary. This feature bridges that boundary by making the first supported language slice explicit and by defining the bounded path through the next thin-slice phases without claiming those later phases are already complete.
- **Stage Ownership**: `docs/` ratifies the brownfield rules first; Phase 1 centers on `parser/` with `compiler/` diagnostics support; Phase 2 centers on `semantic/` and `ir/`; Phase 3 centers on `backend/lvgl_c/` and `cli/`; Phase 4 centers on examples, regression coverage, and snapshot trustworthiness.
- **LVGL Mapping**: Supported emitted constructs in this slice must map only to the documented LVGL 9.x primitives already named for `Screen`, `Column`, `Row`, `Text`, and `Button`. Any widget, property, or interaction without an explicit named mapping remains deferred.
- **Grammar Status**: Grammar is currently provisional. This feature must move the first slice from provisional examples to an explicit accepted-and-rejected contract, including exact markup and style grammar plus the event-reference rule and binding deferral policy.
- **Docs/Fixtures/Tests Impact**: Changes must stay aligned across `docs/LANGUAGE_SPEC.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, `docs/LVGL_MAPPING.md`, `README.md`, `examples/minimal`, any dashboard or other aspirational example labels, parser tests, semantic tests, CLI validation coverage, and generated C snapshots.
- **Explicit Deferrals**: Full HTML or CSS semantics, CSS cascade and specificity, browser-style layout or DOM behavior, all bindings in the first slice, complex selectors, `Container`, `Grid`, `Image`, `Card`, preview integration, broader asset handling, richer style-property expansion, and LVGL 8.x compatibility remain deferred unless later phases ratify them.

## Phase-Gated Delivery Plan

1. **Language Gate**: Update the brownfield language contract so the supported and unsupported first-slice syntax is explicit, the exact event-reference rule is documented, bindings are explicitly out of scope, and `examples/minimal` can be reviewed without guesswork.
2. **Parser Gate**: Parse the normative first slice into real AST structures for screen and style sources, report syntax diagnostics with spans, and cover both valid and invalid first-slice cases.
3. **Semantic Gate**: Validate duplicate identifiers, supported widgets, supported properties, and event references; normalize supported declarations; and lower valid authored input into canonical IR.
4. **Backend Gate**: Build one normative minimal example from the canonical IR into deterministic, reviewable LVGL output that honors the documented ownership model and mapping constraints.
5. **Examples and Stability Gate**: Keep normative fixtures trustworthy, keep aspirational fixtures clearly marked, and align docs, tests, and snapshots every time the slice grows.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Reviewers can identify the governing document hierarchy, the current repo phase baseline, and the next required phase gate from the brownfield spec and docs without relying on undocumented assumptions.
- **SC-002**: Every construct used by the first normative minimal example is explicitly classified as supported, deferred, or rejected in the ratified first-slice contract.
- **SC-003**: Users can validate the normative first-slice example successfully, and intentionally out-of-scope fixtures produce source-located diagnostics for each unsupported construct they contain.
- **SC-004**: One normative minimal example can be built from authored source into generated screen artifacts through a repeatable workflow that requires no manual C translation.
- **SC-005**: Re-running generation for unchanged normative input produces no unintended differences in generated screen artifacts.
- **SC-006**: Maintainers can distinguish normative fixtures from aspirational fixtures in one review pass across docs, examples, tests, and snapshots.
- **SC-007**: The next implementation sequence can proceed from ratified language scope through parser, semantic, backend, and fixture-stability work without requiring a scope-expanding redesign of multiple stages at once.

## Assumptions

- The current repository state has already satisfied the practical Phase 0 foundation gate closely enough that the next work should focus on language ratification rather than reopening repository-shape decisions.
- The first valuable brownfield slice is the `examples/minimal` path described across `README.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, and `docs/ARCHITECTURE.md`, not a broad expansion of the aspirational eventual language surface.
- The first ratified subset should remain small enough to prioritize `Screen`, `Column`, `Row`, `Text`, `Button`, identifiers, classes, a tiny style surface, and named event handler references before broader widgets, layout forms, bindings, or preview concerns are reconsidered.
- Existing aspirational examples may remain in the repository, but they must be labeled or handled in a way that does not imply support beyond the ratified first slice.
- Users of this phase expect text-first workflows, deterministic outputs, and source-located diagnostics rather than runtime preview behavior, asset-pipeline breadth, or browser-style convenience features.
