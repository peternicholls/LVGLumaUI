---

description: "Task list template for feature implementation"
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are REQUIRED whenever parser behavior, semantic validation,
IR shape, backend output, diagnostics, examples, or generated artifacts change.
Omit them only for docs-only work, and state that justification explicitly.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Compiler crates**: `cli/`, `compiler/`, `parser/`, `semantic/`, `ir/`,
  `backend/lvgl_c/`
- **Project docs**: `docs/`
- **Golden fixtures**: `examples/`
- **Regression coverage**: `tests/` and crate-local tests under each touched
  workspace member

<!-- 
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.
  
  The /speckit.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/
  
  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment
  
  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ratification and setup work shared across the feature slice

- [ ] T001 Confirm active phase and exit gate in docs/TASKS.md and docs/NEXT_STEPS.md
- [ ] T002 Ratify or update affected language or mapping docs in docs/
- [ ] T003 [P] Add or update fixture inputs in examples/ for the target slice

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

Examples of foundational tasks (adjust based on the affected compiler stage):

- [ ] T004 Update shared diagnostics, config, or project loading contracts in compiler/
- [ ] T005 [P] Add parser coverage in parser/tests or parser/src/
- [ ] T006 [P] Add semantic or IR validation coverage in semantic/ or ir/
- [ ] T007 Document LVGL mapping decisions in docs/LVGL_MAPPING.md
- [ ] T008 Define snapshot or golden-test expectations in tests/ or backend/lvgl_c/tests/
- [ ] T009 Confirm example claims do not exceed the ratified surface

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 1 ⚠️

> **NOTE: Add failing tests, fixtures, or snapshots before implementation when behavior changes**

- [ ] T010 [P] [US1] Add parser, semantic, or backend regression coverage in [exact path]
- [ ] T011 [P] [US1] Add or update fixture or snapshot coverage in [exact path]

### Implementation for User Story 1

- [ ] T012 [P] [US1] Implement stage-local changes in [exact crate path]
- [ ] T013 [P] [US1] Implement supporting contracts in [exact crate path]
- [ ] T014 [US1] Integrate the slice across the minimum required stages only
- [ ] T015 [US1] Update docs or examples required for the new supported surface
- [ ] T016 [US1] Add validation and deterministic diagnostics
- [ ] T017 [US1] Verify generated output or normalized models remain stable

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 2 ⚠️

- [ ] T018 [P] [US2] Add regression coverage in [exact path]
- [ ] T019 [P] [US2] Add or update fixture or snapshot coverage in [exact path]

### Implementation for User Story 2

- [ ] T020 [P] [US2] Implement stage-local changes in [exact crate path]
- [ ] T021 [US2] Update dependent compiler contracts in [exact crate path]
- [ ] T022 [US2] Implement the user-visible feature in [exact crate path]
- [ ] T023 [US2] Update docs, mappings, or examples that define the slice

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 3 ⚠️

- [ ] T024 [P] [US3] Add regression coverage in [exact path]
- [ ] T025 [P] [US3] Add or update fixture or snapshot coverage in [exact path]

### Implementation for User Story 3

- [ ] T026 [P] [US3] Implement stage-local changes in [exact crate path]
- [ ] T027 [US3] Update dependent compiler contracts in [exact crate path]
- [ ] T028 [US3] Implement the user-visible feature in [exact crate path]

**Checkpoint**: All user stories should now be independently functional

---

[Add more user story phases as needed, following the same pattern]

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] TXXX [P] Documentation updates in docs/
- [ ] TXXX Code cleanup and refactoring
- [ ] TXXX Determinism or snapshot stability verification
- [ ] TXXX [P] Additional crate-level or snapshot tests in tests/ or crate-local tests/
- [ ] TXXX Review escape-hatch, mapping, and generated-output implications
- [ ] TXXX Validate the updated example or quickstart flow

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests, fixtures, or snapshots MUST be added and fail before implementation
- Shared contracts before downstream stage integrations
- Stage-local implementation before cross-stage wiring
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "Add parser, semantic, or backend regression coverage in [exact path]"
Task: "Add or update fixture or snapshot coverage in [exact path]"

# Launch all models for User Story 1 together:
Task: "Implement stage-local changes in [exact crate path]"
Task: "Implement supporting contracts in [exact crate path]"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests, fixtures, or snapshots fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
