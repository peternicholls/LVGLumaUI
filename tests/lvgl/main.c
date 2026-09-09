#include "home_gen.h"
#include "layout_gen.h"
#include <assert.h>
#include <string.h>

int main(void) {
    lv_init();
    lv_display_t *display = lv_display_create(480, 320);
    assert(display != NULL);
    lv_obj_t *screen = minimal_screen_home_create(NULL);
    assert(screen != NULL && lv_obj_get_parent(screen) == NULL);
    lv_screen_load(screen);
    assert(lv_obj_get_child_count(screen) == 1);
    lv_obj_t *column = lv_obj_get_child(screen, 0);
    assert(lv_obj_get_child_count(column) == 2);
    assert(lv_obj_get_style_pad_left(column, LV_PART_MAIN) == 16);
    assert(lv_obj_get_style_bg_opa(column, LV_PART_MAIN) == LV_OPA_COVER);
    assert(strcmp(lv_label_get_text(lv_obj_get_child(column, 0)), "Hello LumaUI") == 0);
    lv_obj_t *button = lv_obj_get_child(column, 1);
    lv_obj_send_event(button, LV_EVENT_CLICKED, NULL);
    assert(strcmp(lv_label_get_text(lv_obj_get_child(button, 0)), "Settings opened") == 0);
    lv_obj_t *layout = layout_screen_layout_create(NULL);
    lv_obj_t *row = lv_obj_get_child(layout, 0);
    assert(lv_obj_get_style_width(row, LV_PART_MAIN) == 320);
    assert(lv_obj_get_style_height(row, LV_PART_MAIN) == 180);
    assert(lv_obj_get_style_width(lv_obj_get_child(row, 0), LV_PART_MAIN) == 120);
    assert(lv_obj_get_child_count(lv_obj_get_child(row, 0)) == 2);
    lv_display_delete(display);
    lv_deinit();
    return 0;
}
