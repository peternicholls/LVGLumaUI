# Luma UI for LVGL

LumaUI is a declarative UI compiler that turns `.lui` widget trees and `.lus`
styles into deterministic, readable LVGL 9 C. Author interfaces as text, review
changes in Git, and compile the generated C into your firmware.

## Version 1.0.0

The supported compiler slice includes Screen, Column, Row, Text and Button;
id and single-class attributes; integer dimensions and padding; hex colors;
and named button callbacks. It follows the current [language contract](docs/LANGUAGE_SPEC.md).

Preview, grids, images, bindings, percentages, multiple classes and richer
styling remain deferred. LumaUI does not implement HTML/CSS or an on-device
scripting runtime. LVGL setup, hardware drivers and application logic belong to
your firmware.

## Get started

Download a native archive and checksum from [Releases](https://github.com/peternicholls/LVGLumaUI/releases),
extract it, and put `lumaui` (`lumaui.exe` on Windows) on PATH. Or install from
this checkout with stable Rust:

```sh
cargo install --path cli --locked
lumaui --version
lumaui init my-ui --name demo
lumaui validate my-ui
lumaui build my-ui
```

The build writes C and headers beneath `my-ui/generated/ui/screens`. Generated
headers declare the callbacks your firmware must implement. Explicit user-owned
regions in generated C are preserved on regeneration.

See [Usage and firmware integration](docs/USAGE.md) for configuration, commands,
installation details, callbacks, ownership rules and limits. The
[minimal example](examples/minimal) includes working user-written C; the
[layout example](examples/layout) exercises rows, dimensions and an empty button.
The dashboard example remains aspirational.

## Development

The Rust workspace preserves six responsibilities: shared infrastructure,
parser, semantic validation, canonical IR, LVGL C backend, and CLI orchestration.
See [Architecture](docs/ARCHITECTURE.md) and [Agent guidance](AGENTS.md).

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
bash scripts/lumaui-phase-check.sh --require-build
```

CI tests the compiler and release packages on Linux, Windows and macOS. It also
compiles generated C against LVGL 9.2.2 and executes a headless widget/style/click
smoke test. No hardware-device validation is claimed.

[Release notes](docs/RELEASE_1.0.0.md) · [Changelog](CHANGELOG.md) ·
[Next steps](docs/NEXT_STEPS.md) · [Versioning](docs/VERSIONING.md)

## License

MIT. See [LICENSE](LICENSE).
