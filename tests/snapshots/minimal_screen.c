#include "home_gen.h"

lv_obj_t *lumaui_screen_home_create(void) {
    lv_obj_t *screen = lv_obj_create(NULL);
    lv_obj_t *root = lv_obj_create(screen);
    lv_obj_set_layout(root, LV_LAYOUT_FLEX);
    lv_obj_set_flex_flow(root, LV_FLEX_FLOW_COLUMN);
    lv_obj_t *title = lv_label_create(root);
    lv_label_set_text(title, "Hello LumaUI");
    lv_obj_t *open_settings = lv_button_create(root);
    lv_obj_t *text_4 = lv_label_create(open_settings);
    lv_label_set_text(text_4, "Settings");

    return screen;
}
