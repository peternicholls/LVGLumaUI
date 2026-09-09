# D-001: Native preview contract

Status: proposed. Owner: maintainer. Date: 2026-09-09.
Scope: Phase 5 host execution, dependencies and CLI behavior.
Implementation is not approved by this document.

## Decision summary

Recommend a maintained CMake host runner using LVGL and SDL, consuming existing
generated C and explicitly supplied callback sources. Preserve LS-0.2.0 and the
firmware integration API. Approve the first supported host set before coding.

## Context and evidence

The released CLI gates preview in `cli/src/main.rs::preview_project`.
`tests/lvgl/CMakeLists.txt` already builds generated minimal/layout C and
`examples/minimal/firmware.c` against LVGL; its runtime test is headless.
The release workflow pins LVGL 9.2.2. The live product demonstration successfully
regenerated a title while preserving a user callback, but did not display a window.

LVGL documents desktop simulation and SDL-backed simulator projects, including
CMake options. That supports investigating a native path; it does not establish
that our runner or dependency combination already works on all hosts.
Source reviewed 2026-09-09:
[LVGL 9.2 PC simulator](https://lvgl.io/docs/open/9.2/integration/ide/pc-simulator).

## Options considered

| Option | Advantages | Costs and limitations |
| --- | --- | --- |
| A. Maintained CMake/LVGL/SDL runner (recommended) | Same generated C as firmware; repeatable example and lifecycle tests; CLI can explain prerequisites | Own native build integration and host qualification; users need native dependencies |
| B. CLI launches a user-maintained simulator project | Less dependency packaging; existing firmware adapters may be reused | Less predictable setup, failure reporting and screen selection; difficult clean-install promise |
| C. Documentation-only simulator integration | Smallest maintenance burden; no new CLI contract | Does not close the deferred preview gap or provide the requested visual workflow |

A browser/DOM reinterpretation is excluded by the constitution rather than offered
as an alternative.

## Proposed contract decisions

- Reuse the build pipeline; host code selects a generated screen factory.
- Single-screen default; multiple screens require explicit selection.
- Explicit host C source list or adapter, with real callback implementations.
  Missing callbacks fail; silent no-op callback generation is excluded.
- Preview dependencies are optional for compiler users. Begin with explicitly
  configured local dependencies; decide distribution/pinning after the spike.
- A dedicated preview build directory prevents interference with firmware output.
  The contract must explain user-region preservation and stale-build cleanup.
- One process per command invocation; closing the window succeeds; native failure
  and interruption have defined exits and cleanup. No automatic reload initially.
- Linux-first qualification is proposed; Windows/macOS must be qualified or clearly
  excluded from preview support with approval. Their compiler support remains.

## Risks and tradeoffs

C sources are executable code, not safe untrusted input. A host callback can only
simulate device behavior. Dependency licensing, native ABI differences, graphical
CI availability, external tool errors and caches need explicit handling.
Do not mask these gaps with success messages or platform-wide compatibility claims.

## Open questions for review

1. Approve A, B or C, and the initial preview platform set?
2. Which exact LVGL/SDL/toolchain revisions and dependency acquisition policy
   does the spike justify? Should any native dependency be bundled?
3. What CLI/config syntax selects screen, host adapter, dependency paths and
   dimensions, and what precedence rules apply?
4. How are preserved generated user regions copied into isolated preview output,
   and who owns cache cleanup?
5. What exit/logging contract and test evidence are required for release?

## Deferred items

Watch mode, reload, editor integration, firmware flashing, language expansion,
binding evaluation and device equivalence are excluded from this decision.
See [deferred work](../../../docs/DEFERRED_WORK.md).

## Developer sign-off

Decision: pending (`approved`, `rejected`, or `revise and resubmit`).
Signed off by: pending.
Date: pending.
Approved platform/dependency/contract scope: pending.
