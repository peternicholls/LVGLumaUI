# Luma UI for LVGL 1.0.0

The first usable release of the declarative LVGL C compiler. Author `.lui`
widget trees and `.lus` styles, validate them, and generate deterministic C and
headers with `lumaui build`.

- Supports Screen, Column, Row, Text and Button, integer sizing and padding,
  hex colors, id/class selectors, and named button callbacks.
- Includes native CLI archives, SHA-256 checksums, installation instructions,
  an authored example, and a working firmware callback.
- Declares callbacks in generated headers and preserves explicit user-owned
  regions when regenerating C.
- Rejects empty projects, nested screens, invalid configuration/symbol prefixes,
  overflowing dimensions and colliding output names.

Extract the archive matching your OS/CPU and put `lumaui` (`lumaui.exe` on
Windows) on PATH. Run `lumaui init my-ui`, then `lumaui build my-ui`. See the
included `docs/USAGE.md` for LVGL integration and source installation.

Publishing requires Rust tests and package smoke checks on Linux, Windows and
macOS, plus compilation and a headless runtime check against LVGL 9.2.2.

Scope is the implemented LS-0.2.0 compiler slice. Preview, percentage sizes,
multiple classes, bindings, images, grids and richer styling remain deferred.
Firmware supplies LVGL, device drivers and event logic. Internal Rust crate
APIs are not a supported public API. Linux binaries require glibc 2.35+; macOS
binaries are unsigned. No hardware-device validation is claimed.
