# Deferred work after 1.0.0

Status: planning register, not a language contract. Updated: 2026-09-09.
Owner: maintainer. Baseline: v1.0.0 / LS-0.2.0.
[Phase 5 plan](../specs/002-native-preview/plan.md) and
[tasks](../specs/002-native-preview/tasks.md) define the proposed immediate work.
Priorities describe order, not delivery promises.

## Next phase

| ID | Priority / work | Evidence and dependency | Acceptance / disposition |
| --- | --- | --- | --- |
| D01 | P0: reusable release workflow | Release workflow explicitly names 1.0.0; Windows checksum issue required a follow-up | Version/tag/notes agree; portable checksums; safe public-release retry and draft recovery; Phase 5 M0 |
| D02 | P1: native preview | CLI is gated; headless LVGL test exists | Real generated-C window, click, close, prerequisite failures and clean install; M1–M4 after D-001 approval |
| D03 | P1: preview platform/dependency support | Compiler packages exist for Linux x86_64, Windows x86_64, macOS arm64; preview support does not | Explicit tested matrix, pinned dependency recipe and redistribution review; no inferred support |
| D04 | P1: runnable demonstration documentation | Demo lives on codex/demo-1.0.0; minimal/layout are normative on integration | Review/import useful demo material without altering generated-code contracts; preview example records actual native behavior |

## Later reliability and distribution work

| ID | Work and reason deferred | Entry condition / acceptance |
| --- | --- | --- |
| D05 | Build transaction recovery; a disk error can leave partial output | Separate ownership/recovery decision; injected write failures preserve recoverable user code and never report a partial build as successful |
| D06 | Cleanup of renamed/deleted screens; currently manual | Define owned-file manifest and user-code recovery first; unrelated files and preserved regions must never be silently deleted |
| D07 | Signed/notarized macOS packages, broader OS/CPU coverage and install channels | Maintainer distribution requirements and signing resources; verify install/update on each added target, document actual trust status |
| D08 | Hardware qualification and broader LVGL versions | Choose a board/driver/toolchain and explicit LVGL revision matrix; record build, display, input and callback results. Current headless test is not hardware certification |
| D09 | Preview watch/reload and editor conveniences | Stable one-shot preview first; define lifecycle/state preservation and failed-reload behavior; no second renderer |
| D10 | Additional diagnostics or machine-readable output | Concrete operator need and observability decision; stable command assertions and source locations; preserve ordinary stderr behavior unless approved |

## Future language proposals

Every row remains unsupported. Require a decision brief naming exact syntax,
validation, IR impact and LVGL 9 mapping before code; advance the language MINOR
version and language changelog for an approved expansion. Implement one coherent
slice at a time, with positive/negative fixtures and exact C snapshots.

| ID | Deferred surface | Decision needed before scheduling |
| --- | --- | --- |
| L01 | Percentage sizing | Meaning, limits and parent-size behavior; named LVGL sizing mapping and numeric tests |
| L02 | Multiple classes, tag/attribute selectors and selector combinators | Resolution order and conflict rules without introducing CSS specificity; preserve current later-rule behavior unless explicitly versioned |
| L03 | Margin, radius, borders, alignment, flex overrides and richer styling | A small property set with units, ranges, defaults and named LVGL style/layout APIs; fonts depend on L05 |
| L04 | Container, Grid, Card and other widgets | Distinct semantics, children and mappings; Grid also needs track/placement rules. Dashboard stays aspirational until all its constructs are ratified |
| L05 | Image/font assets, asset references and theme tokens | Resource ownership, naming, conversion tools, sizes/formats, reproducibility and firmware linkage; no implicit asset pipeline |
| L06 | Events beyond Button onPress | Supported widget/event pairs, callback lifetime/signatures and generated ABI |
| L07 | Bindings and view-model integration | Compile-time/static C interface proposal only; ownership/update API and lifetime rules. Runtime expression evaluation remains excluded |
| L08 | String escapes/entities and broader grammar conveniences | Explicit lexical rules, encoding and diagnostics with regression fixtures; current no-escape semantics remain |
| L09 | LVGL 8.x compatibility | Concrete demand and separate adapter/version policy; not part of Phase 5 or the current LVGL 9 contract |

## Excluded product directions

Full HTML/CSS behavior, CSS cascade/specificity, DOM emulation, browser rendering
parity, JavaScript/on-device interpretation, dynamic template evaluation and a
WYSIWYG-first product are constitution exclusions, not ordinary backlog promises.
Reconsidering them would require an explicit product/governance change.

## Source of truth and maintenance

Supported syntax remains in [LANGUAGE_SPEC](LANGUAGE_SPEC.md), mappings in
[LVGL_MAPPING](LVGL_MAPPING.md), operational limits in [USAGE](USAGE.md), and
release semantics in [VERSIONING](VERSIONING.md). This register grants no support.

At each phase review, mark the relevant row scheduled, completed or still deferred,
link its approved decision and verification results, and update the canonical
references in the same change. Compatible bug fixes can ship on 1.0.x; preview is
a proposed 1.1.0 addition. Do not widen the language to unblock preview.
