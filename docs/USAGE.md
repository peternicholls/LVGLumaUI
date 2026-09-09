# Using LumaUI 1.0

LumaUI compiles text-authored widget trees and styles into LVGL 9 C. The compiler
runs on a development computer; generated code runs in your firmware. Rust is
not required on the device.

## Install

Download the archive for your OS/CPU from the GitHub release, check its adjacent
SHA-256 file, extract it, and put `lumaui` (`lumaui.exe` on Windows) on PATH.
Linux packages require glibc 2.35+. macOS binaries are unsigned; source builds
are available where local security policy requires them.

Alternatively, install from the tagged source with stable Rust:

```sh
git clone --branch v1.0.0 https://github.com/peternicholls/LVGLumaUI.git
cd LVGLumaUI
cargo install --path cli --locked
lumaui --version
```

This release distributes the CLI and source. Internal Rust crates are not a
supported public API or separately published crates.io packages.

## Create and compile

```sh
lumaui init my-ui --name demo
lumaui doctor my-ui
lumaui validate my-ui
lumaui build my-ui
```

Commands default to the current directory when the path is omitted. `init`
refuses a nonempty destination unless `--force` is supplied. That flag overwrites
the starter config, screen and style; unrelated files remain untouched.

Configuration in `lumaui.toml`:

```toml
project_name = "demo"
lvgl_version = "9.x"
source_dir = "ui"
output_dir = "generated/ui"

[naming]
symbol_prefix = "lumaui_"
```

Place `.lui` files directly in `ui/screens` and `.lus` files directly in
`ui/styles`. Discovery is nonrecursive and sorted by filename. Each screen file
contains one `Screen` with exactly one child. `main.lui` produces
`screens/main_gen.c` and `main_gen.h` under `output_dir`. Colliding output slugs
are rejected before writing. Keep source and output directories separate.

```xml
<Screen>
  <Column class="root">
    <Text text="Hello"/>
    <Button onPress="continue_app"><Text text="Continue"/></Button>
  </Column>
</Screen>
```

```css
.root { padding: 16; width: 240; background-color: #20242b; }
```

See [LANGUAGE_SPEC.md](LANGUAGE_SPEC.md) for the complete surface: Screen,
Column, Row, Text and Button; one class per widget; id/class selectors; integer
width, height and padding; six-digit hex colors; and named `onPress` handlers
on Button. Later style declarations win, even over an earlier id rule.
Percentages, bindings, assets and preview remain deferred. Strings have no
escape or XML entity processing.

## Integrate with LVGL

Initialize LVGL and register display/input drivers first. Compile generated C
alongside your LVGL build, and add the generated `screens` directory to the
include path. Enable LVGL labels, buttons and flex layout.

```c
#include "main_gen.h"

void lumaui_event_continue_app(lv_event_t *e) {
    /* Your application logic. */
    (void)e;
}

void show_ui(void) {
    lv_obj_t *screen = lumaui_screen_main_create(NULL);
    lv_screen_load(screen);
}
```

Pass `NULL` to create a loadable screen. The existing non-NULL parent argument
embeds the tree in an LVGL object. Firmware owns object lifetimes and continues
calling LVGL's timer handler. Generated code does not set up hardware or start
an event loop.

`onPress="continue_app"` registers `lumaui_event_continue_app` for
`LV_EVENT_CLICKED` (release without scrolling). The configured prefix replaces
`lumaui_`. Headers declare exact signatures and C linkage for C++ callers.
Implement each callback in firmware; missing implementations fail at link time.
`examples/minimal/firmware.c` demonstrates updating the button label.

## Regeneration

Prefer separate firmware files for custom logic. Generated C also has one
`lumaui-region: user-owned` section preserved verbatim across builds. Keep the
marker lines intact and unique. Compiler-owned content is replaced; headers
are compiler-owned. Damaged markers and unrelated files at output paths cause
an error before any output is written.

Validation failures write no output. Collision and ownership checks precede
writing; disk/permission failures during writing may still leave a partial
build. Re-run after correcting the error. Renamed/deleted screens' old files
are not removed automatically: save any user-region code, then remove obsolete
generated pairs from your firmware build.

## Diagnostics and verification

Diagnostics and stage summaries go to stderr. Errors exit nonzero. `doctor`
checks config/discovery, not firmware libraries or hardware. Nesting is limited
to 128 widgets. Pixel values must fit LVGL's plain range (0–536870911); practical
display sizes are much smaller. Text cannot contain NUL.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
bash scripts/lumaui-phase-check.sh --require-build
```

With an LVGL 9.2.2 checkout, run the generated-C runtime smoke test:

```sh
cargo run -p lumaui-cli -- build examples/layout
cmake -S tests/lvgl -B target/lvgl-smoke -DLVGL_SOURCE_DIR=/absolute/path/to/lvgl
cmake --build target/lvgl-smoke --parallel 2
ctest --test-dir target/lvgl-smoke --output-on-failure
```

It creates a display and screen, checks widgets/styles, sends a click, and
verifies the user callback updates a label.
