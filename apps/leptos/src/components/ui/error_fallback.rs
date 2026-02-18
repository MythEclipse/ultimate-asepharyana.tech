use leptos::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn ErrorFallback(error: String) -> impl IntoView {
    let (countdown, set_countdown) = create_signal(5i32);

    // avoid moving `error` into closures: clone and compute boolean once
    let error_text = error.clone();
    let low = error_text.to_lowercase();
    let is_api_error = low.contains("fetch")
        || low.contains("network")
        || low.contains("http")
        || low.contains("api")
        || low.contains("cors")
        || low.contains("timeout")
        || low.contains("failed to fetch");

    // countdown + auto-reload
    create_effect(move |_| {
        let win = window().expect("no window");

        let cb = Closure::wrap(Box::new(move || {
            set_countdown.update(|c| {
                if *c <= 1 {
                    // attempt reload
                    let _ = web_sys::window().and_then(|w| w.location().reload().ok());
                    *c = 0;
                } else {
                    *c -= 1;
                }
            });
        }) as Box<dyn Fn()>);

        let id = win
            .set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 1000)
            .unwrap_or(-1);

        // keep closure alive (will be cleaned when page unloads)
        cb.forget();

        on_cleanup(move || {
            if id != -1 {
                if let Some(w) = web_sys::window() { w.clear_interval_with_handle(id); }
            }
        });
    });

    view! {
        <div class="min-h-[50vh] flex items-center justify-center p-8">
            <div class=move || {
                if is_api_error {
                    "max-w-md w-full border rounded-xl p-6 text-center bg-orange-500/10 border-orange-500/30"
                } else {
                    "max-w-md w-full border rounded-xl p-6 text-center bg-destructive/10 border-destructive/20"
                }
            }>
                <div class=move || {
                    if is_api_error { "w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center bg-orange-500/20" }
                    else { "w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center bg-destructive/20" }
                }>
                    {move || if is_api_error {
                        view! {
                            <svg class="w-8 h-8 text-orange-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0"/>
                            </svg>
                        }
                    } else {
                        view! {
                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                            </svg>
                        }
                    }
                    }
                </div>

                <h2 class="text-xl font-semibold text-foreground mb-2">
                    {move || if is_api_error { "🔌 API Connection Error".to_string() } else { "Oops! Something went wrong".to_string() }}
                </h2>

                <p class="text-muted-foreground mb-2 text-sm">{error.clone()}</p>

                <Show when=move || is_api_error>
                    <p class="text-orange-500/80 mb-4 text-xs font-mono bg-orange-500/10 px-2 py-1 rounded">"Server may be temporarily unavailable"</p>
                </Show>

                <p class="text-muted-foreground mb-4 text-sm">
                    "Auto-refreshing in "
                    <span class="font-bold text-primary">{move || countdown.get()}</span>
                    "s..."
                </p>

                <div class="flex gap-3 justify-center">
                    <button
                        on:click=move |_| { let _ = web_sys::window().and_then(|w| w.location().reload().ok()); }
                        class="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition-opacity"
                    >
                        "Refresh Now"
                    </button>
                    <button
                        on:click=move |_| { let _ = web_sys::window().and_then(|w| w.location().set_href("/").ok()); }
                        class="px-4 py-2 bg-secondary text-secondary-foreground rounded-lg hover:opacity-90 transition-opacity"
                    >
                        "Go Home"
                    </button>
                </div>
            </div>
        </div>
    }
}
