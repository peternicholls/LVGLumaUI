# Next Steps

## Current phase

Stability and release verification for 1.0.0. The implemented language remains
LS-0.2.0; this release adds no widgets, properties, selectors or runtime language.

The compiler pipeline is implemented end to end. The release work fixes
validation, C declarations and output-preservation defects, and adds native
packages, integration documentation and a real LVGL smoke test.

## Release gate

- All Rust tests, formatting and strict Clippy checks pass.
- The normative examples validate and build deterministically.
- User-region preservation and invalid-input rejection have regression tests.
- Linux, Windows and macOS packages execute their included compiler and example.
- Generated C compiles with LVGL 9.2.2 and passes the headless runtime test.
- The version, lockfile, changelog, public usage docs and release notes agree.

The GitHub release job publishes only after every required job succeeds on the
integration branch. A release number or successful Rust test alone does not
establish that this gate has passed.

## After 1.0

Keep bug fixes compatible with the documented CLI and authored-language slice.
Proposals for preview, grids, images, bindings, percentage sizes, multiple
classes or richer styling require the existing decision/sign-off workflow.
Do not implement them as incidental release polish.

Use `scripts/lumaui-phase-check.sh --require-build` for the standard verification
bundle. Keep backend and frontend snapshots exact and synchronized. Follow
`docs/TASKS.md` for phase branches and PRs into `001-brownfield-spec`.
