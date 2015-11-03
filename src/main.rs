#![crate_type="staticlib"]
#![feature(intrinsics)]
#![feature(no_std)]
#![feature(lang_items)]
#![feature(link_args)]

#![no_std]

extern crate pebble;

use pebble::types::{GPoint,GSize,GRect,ClickRecognizer,Window,WindowHandlers};
use pebble::types;
use pebble::raw;
use pebble::Layer;

extern fn select_handler(_: *mut ClickRecognizer, layer: *mut types::TextLayer) {
    let text_layer = pebble::TextLayer::new_from(layer);
    text_layer.set_text("Rust is running!\0");
}

extern fn click_config_provider(_: *mut types::TextLayer) {
    raw::window_single_click_subscribe(2, select_handler);
}

extern fn window_load_handler(window: *mut Window) {
    let window_layer = raw::window_get_root_layer(window);
    let window_bounds = raw::layer_get_bounds(window_layer);

    let bitmap = pebble::Bitmap::new(1);

    let bitmap_layer = pebble::BitmapLayer::new(window_bounds);

    bitmap_layer.set_bitmap(&bitmap);
    bitmap_layer.set_compositing_mode(types::GCompOp::GCompOpAssign);

    raw::layer_add_child(window_layer, bitmap_layer.get_layer());

    let text_bounds = GRect {
        origin: GPoint { x: 0, y: 100 },
        size: GSize { w: window_bounds.size.w, h: 20 }
    };
    
    let text_layer = pebble::TextLayer::new(text_bounds);

    text_layer.set_text("Press a button\0");

    raw::layer_add_child(window_layer, text_layer.get_layer());
    raw::window_set_click_config_provider_with_context(window, click_config_provider,
                                                       text_layer.get_raw()); 
}

extern fn window_unload_handler(window: *mut Window) {
}
extern fn window_appear_handler(window: *mut Window) {
}
extern fn window_disappear_handler(window: *mut Window) {
}

fn init() -> *mut Window{
    let window = raw::window_create();

    raw::window_set_window_handlers(window, WindowHandlers {
        load: window_load_handler,
        appear: window_appear_handler,
        disappear: window_disappear_handler,
        unload: window_unload_handler,
    });

    raw::window_stack_push(window, false);
    window
}

fn deinit(window: *mut Window) {
    raw::window_destroy(window);
}

#[no_mangle]
pub extern fn main() {
    let window = init();
    raw::app_event_loop();
    deinit(window);
}
