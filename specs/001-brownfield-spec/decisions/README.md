# Decision Briefs

## Purpose

This directory stores discussion-ready decision material for `001-brownfield-spec`.

Use it for stage-shaping choices that require research, tradeoff analysis, and explicit developer sign-off before they are treated as settled.

## File Naming

Use:

- `D-001-<short-title>.md`
- `D-002-<short-title>.md`

## Required Contents

Each decision brief should include:

- title
- status
- owner
- date
- scope
- decision summary
- options considered
- pros and cons
- relevant practices or constraints
- implementation developments or evidence
- risks and tradeoffs
- open questions
- developer sign-off

## Status Values

Use one of:

- `draft`
- `proposed`
- `approved`
- `rejected`
- `superseded`

## Current Use

Create a decision brief here before finalizing major choices about:

- language ratification
- shared compiler contracts
- IR boundaries
- backend ownership policy
- observability conventions that affect command behavior

Use `docs/DECISION_BRIEF_TEMPLATE.md` as the repository-wide starting format.
