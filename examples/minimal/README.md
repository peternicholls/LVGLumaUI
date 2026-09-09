# Minimal example — normative

```sh
lumaui validate examples/minimal
lumaui build examples/minimal
```

Compile `generated/ui/screens/home_gen.c` and `firmware.c` with LVGL 9.x, adding
the generated screens directory to your include path. After setting up LVGL and
your display, call `minimal_screen_home_create(NULL)` and `lv_screen_load`.

`firmware.c` implements the generated callback and changes the button label
when clicked. The real LVGL smoke test exercises this behavior. This example
is an exact frontend snapshot fixture; change its generated contract and tests
together. See `docs/USAGE.md` for installation and firmware integration.
