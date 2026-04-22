# LVGL Mapping

**Document Status**: active repository policy
**Versioning**: tracked through the repository/workspace version and `CHANGELOG.md`
**Decision Sync**: mapping-policy changes should stay aligned with approved decision briefs in `specs/001-brownfield-spec/decisions/`

## Scope

This document defines the intended mapping from LumaUI concepts to LVGL 9.x APIs. The goal is to keep mappings conservative, explicit, and auditable.

Where a mapping is not yet settled, it is called out as a TODO instead of being hand-waved.

The current ratified backend target is the first MVP slice defined in `docs/LANGUAGE_SPEC.md`. Broader widget and style rows in this document describe intended future direction, not permission for the backend to accept unvalidated authored input.

This document only defines backend mapping responsibilities. Syntax acceptance, semantic validation, command logging, and project discovery remain owned by their upstream stages.

It is also a workflow document for backend-mapping decisions: changes here should reflect approved repository direction, not speculative implementation shortcuts.

## Workflow Status

Current workflow posture:

- this document records active mapping policy and intended LVGL 9.x lowering direction
- it must stay aligned with the current ratified language slice and current feature packet
- unresolved or disputed mappings should be captured as explicit TODOs or decision briefs, not treated as implicit permission to generate code

## Mapping Workflow

Use this sequence when proposing or changing a mapping:

1. Confirm the authored-language surface is already ratified in `docs/LANGUAGE_SPEC.md` or the active feature packet.
2. Confirm the mapping fits the active phase and narrow-slice constraints in `docs/TASKS.md` and `docs/NEXT_STEPS.md`.
3. If the mapping changes repository policy, ownership boundaries, or generated-output conventions, write or update a decision brief under `specs/<feature-id>/decisions/` using `docs/DECISION_BRIEF_TEMPLATE.md`.
4. Update this document only after the recommended direction is ready for developer review or has already been approved.
5. Keep downstream backend code, snapshots, and operator-facing docs synchronized with the chosen mapping.

Do not use backend code as the place where unresolved mapping policy gets decided.

## Decision Sync Rules

This document must stay in sync with decision briefs and approved feature direction.

Rules:

- if a mapping row is ratified for active implementation, the corresponding decision state should be visible in the relevant feature packet or supporting docs
- if a mapping is still under discussion, this document should say so explicitly and point to a decision brief rather than presenting the mapping as settled
- if a decision brief is approved and changes backend policy, update this document in the same change or the next immediately linked follow-up
- if a previous mapping direction is superseded, remove or rewrite the outdated guidance here instead of leaving contradictory text behind

For the active brownfield slice, feature-scoped decision material belongs under `specs/001-brownfield-spec/decisions/`.

## Target Baseline

First-pass planning assumes LVGL 9.x.

Version adapters for LVGL 8.x or other compatibility modes are future work.

## Mapping Ownership

The mapping layer exists to answer one question: how does canonical IR become explicit LVGL 9.x calls?

It does not own:

- authored-language grammar decisions
- duplicate-id or supported-surface validation
- command orchestration or CLI output formatting
- logging policy beyond exposing backend-stage events for the CLI to present

If a mapping requires the backend to guess intent that should already have been resolved in `parser/` or `semantic/`, the upstream contract should be tightened first.

If a mapping change would force simultaneous redesign across `semantic/`, `ir/`, and `backend/lvgl_c/`, reduce the slice and capture the tradeoff in decision material before updating this document as active policy.


## Widget Mapping

| LumaUI widget | LVGL construct | Notes |
| --- | --- | --- |
| `Screen` | `lv_obj_create(NULL)` | Screen roots are base objects created with `NULL` parent and later loaded with `lv_screen_load`. |
| `Container` | `lv_obj_create(parent)` | Generic container wrapper. |
| `Row` | `lv_obj_create(parent)` + flex row layout | Lowered via `lv_obj_set_layout(..., LV_LAYOUT_FLEX)` and row flow configuration. |
| `Column` | `lv_obj_create(parent)` + flex column layout | Same base object, different flex flow. |
| `Grid` | `lv_obj_create(parent)` + grid layout | Lowered via `LV_LAYOUT_GRID` and generated row/column descriptors. |
| `Text` | `lv_label_create(parent)` | Text content lowered via `lv_label_set_text`. |
| `Button` | `lv_button_create(parent)` | Child text is typically emitted as a nested label. |
| `Image` | `lv_image_create(parent)` | Source handling depends on the future asset pipeline. |
| `Card` | `lv_obj_create(parent)` | No dedicated LVGL widget; treated as a styled container preset. |

Only `Screen`, `Column`, `Row`, `Text`, and `Button` should reach backend emission in the current slice. Broader rows in this table document intended direction, not permission for the backend to accept unvalidated constructs opportunistically.

When a row moves from intended direction to active implementation policy, the ratifying decision should be reflected in the active feature docs and any relevant decision brief.


## Layout Mapping

### Row and Column

Planned mapping:

- `lv_obj_set_layout(obj, LV_LAYOUT_FLEX)`
- `lv_obj_set_flex_flow(obj, ...)`
- `lv_obj_set_flex_align(obj, main, cross, track)` for supported align and justify subsets

The compiler should only expose align/justify values that can be mapped deterministically.

Layout normalization belongs upstream. By the time a layout reaches LVGL mapping, the backend should be consuming explicit canonical intent rather than interpreting ambiguous authored syntax.

### Grid

Planned mapping:

- `lv_obj_set_layout(obj, LV_LAYOUT_GRID)`
- generated `lv_coord_t` row/column descriptor arrays
- explicit placement on child widgets

TODO:

- finalize the authored-language grid vocabulary before locking the backend surface
- decide whether the MVP allows only explicit track definitions or also named templates
- record the chosen grid policy in the active feature decision packet before treating it as settled backend behavior

## Style Property Mapping

| LumaUI concept | Likely LVGL API family | Notes |
| --- | --- | --- |
| width / height | `lv_obj_set_width`, `lv_obj_set_height`, `lv_obj_set_size` | MVP supports integer pixels and percentages. Percent values map directly to LVGL sizing semantics relative to the parent content area. |
| padding | `lv_obj_set_style_pad_*` | Use explicit sides after semantic normalization. |
| margin subset | `lv_obj_set_style_margin_*` | Keep subset narrow and predictable. |
| background color | `lv_obj_set_style_bg_color` | Only where the widget supports background styling. |
| text color | `lv_obj_set_style_text_color` | Mainly for label-like text content. |
| radius | `lv_obj_set_style_radius` | Applies to container-like widgets. |
| border width | `lv_obj_set_style_border_width` | |
| border color | `lv_obj_set_style_border_color` | |
| font reference | `lv_obj_set_style_text_font` | Asset and declaration flow is future work. |

Style emission should operate on normalized semantic properties. The backend should not decide shorthand expansion, resolve conflicting declarations, or infer property validity on its own.


## Event Mapping

The ratified MVP event surface uses handler references, not inline code.

Current MVP contract:

- authored event attribute: `onPress` only
- authored value shape: quoted identifier, for example `onPress="open_settings"`
- no embedded expressions, argument lists, or inline code

Planned lowering shape:

- generate a named hook point per emitted widget that has an event reference
- register with `lv_obj_add_event_cb`
- keep business logic in user-authored C

Event mapping ownership is split deliberately:

- `semantic/` decides whether an event reference is valid and how it is represented canonically
- `backend/lvgl_c/` decides how that canonical event metadata becomes LVGL registration code
- `cli/` owns any operator-facing logging around event-enabled build paths

TODO:

- settle the generated callback signature contract
- decide whether handler symbol validation is compile-time strict or configurable
- capture the approved callback contract in feature decision material before relying on it as stable repository policy

## Binding Mapping

Bindings are explicitly out of scope for the ratified MVP slice.

Current rule:

- `bind`-style input is rejected during semantic validation
- the backend should emit no binding-related code paths in the current slice

Possible future direction:

Non-goals for v1:

- no hidden reactive engine
- no arbitrary expressions
- no template-time scripting

Possible backend patterns later:

- generated setter APIs
- generated view structs
- explicit user-driven update calls

Bindings remain a semantic and product-scope decision first. The backend should not invent reactive behavior to compensate for missing upstream contracts.

## Asset Mapping

Image and font support require a dedicated asset pass and should not be guessed in early backend code.

TODO:

- decide conversion responsibility boundaries
- define generated declarations for image/font assets
- define cache invalidation strategy for asset outputs
- treat asset-pipeline policy as decision-brief material before expanding backend support

## Backend Observability

Backend observability should help operators understand what the build emitted without changing emitted code content.

Rules:

- backend-stage logs describe file planning, emission progress, and mapping failures only when surfaced through the CLI
- mapping diagnostics stay actionable and deterministic
- generated `.c` and `.h` files remain free of progress logging or trace text
- any future verbose mode should expose additional backend detail through command output, not through generated-file comments added only for tracing

Observability changes that alter operator-facing command behavior should be kept in sync with decision briefs and CLI contract docs, not documented here in isolation.
