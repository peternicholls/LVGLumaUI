# D-001: Usable 1.0 compiler release

## Status

Proposed; awaiting developer sign-off.

## Date and owner

2026-09-08 — Codex, for project maintainer review.

## Scope and decision summary

Complete roadmap phases 1–4 for the already-ratified LS-0.2.0 surface and ship
the resulting compiler as 1.0.0. Preserve the staged Rust architecture. Preview,
Grid, Image, Card, bindings and richer styling remain explicitly deferred.

## Problem and evidence

The baseline passes four tests but returns empty AST and IR models. Build cannot
emit authored projects. Parser tests now exercise the normative fixture and
reject malformed input. Event signatures and output preservation are still
unsettled in the mapping document. These must be concrete before calling the
compiler stable.

## Options considered

1. Ship the ratified compiler slice: small, testable public surface with a real
   firmware integration example. It excludes the broader aspirational v1 list.
2. Ratify all broader widgets and preview first: more capabilities, but requires
   further language and asset design and delays a reliable release.

For events, use `LV_EVENT_CLICKED` (activation on release, excluding scrolling)
or `LV_EVENT_PRESSED` (immediate press). Recommend clicked for conventional
button activation. Generated headers declare `void <prefix>event_<encoded-name>(lv_event_t *event)`.
The firmware supplies these functions in separate user-owned C files; missing
implementations fail at link time. Hyphens and underscores are encoded distinctly
so all ratified identifiers remain usable without symbol collisions.

## Proposed implementation contract

- Semantic validation owns project-wide identifiers, accepted attributes,
  selector matching, precedence, typed dimensions/colors and normalized padding.
- IR adds explicit styles and an optional named press reference to each widget;
  screen root properties apply to the actual LVGL screen, not a wrapper object.
- Backend emits output-relative `screens/*_gen.c` and `.h`, respecting configured
  symbol prefix. Root ids name screens; absent ids use the source file stem.
  Duplicate resulting screen names are errors before writing output.
- Preserve delimited user regions in regenerated C/header files to retain the
  constitution's hybrid ownership model. Separate callback files are preferred.
  Invalid region markers fail before overwriting output. Do not delete old files
  automatically; identify stale output in integration documentation.
- CLI returns nonzero on syntax/semantic errors, writes nothing on validation
  failure, reports diagnostics on stderr and stable summaries on stdout.
- A generated-C build and headless LVGL runtime test must supplement Rust tests
  before release; packaging includes installation and firmware integration steps.

## Relevant practices and references

LVGL's official event documentation demonstrates `lv_obj_add_event_cb` with a
`void (lv_event_t *)` callback and `LV_EVENT_CLICKED`, which occurs on release if
the interaction did not scroll:
https://lvgl.io/docs/open/9.2/overview/event

The existing LANGUAGE_SPEC, data-model and CLI contract prescribe the narrow
surface, typed semantic lowering, deterministic discovery and generated output.

## Risks and tradeoffs

Stable version numbering commits to the documented compiler surface, not every
aspirational feature. A separately versioned LS contract avoids conflating these.
LVGL theme defaults still affect appearance. Firmware owns LVGL setup, event
logic and object lifetime. C link testing is needed to catch ABI mistakes that
snapshots alone cannot find.

## Recommendation and deferred items

Choose option 1 and the clicked-event callback contract above. Defer preview,
bindings, assets and language expansion. Preserve compile-time-first behavior,
stage isolation, deterministic artifacts and paired tests/documentation.

## Developer sign-off

Decision: pending. The active task asks the maintainer to choose this scope.
No unratified extension is implemented while this decision is pending.
