# Phase 5: Native preview and release maintenance

Status: proposed; documentation only. Owner: maintainer. Date: 2026-09-09.
Baseline: [v1.0.0](https://github.com/peternicholls/LVGLumaUI/releases/tag/v1.0.0),
commit `3cfbafd3d39abc663d47c203803d66c03fe254d2`, language LS-0.2.0.
Target: a compatible 1.1.0 candidate, conditional on acceptance; no release date committed.

## Outcome

A developer can compile an existing supported screen, see it in a native desktop
window, click its button, and close the process cleanly. Preview consumes the same
generated C and user callbacks used by firmware. It must not interpret markup or
introduce a second renderer.

Phase 0–4 and the 1.0.0 release gate are complete. Phase 5 is proposed, not implemented.
The current `preview` command still returns a gated error.
[The decision brief](decisions/D-001-native-preview-contract.md) must be approved
before its preview contract becomes implementation authority. The request to plan
does not ratify the proposed API, dependencies, or platform scope.

## Scope and ordering

| Milestone | Deliverable | Dependency | Completion evidence |
| --- | --- | --- | --- |
| M0: release maintenance | Version-driven packaging/publication plan implemented and verified | Existing 1.0 release behavior understood | Candidate tag, notes, workspace version and archive names agree; repeat publication preserves an existing public release; interrupted draft is recoverable |
| M1: contract approval | Approved preview decision and concrete CLI/config contract | Decision review and prerequisite spike | Recorded sign-off covers platforms, dependency sourcing, callbacks, screen selection and lifecycle |
| M2: native runner | CMake/LVGL/SDL desktop executable using generated C | M1 | Minimal and layout screens compile; visible window and click callback work on the first supported host |
| M3: CLI orchestration | Validate, generate, compile and launch through preview | M2 | Failure-path assertions and lifecycle tests pass; ordinary build needs no SDL |
| M4: qualification | Repeatable desktop demo, platform matrix and release docs | M3 | Required gates below pass; support claims match actual evidence |

M0 and the M1 research work can be prepared independently; implementation phases
still use sequential review branches. No new compiler features are needed to
demonstrate the released widget/style slice.

## Proposed narrow preview slice

- One selected screen per invocation. Use the sole screen automatically; require
  an explicit selection when a project has multiple screens. Unknown selections
  must fail before native compilation.
- Use an explicit host adapter: user-provided C sources implement generated
  callback declarations. Missing callbacks cause an actionable link failure.
  Do not silently provide no-op stubs or link arbitrary firmware source trees.
- A maintained example adapter changes a label on click. Device-specific drivers,
  services and hardware behavior are outside this demonstration.
- Build into a dedicated preview directory, separate from authored sources and
  normal firmware output. Preserve explicit user regions through the existing
  generation path; record how preserved regions reach the preview build.
- One-shot launch and manual rerun. No watcher, live reload, editor, asset compiler
  or language additions.
- Proposed first qualification host: Linux x86_64, matching existing LVGL CI.
  Windows x86_64 and macOS arm64 follow as explicit qualification tasks. A
  Linux-only initial preview release needs approval and an explicit support table;
  compiler-only support on all three platforms remains required.
- Start the dependency spike at the already tested LVGL 9.2.2 revision
  `7f07a129e8d77f4984fff8e623fd5be18ff42e74`. Investigate SDL2 and CMake,
  then pin the tested combination. Do not infer compatibility with every LVGL 9.x.
  Dependency paths are explicit; no silent installation or network fetch on launch.

These are recommendations pending M1, not current supported options.

## Stage ownership

Keep generation in the existing parser → semantic → IR → LVGL C pipeline.
The CLI coordinates subprocesses, configuration and error presentation. A small
host-runner module or support directory owns CMake templates, native entrypoint
and SDL setup; its final location is an M1 implementation detail.

Do not put SDL handles, filesystem paths or subprocess state in IR. Do not teach
the backend preview-specific widget semantics. Any refactor to reuse build
orchestration must retain exact existing snapshots and public build behavior.

## Acceptance and verification

1. Run formatting, strict Clippy and
   `bash scripts/lumaui-phase-check.sh --require-build`; all existing tests and
   canonical output snapshots remain valid.
2. Compile and execute existing headless LVGL tests. They establish generated-C
   behavior but are not evidence that a desktop window or physical input works.
3. In a graphical session on each advertised preview platform, record OS,
   architecture, toolchain and dependency revisions; show minimal and layout
   screens; click the example button; confirm the label changes; close cleanly.
4. Add automation for invalid source, no/multiple/unknown screen, absent compiler,
   CMake/LVGL/SDL, missing callback, native compile/link failure, unavailable
   display, paths containing spaces, native nonzero exit and Ctrl+C cleanup.
   Failures must identify the stage and useful remediation, exit nonzero and
   never launch a stale executable. Repeated launches must leave no child process.
5. Verify normal `validate` and `build` work without preview dependencies.
   Confirm preview generation does not change authored files or lose user code.
6. Test the packaged CLI and documented preview setup from a clean environment,
   not just a developer checkout. Publish prerequisite and platform limits.
7. Before release, test the version-driven publication path and checksum files
   across Windows/Linux/macOS, including interrupted draft recovery. Never move
   or replace the existing v1.0.0 tag/assets.

## Risks and release policy

Native compilation executes project-supplied C; preview is for trusted local
projects and is not a sandbox. Callback adapters may behave differently from real
hardware and must say so. SDL/display availability and Windows compiler discovery
are qualification risks, not reasons to invent language workarounds.

Compatible maintenance may ship as 1.0.x. Adding the preview command's supported
behavior is a proposed 1.1.0 feature. Keep LS-0.2.0 unchanged if no authored-language
surface changes. Any language expansion has its own decision and language MINOR
revision; breaking public changes follow [versioning](../../docs/VERSIONING.md).

## Execution

Use [tasks](tasks.md) for implementation handoff and
[the deferred register](../../docs/DEFERRED_WORK.md) for everything outside this slice.
Merge this planning branch into `001-brownfield-spec` after review, then create
each implementation branch from the updated integration tip. Do not merge the
superseded parser WIP as a shortcut to future syntax support.
