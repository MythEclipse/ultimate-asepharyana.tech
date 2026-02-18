use leptos::*;
use leptos_router::use_location;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn NavigationProgress() -> impl IntoView {
    // Reactive location to detect route changes
    let location = use_location();

    let (progress, set_progress) = create_signal(0f64);
    let (visible, set_visible) = create_signal(false);

    // Start simulated progress on route change
    create_effect(move |_| {
        // access pathname so this effect runs on navigation
        let _ = location.pathname.clone();

        set_visible.set(true);
        set_progress.set(0.0);

        // small delayed set to trigger CSS transition to a long-running partial width
        let set_to_seventy = Closure::wrap(Box::new({
            let set_progress = set_progress.clone();
            move || set_progress.set(70.0)
        }) as Box<dyn Fn()>);
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                set_to_seventy.as_ref().unchecked_ref(),
                10,
            )
            .ok();
        // keep closure alive until it runs
        set_to_seventy.forget();

        // finish the bar after a short delay (simulate completion)
        let finish = Closure::wrap(Box::new({
            let set_progress = set_progress.clone();
            move || set_progress.set(100.0)
        }) as Box<dyn Fn()>);
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                finish.as_ref().unchecked_ref(),
                700,
            )
            .ok();
        finish.forget();
    });

    // Hide and reset after it reaches 100%
    create_effect(move |_| {
        if progress.get() >= 100.0 {
            let hide = Closure::wrap(Box::new({
                let set_visible = set_visible.clone();
                let set_progress = set_progress.clone();
                move || {
                    set_visible.set(false);
                    set_progress.set(0.0);
                }
            }) as Box<dyn Fn()>);

            window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(hide.as_ref().unchecked_ref(), 300)
                .ok();
            hide.forget();
        }
    });

    view! {
        <div>
            <Show when=move || visible.get()>
                <div class="fixed top-0 left-0 right-0 z-[9999] h-1 bg-transparent">
                    <div
                        class="h-full bg-gradient-to-r from-primary via-accent to-primary shadow-lg shadow-primary/50 transition-all duration-200 ease-out"
                        style=move || format!("width: {}%; transition: width 20s linear;", progress.get())
                    />
                    <div
                        class="absolute top-0 right-0 h-full w-24 bg-gradient-to-l from-white/30 to-transparent animate-pulse"
                        style=move || {
                            let opacity = if progress.get() < 100.0 { 1 } else { 0 };
                            let translate = if progress.get() < 100.0 { "0" } else { "100%" };
                            format!("transform: translateX({}); opacity: {};", translate, opacity)
                        }
                    />
                </div>
            </Show>
        </div>
    }
}


// --- additional helpers ported from Solid version ---

#[component]
pub fn PageLoadingOverlay() -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-[9998] bg-background/80 backdrop-blur-sm flex items-center justify-center">
            <div class="flex flex-col items-center gap-4">
                <div class="relative">
                    <div class="w-16 h-16 rounded-full border-4 border-primary/20 border-t-primary animate-spin" />
                    <div class="absolute inset-0 w-16 h-16 rounded-full bg-gradient-to-r from-primary/0 via-primary/10 to-primary/0 animate-pulse" />
                </div>
                <div class="text-muted-foreground font-medium animate-pulse">"Loading..."</div>
            </div>
        </div>
    }
}

#[component]
pub fn ContentSkeleton(lines: Option<usize>, class: Option<String>) -> impl IntoView {
    let lines = lines.unwrap_or(3);
    view! {
        <div class=move || class.clone().unwrap_or_default()>
            { (0..lines).map(|i| view! { <div class="h-4 shimmer rounded" style=move || format!("width: {}%;", 100 - i * 15) /> }).collect_view() }
        </div>
    }
}
