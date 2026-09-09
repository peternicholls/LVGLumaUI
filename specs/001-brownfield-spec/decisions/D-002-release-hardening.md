# D-002: 1.0.0 release hardening record

## Status, date and scope

Implementation/verification record, 2026-09-09. Scope: fulfill the requested
usable 1.0.0 release using the implemented LS-0.2.0 compiler contract.

## Existing direction

The integration branch already implements five widgets, scalar styles, named
clicked-event callbacks, configurable symbol prefixes and ownership markers.
The constitution requires preserved user-owned code. This work fixes gaps in
those existing behaviors; it does not ratify a broader language or preview.

## Evidence and changes

New regressions demonstrated empty projects and nested screens were accepted,
invalid prefixes and oversized dimensions reached C, and output slug collisions
silently overwrote files. Callback registrations had no declarations. The
backend's exact snapshot did not match its output, and regeneration discarded
user code. Each fix has targeted regression coverage and matching documentation.

The existing callback signature is retained, now declared in generated headers.
The existing compiler/user ownership convention is honored with a single
preserved C region and preflight rejection of malformed markers. Explicit
firmware files remain the recommended home for callback definitions.

## Alternatives and tradeoffs

Keeping the old behavior would ship a compiler that passes a narrow Rust suite
but emits uncompilable C or loses user edits. Widening the language would create
new decisions and distract from establishing a reliable release. Keep the current
slice and require real target-library compilation alongside Rust tests.

## References

- Current LANGUAGE_SPEC, LVGL_MAPPING, and constitution.
- LVGL 9.2.2 `src/misc/lv_area.h` reserves coordinate type bits starting at 29.
- https://lvgl.io/docs/open/9.2/overview/event documents the clicked-event callback.

## Verification and remaining limits

The release workflow requires three host-platform packages and the LVGL 9.2.2
headless widget/style/event test. Firmware hardware, unsigned macOS distribution,
and device-specific theme behavior remain outside automated validation.
Disk failures can leave partial output; stale renamed-screen outputs require
manual cleanup. Internal crates are not a supported stable library API.

## Developer direction

The active user request authorizes completing and releasing a usable 1.0.0.
No deferred widget, style, asset or preview decision is made by this change.
