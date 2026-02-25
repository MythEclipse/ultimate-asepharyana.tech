use apps_leptos::AppRoot;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <AppRoot />
        }
    });

    // Dispatch app-ready after a tick so Leptos reactive graph is settled.
    // index.html overlay listens for this to start its hide sequence.
    // We intentionally keep the delay at 0 — the overlay's own 500ms +
    // 1000ms fade handles the visual transition.
    if let Some(window) = web_sys::window() {
        let window_clone = window.clone();
        let cb = Closure::once(move || {
            let _ = js_sys::Reflect::set(
                &window_clone,
                &JsValue::from_str("__LEPTOS_READY"),
                &JsValue::from_bool(true),
            );
            let _ = window_clone
                .dispatch_event(&web_sys::Event::new("app-ready").unwrap());
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            0,
        );
        cb.forget();
    }
}
