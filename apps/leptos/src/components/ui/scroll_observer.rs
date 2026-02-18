use leptos::*;
use leptos::html::Div;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use wasm_bindgen::prelude::*;

#[component]
pub fn ScrollObserver(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = "reveal-on-scroll".to_string(), into)] observe_class: String,
) -> impl IntoView {
    let element_ref = create_node_ref::<Div>();

    create_effect(move |_| {
        if let Some(el) = element_ref.get() {
            // Options for the observer
            let mut options = IntersectionObserverInit::new();
            options.root(None); // browser viewport
            options.root_margin("0px");
            options.threshold(&JsValue::from_f64(0.1)); // Trigger when 10% visible

            // Callback
            let callback = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _observer: IntersectionObserver| {
                for entry in entries {
                    let entry = IntersectionObserverEntry::from(entry);
                    if entry.is_intersecting() {
                        let target = entry.target();
                        let _ = target.class_list().add_1("is-visible");
                        // Optional: unobserve if we only want to trigger once
                        // observer.unobserve(&target); 
                    }
                }
            }) as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options
            ).expect("Failed to create IntersectionObserver");

            // Prevent closure from being dropped
            callback.forget();

            let _ = observer.observe(&el);
        }
    });

    view! {
        <div node_ref=element_ref class=format!("{} opacity-0 transition-all duration-1000 transform translate-y-8 {}", class, observe_class)>
            {children()}
        </div>
    }
}
