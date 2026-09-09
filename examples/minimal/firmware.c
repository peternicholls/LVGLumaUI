/* User-owned firmware code. LumaUI never rewrites this file. */
#include "home_gen.h"

void minimal_event_open_settings(lv_event_t *e) {
    lv_obj_t *button = lv_event_get_target(e);
    lv_label_set_text(lv_obj_get_child(button, 0), "Settings opened");
}
