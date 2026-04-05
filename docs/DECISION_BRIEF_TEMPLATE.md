# Decision Brief Template

## Purpose

Use this template for major decisions that need research, structured discussion, and explicit developer sign-off before they are treated as settled.

Typical uses:

- language ratification choices
- parser, semantic, diagnostic, or IR contract changes
- backend ownership-boundary policy
- mapping conventions with meaningful tradeoffs
- observability choices that affect operator-facing command behavior
- preview/runtime direction changes

## Filename

Use one of these locations:

- `specs/<feature-id>/decisions/D-###-short-title.md` for feature-scoped decisions
- `docs/` only when the decision is repository-wide and no longer tied to one feature

## Template

```md
# D-###: <Decision Title>

## Status

`draft` | `proposed` | `approved` | `rejected` | `superseded`

## Date

YYYY-MM-DD

## Owner

Name or role

## Scope

Repository-wide or feature-specific scope.

## Decision Summary

One short paragraph describing the decision under consideration.

## Problem Statement

What problem needs to be solved, and why now?

## Constraints and Context

- technical constraints
- product constraints
- architecture constraints
- documentation or governance constraints

## Options Considered

### Option A: <Name>

Description.

Pros:

- point
- point

Cons:

- point
- point

### Option B: <Name>

Description.

Pros:

- point
- point

Cons:

- point
- point

## Relevant Practices and References

- repository practices
- external constraints
- comparable implementation patterns

## Implementation Developments and Evidence

- current repository state
- code or fixture findings
- test or snapshot implications
- operational observations

## Risks and Tradeoffs

- risk
- risk

## Open Questions

- question
- question

## Recommendation

State the recommended option and why.

## Deferred Items

- item intentionally not decided now
- item intentionally deferred to later phase

## Developer Sign-Off

Decision: `approved` | `rejected` | `revise and resubmit`

Signed off by:

Date:

Notes:
```

## Usage Rules

- Keep the summary short and factual.
- Make the options meaningfully distinct.
- Prefer concrete tradeoffs over advocacy language.
- Record evidence from the current repository when available.
- Leave the sign-off section explicit; do not imply approval in surrounding prose.