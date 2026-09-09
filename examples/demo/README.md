# LumaUI 1.0.0 demonstration

From the repository root, run:

```sh
lumaui validate examples/demo
lumaui build examples/demo
```

This example displays a title and a Settings button. The generated C is checked in to demonstrate the compiler output and its user-owned callback region. The callback changes the button label to `Settings opened` when clicked in an LVGL application.

Change the title in `ui/screens/main.lui` and rebuild: generated widget code updates while the callback in the user-owned region is preserved. Firmware must supply LVGL, initialize a display, and load the screen returned by `lumaui_screen_main_create(NULL)`. There is no visual preview runner in 1.0.0.
