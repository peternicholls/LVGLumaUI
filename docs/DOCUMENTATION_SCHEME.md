# Documentation Storage and Filing Scheme

## Purpose

This document defines where documentation should live, how it should be named, and how it should move through draft, review, sign-off, and archive states.

The goal is simple:

- make documents easy to find
- keep repository-wide guidance separate from feature-specific work
- preserve a clear review trail for decisions that require developer sign-off
- reduce stale or duplicate documents

## Filing Principles

- Put documents as close as possible to the scope they govern.
- Keep one canonical home for each document type.
- Separate active reference material from historical or superseded material.
- Distinguish working research from approved decisions.
- Prefer predictable filenames over creative names.
- Archive, do not silently overwrite, when a document has historical value.

## Top-Level Zones

Use these zones consistently.

### `docs/`

Repository-wide documentation that applies across features.

Use this area for:

- architecture
- language and backend reference material
- roadmap and execution guidance
- repository-wide governance
- shared filing and contribution conventions
- approved cross-cutting decisions

### `specs/<feature-id>/`

Feature-specific working packet for one initiative.

Use this area for:

- the feature specification
- research material
- implementation plan
- data model
- task list
- contracts
- checklists
- feature-level decision briefs and sign-off notes

### `docs/archive/`

Superseded or historical repository-wide material that should remain discoverable but should no longer be treated as current guidance.

### `tests/`, `examples/`, and code-adjacent README files

Use local READMEs only for material tightly coupled to those folders, such as fixture classification, example status, or test participation rules. Do not duplicate repository-wide policy there.

## Canonical Homes by Document Type

| Document type | Canonical location | Notes |
| --- | --- | --- |
| Repository-local agent instructions | `AGENTS.md` | Active agent guidance at the repository root. |
| Repository overview | `README.md` | Entry point only; keep concise and link outward. |
| Repository changelog | `CHANGELOG.md` | Root-level release history for the repository. |
| Architecture and stage ownership | `docs/ARCHITECTURE.md` | Cross-feature compiler structure only. |
| Language contract | `docs/LANGUAGE_SPEC.md` | Normative authored-language scope and constraints. |
| Language contract changelog | `docs/LANGUAGE_CHANGELOG.md` | Revision history for the authored-language contract. |
| LVGL backend mapping | `docs/LVGL_MAPPING.md` | Cross-feature backend reference and mapping limits. |
| Versioning policy | `docs/VERSIONING.md` | Repository-wide versioning, tagging, and changelog rules. |
| Repository roadmap and execution order | `docs/TASKS.md`, `docs/NEXT_STEPS.md` | Roadmap and operational guidance. |
| Filing rules and housekeeping | `docs/DOCUMENTATION_SCHEME.md` | This document. |
| Decision brief template | `docs/DECISION_BRIEF_TEMPLATE.md` | Reusable repository-wide format for research and sign-off documents. |
| Feature specification | `specs/<feature-id>/spec.md` | One spec per feature packet. |
| Feature research | `specs/<feature-id>/research.md` | Research notes and evidence. |
| Feature implementation plan | `specs/<feature-id>/plan.md` | Design and implementation strategy. |
| Feature tasks | `specs/<feature-id>/tasks.md` | Execution checklist. |
| Feature quickstart | `specs/<feature-id>/quickstart.md` | Verification and operator workflow for the feature. |
| Feature data model | `specs/<feature-id>/data-model.md` | Domain entities and relationships. |
| Feature contracts | `specs/<feature-id>/contracts/` | CLI, API, or interface contracts for that feature. |
| Feature checklists | `specs/<feature-id>/checklists/` | Review and acceptance checklists. |
| Feature decision briefs | `specs/<feature-id>/decisions/` | Discussion-ready options and sign-off records. |
| Local example guidance | `examples/*/README.md` | Example-specific status only. |
| Local test guidance | `tests/README.md` | Test and fixture rules only. |

## Recommended Folder Scheme

The current repository can keep its existing active files in place. For all new documents, use the following structure.

```text
docs/
  ARCHITECTURE.md
  LANGUAGE_SPEC.md
  LANGUAGE_CHANGELOG.md
  LVGL_MAPPING.md
  NEXT_STEPS.md
  PRD.md
  TASKS.md
  VERSIONING.md
  DOCUMENTATION_SCHEME.md
  archive/

CHANGELOG.md

specs/
  <feature-id>/
    spec.md
    research.md
    plan.md
    data-model.md
    quickstart.md
    tasks.md
    contracts/
    checklists/
    decisions/
      README.md
      D-001-<short-title>.md
      D-002-<short-title>.md
```

## Decision Brief Scheme

Major stage-shaping decisions should have a durable, reviewable home.

Store them in:

- `specs/<feature-id>/decisions/` for feature-scoped decisions
- `docs/` only when the decision is repository-wide and no longer scoped to one feature

Recommended filename format:

- `D-001-language-slice-ratification.md`
- `D-002-ir-contract-boundaries.md`
- `D-003-backend-ownership-policy.md`

Each decision brief should contain:

- title
- status: `draft`, `proposed`, `approved`, `rejected`, or `superseded`
- owner
- date
- scope
- decision summary
- options considered
- pros and cons
- relevant practices or external constraints
- implementation developments or evidence
- risks and tradeoffs
- open questions
- developer sign-off section

## Naming Rules

Use these naming conventions consistently.

- Repository-wide canonical docs: uppercase, descriptive names such as `ARCHITECTURE.md`.
- Feature packet core files: fixed lowercase names such as `spec.md`, `plan.md`, `research.md`, `tasks.md`.
- Decision briefs: `D-###-short-kebab-case-title.md`.
- Checklists: concise lowercase names in `checklists/`, such as `requirements.md`.
- Contracts: concise lowercase names in `contracts/`, such as `cli-commands.md`.
- Archive files: preserve the original name when practical; prefix with a date if needed to avoid collision.

Avoid:

- vague names like `notes.md`, `thoughts.md`, or `misc.md`
- duplicate documents with overlapping authority
- embedding status only in prose when the filename or location should already imply it

## Lifecycle Rules

### Draft

Working material under active discussion.

- feature-scoped drafts belong in `specs/<feature-id>/`
- repository-wide drafts belong in `docs/` only if they already govern the whole repo

### Proposed

Ready for developer review but not yet approved.

- keep in the same canonical location
- mark status clearly in the document body

### Approved

Signed off and authoritative.

- remain in the canonical location
- update linked docs in the same change so guidance stays synchronized

### Superseded or Rejected

No longer current, but worth retaining for auditability.

- move repository-wide material to `docs/archive/`
- keep feature-level historical decision material inside the feature packet, marked `superseded` or `rejected`

## Housekeeping Rules

- Do not create a new document when an existing canonical document should be updated instead.
- When a document is superseded, add a short pointer to the newer canonical document.
- Link related documents rather than copying the same policy into multiple places.
- Keep `README.md` as an index and orientation layer, not a full policy dump.
- Review feature packets at story or phase boundaries and archive or trim stale supporting material.
- If a document requires developer sign-off, include a dedicated sign-off section rather than leaving approval implicit.

## Current Repository Application

Apply the scheme to the current repository like this:

- `AGENTS.md` is the canonical repository-local agent instruction file.
- `README.md` remains the top-level entry point.
- `CHANGELOG.md` is the canonical repository release history.
- `docs/ARCHITECTURE.md`, `docs/LANGUAGE_SPEC.md`, `docs/LVGL_MAPPING.md`, `docs/TASKS.md`, and `docs/NEXT_STEPS.md` remain the canonical repository-wide active set.
- `docs/LANGUAGE_CHANGELOG.md` is the canonical change history for the authored-language contract.
- `docs/VERSIONING.md` is the canonical versioning and tagging policy.
- `docs/archive/CONSTITUTION.md` remains historical until or unless it is restored as an active governing file.
- `docs/archive/intent.md`, `docs/archive/one-shot-prompt.md`, and `docs/archive/project_specification.md` are historical bootstrap or precursor material.
- `specs/001-brownfield-spec/` remains the canonical feature packet for the current slice.
- Future discussion and sign-off material for that feature should go in `specs/001-brownfield-spec/decisions/`.

## Minimum Filing Checklist

Before adding a new document, confirm:

1. The document has one clear scope: repository-wide or feature-specific.
2. The filename matches its document type and authority.
3. The canonical home is obvious from its location.
4. Related docs are linked instead of duplicated.
5. If the document drives a major decision, it includes status and developer sign-off handling.