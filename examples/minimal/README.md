# Minimal Example

Status: **normative** for the ratified first slice.

This example is the canonical authored-source input the compiler is expected to validate and build successfully. Every construct used here is part of the ratified first slice.

## Support Status

- `lumaui validate examples/minimal` MUST succeed.
- `lumaui build examples/minimal` MUST succeed and emit deterministic LVGL C under `examples/minimal/generated/ui/`.

## Ratified Constructs

Markup:

- `Screen`, `Column`, `Row`, `Text`, `Button`
- `id="..."` and `class="..."` attributes
- `text="..."` literal on `Text`
- Named event references on `Button` via `onPress="handler_name"`

Styles:

- Class selector (`.name`) and id selector (`#name`)
- Properties: `padding`, `background-color`, `text-color`, `width`, `height`

## Deferred Constructs

These remain out of scope for the first slice:

- Widgets: `Container`, `Grid`, `Image`, `Card`
- Bindings: `bind="..."` and any reactive value reference
- Style properties beyond the ratified set (e.g. `border-radius`, `margin`, fonts)
- Selector combinators, pseudo selectors, and tag selectors
