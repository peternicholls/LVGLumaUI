# Document Reclassification Audit

## Purpose

This audit identifies current documents that should remain active, stay feature-scoped, or be reclassified under the documentation scheme for better housekeeping and reference hygiene.

The goal is to reduce ambiguous authority, remove stale entry points, and make the active document set easier to navigate.

## Implementation Status

Status as of 2026-04-05:

- root `AGENTS.md` has been created as the canonical agent-instructions file
- `docs/intent.md` has been reclassified into archived historical material
- `docs/one-shot-prompt.md` has been archived
- `docs/project_specification.md` has been archived
- `README.md` workspace layout has been updated to match the active documentation set

## Classification Summary

### Keep Active in `docs/`

- `ARCHITECTURE.md`
- `DOCUMENTATION_SCHEME.md`
- `LANGUAGE_SPEC.md`
- `LVGL_MAPPING.md`
- `NEXT_STEPS.md`
- `PRD.md`
- `TASKS.md`
- `DECISION_BRIEF_TEMPLATE.md`

Reason:
These documents are repository-wide, active, and aligned with the current filing scheme.

### Keep Active in `specs/001-brownfield-spec/`

- `spec.md`
- `research.md`
- `plan.md`
- `data-model.md`
- `quickstart.md`
- `tasks.md`
- `contracts/cli-commands.md`
- `checklists/requirements.md`
- `decisions/README.md`

Reason:
These belong to the current feature packet and already follow the intended feature-scoped structure.

## Documents That Should Be Reclassified

### `docs/intent.md`

Current classification:

- ambiguous repository-wide doc

Recommended classification:

- either promote to root `AGENTS.md` after review and cleanup
- or archive it as a historical precursor if the repo will not use a root agent-instructions file

Why:

- the content is agent-operating guidance, not general repository documentation
- the current filename does not reflect its authority or purpose
- it duplicates policy already present in the active docs set

Implemented action:

- curated content was promoted into root `AGENTS.md`
- `docs/intent.md` was archived as historical precursor material

### `docs/one-shot-prompt.md`

Current classification:

- active-looking repository doc

Recommended classification:

- archive document

Why:

- it is bootstrap material for project creation, not ongoing repository governance
- it is not part of the canonical active documentation set
- it reads as historical setup context rather than current operating guidance

Implemented action:

- moved to `docs/archive/` as historical project-bootstrap material

### `docs/project_specification.md`

Current classification:

- active-looking repository doc

Recommended classification:

- archive document unless it is intentionally retained as historical origin material

Why:

- the repository now has a canonical feature packet under `specs/001-brownfield-spec/`
- the filename overlaps conceptually with `spec.md` and can confuse document authority
- it appears to be a precursor artifact rather than the current source of truth

Implemented action:

- moved to `docs/archive/` with a pointer to the canonical feature packet

### `docs/archive/CONSTITUTION.md`

Current classification:

- archived governance document

Recommended classification:

- keep archived until intentionally restored

Why:

- the document still has historical or governance value
- the current repository filing scheme already treats it as historical
- it should not silently regain active authority without an explicit restoration decision

Recommended action:

- no move required now
- if restored later, promote it deliberately and update linked docs in the same change

## Additional Cleanup Findings

### Root `AGENTS.md` is absent

Resolution:

- `AGENTS.md` now exists at the repository root and is the canonical agent-instructions file

### Historical bootstrap material should be visually separated from active docs

Observation:

- `docs/` currently mixes canonical active docs with historical bootstrap material

Implemented action:

- historical bootstrap documents were moved into `docs/archive/`

## Proposed Reclassification Plan

1. Promote agent guidance into root `AGENTS.md`.
2. Archive `docs/one-shot-prompt.md`.
3. Archive `docs/project_specification.md`.
4. Archive `docs/intent.md` as historical precursor material.
5. Update `README.md` workspace layout so it matches the real repository state.

## Sign-Off Note

This audit now records the implemented reclassification state and can serve as the reference point for future cleanup passes.