# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION]  
**Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]  
**Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A]  
**Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION]  
**Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS CLARIFICATION]
**Project Type**: [e.g., library/cli/web-service/mobile-app/compiler/desktop-app or NEEDS CLARIFICATION]  
**Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60 fps or NEEDS CLARIFICATION]  
**Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory, offline-capable or NEEDS CLARIFICATION]  
**Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [ ] The proposal preserves the compile-time-only path and does not introduce
  runtime interpretation, DOM-style mutation, or browser-only semantics.
- [ ] Every new widget, property, event surface, or layout rule maps cleanly to
  a named LVGL 9.x primitive or API family, or is explicitly deferred in
  `docs/LVGL_MAPPING.md`.
- [ ] The impacted crates respect stage boundaries (`compiler/`, `parser/`,
  `semantic/`, `ir/`, `backend/lvgl_c/`, `cli/`). If multiple stages must be
  redesigned at once, the slice has been reduced first.
- [ ] Any syntax or property-surface change is already ratified in
  `docs/LANGUAGE_SPEC.md`, or this plan stops at ratification work instead of
  implementing unratified grammar.
- [ ] The active phase and exit gate from `docs/TASKS.md` and `docs/NEXT_STEPS.md`
  are identified and preserved.
- [ ] Required updates to docs, examples, fixtures, and tests or snapshots are
  listed explicitly in this plan.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
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

**Structure Decision**: Identify the exact crates, docs, examples, and tests
touched by the feature. Plans MUST explain why each touched stage is in scope
and why adjacent stages do not need redesign.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., unratified grammar work] | [current need] | [why ratification-first was insufficient] |
| [e.g., multi-stage redesign] | [specific problem] | [why a narrower compiler slice was insufficient] |
