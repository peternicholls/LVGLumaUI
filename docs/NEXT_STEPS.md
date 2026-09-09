# Next steps

## Baseline

LumaUI 1.0.0 is [published](https://github.com/peternicholls/LVGLumaUI/releases/tag/v1.0.0).
Its [final release workflow](https://github.com/peternicholls/LVGLumaUI/actions/runs/34389031479)
passed compiler/package checks on Linux, Windows and macOS plus the LVGL 9.2.2
headless runtime test. Phases 0–4 are complete. The language remains LS-0.2.0.

## Active work: Phase 5 planning

The proposed next phase delivers an optional native preview of generated C,
preceded by reusable release maintenance. Planning is complete enough for review;
implementation and the preview contract are not yet approved.

Read in order:

1. [Phase 5 plan](../specs/002-native-preview/plan.md): scope, dependencies and exit gate.
2. [Preview decision](../specs/002-native-preview/decisions/D-001-native-preview-contract.md):
   alternatives, evidence and pending sign-off.
3. [Execution tasks](../specs/002-native-preview/tasks.md): ordered implementation checklist.
4. [Deferred work](DEFERRED_WORK.md): later language, reliability and distribution work.

The next action is the release-maintenance contract review and dependency spike,
then explicit preview decision sign-off. Do not treat proposed CLI flags,
dependency acquisition or platform support as released behavior.

## Exit gate

Preview must use the existing generated C, display a selected screen, execute a
real example callback and exit cleanly on each advertised preview platform.
Invalid input, missing prerequisites, link errors and interruption need tested
failure behavior. Existing compiler snapshots and all three native compiler
packages must remain valid; build/validate must not require SDL.

A clean-install demonstration and version-driven publication verification are
required before a 1.1.0 candidate. Graphical evidence is separate from the existing
headless test. The plan specifies the full acceptance matrix.

Keep compatible bug fixes on 1.0.x where appropriate. No language expansion is
part of this plan. Follow [TASKS](TASKS.md) for phase branches and review.
