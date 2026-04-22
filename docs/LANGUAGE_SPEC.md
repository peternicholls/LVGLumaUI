# Language Specification

**Language Spec Version**: `LS-0.2.0`
**Status**: first-slice ratified for the brownfield MVP; broader surface remains provisional
**Repository Release Version**: tracked separately in the root workspace version and `CHANGELOG.md`

This specification governs the authored language for Luma UI for LVGL, shortened to LumaUI.

Language-specification versioning is intentionally separate from application and workspace versioning. Language-contract history is tracked in `docs/LANGUAGE_CHANGELOG.md`.

## Status

The first slice of the LumaUI authored language is **ratified for the brownfield MVP**. Everything in [§ Ratified First Slice](#ratified-first-slice) is normative: the parser MUST accept it, the semantic layer MUST validate it, and the backend MUST emit deterministic LVGL 9.x output for it. Everything else listed in [§ Deferred Constructs](#deferred-constructs) and [§ Provisional Surface (Not Ratified)](#provisional-surface-not-ratified) remains provisional.

The language specification assumes clean stage ownership: syntax belongs to `parser/`, meaning and supported-surface decisions belong to `semantic/`, canonical representation belongs to `ir/`, and LVGL API mapping belongs to `backend/lvgl_c/`.

Anything not listed in the ratified slice requires a decision brief and developer sign-off before implementation.

## Versioning Policy

Use the `LS-MAJOR.MINOR.PATCH` format for the authored-language contract.

- `MAJOR` for incompatible changes to already-ratified grammar or contract meaning.
- `MINOR` for ratified surface expansion or major clarifications.
- `PATCH` for editorial clarifications.

`LS-0.2.0` ratifies the first slice described below. Any future expansion of the supported surface MUST advance `MINOR` and update `docs/LANGUAGE_CHANGELOG.md` in the same change.

## Ratified First Slice

### File Conventions

- `.lui` — markup (widget tree) source files.
- `.lus` — style source files.
- `lumaui.toml` — project configuration.

### Markup Grammar (XML-like)

```ebnf
markup_document := element*
element         := "<" Identifier attribute* "/>"
                 | "<" Identifier attribute* ">" element_content "</" Identifier ">"
attribute       := Identifier "=" StringLiteral
element_content := element*
```

Lexical rules:

- `Identifier` matches `[A-Za-z_][A-Za-z0-9_]*`.
- `StringLiteral` is a double-quoted run of characters with no embedded newlines and no escape processing in the first slice.
- Whitespace (` `, `\t`, `\r`, `\n`) is insignificant between tokens.
- `// ... \n` line comments are accepted between tokens; block comments are not ratified.

A closing tag's identifier MUST match the opening tag's identifier exactly.

### Markup Widget Set

The parser accepts any opening identifier syntactically; the semantic layer rejects everything outside this ratified set:

- `Screen` — exactly one per markup document, must be the document root.
- `Column`
- `Row`
- `Text`
- `Button`

Children rules:

- `Screen` MUST contain exactly one child element.
- `Column` and `Row` MAY contain zero or more ratified children.
- `Button` MAY contain zero or one ratified child (typically a `Text`).
- `Text` MUST NOT contain children.

### Markup Attribute Surface

Ratified attributes:

- `id="…"` — optional on every widget. Identifier-shaped value. MUST be unique across the compiled project.
- `class="…"` — optional on every widget. A single class name with the same identifier shape; multiple classes are NOT ratified.
- `text="…"` — required on `Text`. A non-empty string literal.
- `onPress="handler_name"` — optional on `Button`. Value MUST be an identifier (the handler symbol).

Any other attribute name is rejected at the semantic layer with a source-located diagnostic.

### Bindings (Explicitly Rejected)

`bind="…"` and any binding-shaped attribute MUST be rejected at the semantic layer with a diagnostic that names the construct and points at the deferred-bindings policy.

### Style Grammar

```ebnf
style_document := rule*
rule           := selector "{" declaration* "}"
selector       := class_selector | id_selector
class_selector := "." Identifier
id_selector    := "#" Identifier
declaration    := property ":" value ";"
property       := Identifier ("-" Identifier)*
value          := Number | HexColor
```

Lexical rules:

- `Identifier` and `//` line comments behave as in the markup grammar.
- `Number` matches `[0-9]+`. Negative numbers are not ratified.
- `HexColor` matches `#` followed by exactly 6 hex digits.

Selector combinators (descendant, child, multiple selectors per rule, pseudo-selectors, and tag selectors) are NOT ratified.

### Style Property Surface

Ratified properties and accepted value shapes:

| Property | Accepted value | Applies to |
| --- | --- | --- |
| `padding` | `Number` (pixels, all sides) | any widget |
| `background-color` | `HexColor` | any widget |
| `text-color` | `HexColor` | any widget (semantically meaningful only on `Text`) |
| `width` | `Number` (pixels) | any widget |
| `height` | `Number` (pixels) | any widget |

Any other property name is rejected at the semantic layer with a source-located diagnostic.

### Selector Application

Style rules apply by exact match:

- `.foo` applies to every widget whose `class` attribute equals `foo`.
- `#bar` applies to every widget whose `id` attribute equals `bar`.

When multiple ratified rules set the same property on the same widget, the rule that appears later in style document order wins. There is no specificity-based cascade in the first slice.

### Event References

The only ratified event attribute is `onPress` on `Button`. Its value is treated as a named handler reference. The compiler does not validate handler symbols; it lowers the reference to a generated `lv_obj_add_event_cb` registration with a TODO marker for the user-supplied callback.

## Deferred Constructs

Out of scope for `LS-0.2.0`. Adding any of them requires a decision brief and a `MINOR` bump.

- Widgets: `Container`, `Grid`, `Image`, `Card`, and any other kind not listed above.
- Multiple classes on one element, attribute selectors, descendant/child/sibling combinators, pseudo-selectors, tag selectors.
- Style properties beyond the ratified table above (margin, radius, borders, fonts, alignment, flex flow overrides, asset references).
- Bindings (`bind="…"`), reactive references, expression evaluation, templates.
- Event attributes other than `onPress` on `Button`.
- LVGL 8.x compatibility.

## Provisional Surface (Not Ratified)

Still under design for later language phases:

- richer style properties (margin, radius, borders, fonts, alignment)
- broader event surface
- bindings and view-model integration
- asset pipeline (images, fonts)
- preview/runtime integration

## Stage Ownership Recap

- `parser/` decides syntactic validity and reports source spans.
- `semantic/` decides whether parsed constructs are in the ratified slice, normalizes values, and lowers to IR.
- `ir/` is the canonical backend-facing model.
- `backend/lvgl_c/` maps canonical IR to LVGL 9.x C output.
- `cli/` orchestrates stages and presents diagnostics and progress.

## Determinism Guarantees

- Source files within a directory are processed in sorted filename order.
- Diagnostics are emitted in source-document order, then in source-position order within each document.
- Generated files use slugified, prefix-stable names; iteration order across IR collections preserves authored order.

## Configuration

```toml
project_name = "minimal"
lvgl_version = "9.x"
source_dir = "ui"
output_dir = "generated/ui"
```

`lvgl_version` MUST be `"9.x"` for `LS-0.2.0`.
