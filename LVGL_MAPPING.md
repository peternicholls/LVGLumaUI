# LVGL Mapping

## Scope

This document defines the intended mapping from LumaUI concepts to LVGL 9.x APIs. The goal is to keep mappings conservative, explicit, and auditable.

Where a mapping is not yet settled, it is called out as a TODO instead of being hand-waved.

## Target Baseline

First-pass planning assumes LVGL 9.x.

Version adapters for LVGL 8.x or other compatibility modes are future work.

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

## Layout Mapping

### Row and Column

Planned mapping:

- `lv_obj_set_layout(obj, LV_LAYOUT_FLEX)`
- `lv_obj_set_flex_flow(obj, ...)`
- `lv_obj_set_flex_align(obj, main, cross, track)` for supported align and justify subsets

The compiler should only expose align/justify values that can be mapped deterministically.

### Grid

Planned mapping:

- `lv_obj_set_layout(obj, LV_LAYOUT_GRID)`
- generated `lv_coord_t` row/column descriptor arrays
- explicit placement on child widgets

TODO:

- finalize the authored-language grid vocabulary before locking the backend surface
- decide whether the MVP allows only explicit track definitions or also named templates

## Style Property Mapping

| LumaUI concept | Likely LVGL API family | Notes |
| --- | --- | --- |
| width / height | `lv_obj_set_width`, `lv_obj_set_height`, `lv_obj_set_size` | Percent values require careful support policy. |
| padding | `lv_obj_set_style_pad_*` | Use explicit sides after semantic normalization. |
| margin subset | `lv_obj_set_style_margin_*` | Keep subset narrow and predictable. |
| background color | `lv_obj_set_style_bg_color` | Only where the widget supports background styling. |
| text color | `lv_obj_set_style_text_color` | Mainly for label-like text content. |
| radius | `lv_obj_set_style_radius` | Applies to container-like widgets. |
| border width | `lv_obj_set_style_border_width` | |
| border color | `lv_obj_set_style_border_color` | |
| font reference | `lv_obj_set_style_text_font` | Asset and declaration flow is future work. |

## Event Mapping

The language will use handler references, not inline code.

Planned lowering shape:

- generate a named hook point per emitted widget that has an event reference
- register with `lv_obj_add_event_cb`
- keep business logic in user-authored C

TODO:

- settle the generated callback signature contract
- decide whether handler symbol validation is compile-time strict or configurable

## Binding Mapping

Bindings are planned as symbolic references only.

Non-goals for v1:

- no hidden reactive engine
- no arbitrary expressions
- no template-time scripting

Possible backend patterns later:

- generated setter APIs
- generated view structs
- explicit user-driven update calls

## Asset Mapping

Image and font support require a dedicated asset pass and should not be guessed in early backend code.

TODO:

- decide conversion responsibility boundaries
- define generated declarations for image/font assets
- define cache invalidation strategy for asset outputs
