# Fixtures

Shared fixtures used by parser, semantic, CLI, and backend tests. See `tests/README.md` for category rules.

## Normative

- `minimal_screen.lui` — first-slice markup; mirrors `examples/minimal/ui/screens/home.lui`.
- `minimal_theme.lus` — first-slice styles; mirrors `examples/minimal/ui/styles/theme.lus`.

## Expected-fail

- `unsupported_widget.lui` — uses `Card`, which is deferred from the first slice.
- `binding_reference.lui` — uses `bind="..."`, explicitly rejected for the first slice.
- `duplicate_ids.lui` — declares two widgets with the same `id`.
- `unsupported_selector.lus` — uses a descendant combinator selector outside the ratified surface.
- `unsupported_property.lus` — uses `border-radius`, which is deferred.

These fixtures are loaded by tests in `parser/`, `semantic/`, `cli/`, and `backend/lvgl_c/` to lock in failure behavior.
