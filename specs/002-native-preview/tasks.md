# Phase 5 execution checklist

Status: proposed; all implementation tasks are pending.
Owner: maintainer, who assigns the implementer/reviewer when each task starts.
Date: 2026-09-09. See [plan](plan.md) for scope and acceptance.

## Planning completed

- [x] Reconcile the published 1.0.0 baseline with the phase roadmap.
- [x] Inventory language, tooling, distribution and validation deferrals.
- [x] Document preview alternatives and explicit sign-off points.

## M0: reusable release maintenance

- [ ] P5-01 Audit hard-coded 1.0.0 in the release workflow and package script.
      Specify one workspace version source and matching notes/tag/asset names.
- [ ] P5-02 Before implementation, add command-level checks for version mismatch,
      Windows LF checksum portability, existing published release, interrupted
      draft, partial asset upload and a retry. Preserve public releases; recover
      only the matching intended draft. Use a non-public test or mocked publisher.
- [ ] P5-03 Implement the approved release flow; document how future versions are
      triggered and how an operator retries safely. Validate all archive checksums.
      Depends on P5-01/02. Do not retag 1.0.0.

## M1: preview decision and contract

- [ ] P5-04 Spike an LVGL 9.2.2 + SDL host build. Record exact SDL/toolchain
      versions, build commands, licensing/redistribution considerations and
      Linux/Windows/macOS feasibility. This spike must not widen language support.
- [ ] P5-05 Review D-001; record approval or required revisions. Decide supported
      platforms, dependency sourcing, callback sources and preview cache ownership.
      Depends on P5-04.
- [ ] P5-06 After approval, write `contracts/cli-commands.md` with exact
      arguments/config precedence, screen selection, dimensions, error reporting,
      callback source handling and process exit behavior. Add failing command
      assertions before implementing. Depends on P5-05.

## M2/M3: runner and CLI

- [ ] P5-07 Add a minimal native runner and explicit example callback adapter;
      compile the generated minimal/layout C. Add headless runtime checks plus
      a graphical test procedure. Depends on P5-06.
- [ ] P5-08 Reuse compiler orchestration without changing canonical generated
      output. Add prerequisite detection and stage-specific errors. Native tools
      receive argument arrays, including paths with spaces. Depends on P5-07.
- [ ] P5-09 Implement build/launch/close and interruption cleanup; exercise missing
      callbacks, invalid inputs, native failures and stale-output prevention.
      Confirm firmware output and user code survive. Depends on P5-08.

## M4: qualification and handoff

- [ ] P5-10 Run the graphical click/close scenario on Linux x86_64 and record
      evidence. Qualify Windows x86_64 and macOS arm64, or explicitly obtain
      approval for a narrower preview support table. Depends on P5-09.
- [ ] P5-11 Run clean-install package verification on every advertised platform.
      Existing compiler tests/packages must still pass on all three.
- [ ] P5-12 Update README, USAGE, ARCHITECTURE, PRD current-status wording and
      CLI help to match delivered behavior. Add `quickstart.md` with reproducible
      commands only after they work. Keep dashboard aspirational.
- [ ] P5-13 Run the complete plan acceptance gate, prepare version/changelog/release
      notes and publish the agreed candidate only after review. Depends on M0 and
      P5-10/11/12. No completion checkbox is evidence without linked results.

## Delivery branches

After the planning PR merges, use a flat phase branch such as
`codex/phase-5-release-maintenance` for M0, then a new
`codex/phase-5-native-preview` from the merged integration tip for preview.
Keep PRs scoped to their milestone and base them on `001-brownfield-spec`.
If the preview contract is revised, update the decision and tests before code.
