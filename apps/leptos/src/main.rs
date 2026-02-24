use apps_leptos::AppRoot;
use leptos::*;
use wasm_bindgen::prelude::*;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <AppRoot />
        }
    });

    // Signal that the app is ready
    if let Some(window) = web_sys::window() {
        let _ = js_sys::Reflect::set(&window, &JsValue::from_str("__LEPTOS_READY"), &JsValue::from_bool(true));
        let _ = window.dispatch_event(&web_sys::Event::new("app-ready").unwrap());
    }
}
