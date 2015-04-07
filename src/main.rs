#![crate_type="staticlib"]
#![feature(intrinsics)]
#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![feature(link_args)]
#![no_std]

extern crate pebble;


use pebble::{GPoint,GSize,GRect,ClickRecognizer,TextLayer,Window,WindowHandlers};

extern fn select_handler(_: *mut ClickRecognizer, layer: *mut TextLayer) {
    pebble::text_layer_set_text(layer, "WOW IT WORKED!\0");
}

extern fn click_config_provider(_: *mut TextLayer) {
    pebble::window_single_click_subscribe(2, select_handler);
}

extern fn window_load_handler(window: *mut Window) {
    let window_layer = pebble::window_get_root_layer(window);
    let window_bounds = pebble::layer_get_bounds(window_layer);

    let text_bounds = GRect {
    origin: GPoint { x: 0, y: 72 },
    size: GSize { w: window_bounds.size.w, h: 20 }
    };
    let text_layer = pebble::text_layer_create(text_bounds);

    pebble::text_layer_set_text(text_layer, "Press a button\0");
    pebble::layer_add_child(window_layer, pebble::text_layer_get_layer(text_layer));

    pebble::window_set_click_config_provider_with_context(window, click_config_provider, text_layer); 
}

extern fn window_unload_handler(window: *mut Window) {
}
extern fn window_appear_handler(window: *mut Window) {
}
extern fn window_disappear_handler(window: *mut Window) {
}

fn init() -> *mut Window{
    let window = pebble::window_create();

    pebble::window_set_window_handlers(window, WindowHandlers {
        load: window_load_handler,
        appear: window_appear_handler,
        disappear: window_disappear_handler,
        unload: window_unload_handler,
    });

    pebble::window_stack_push(window, false);
    window
}

fn deinit(window: *mut Window) {
    pebble::window_destroy(window);
}

#[no_mangle]
pub extern fn main() {
    let window = init();
    pebble::app_event_loop();
    deinit(window);
}
