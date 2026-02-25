use leptos::*;
use leptos_router::use_location;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn NavigationProgress() -> impl IntoView {
    let location = use_location();
    let (progress, set_progress) = create_signal(0f64);
    let (visible, set_visible) = create_signal(false);

    create_effect(move |_| {
        let _ = location.pathname.clone();
        set_visible.set(true);
        set_progress.set(0.0);

        let set_to_seventy = Closure::wrap(Box::new({
            let set_progress = set_progress.clone();
            move || set_progress.set(70.0)
        }) as Box<dyn Fn()>);
        window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                set_to_seventy.as_ref().unchecked_ref(), 10).ok();
        set_to_seventy.forget();

        let finish = Closure::wrap(Box::new({
            let set_progress = set_progress.clone();
            move || set_progress.set(100.0)
        }) as Box<dyn Fn()>);
        window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                finish.as_ref().unchecked_ref(), 700).ok();
        finish.forget();
    });

    create_effect(move |_| {
        if progress.get() >= 100.0 {
            let hide = Closure::wrap(Box::new({
                let set_visible = set_visible.clone();
                let set_progress = set_progress.clone();
                move || { set_visible.set(false); set_progress.set(0.0); }
            }) as Box<dyn Fn()>);
            window().unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    hide.as_ref().unchecked_ref(), 300).ok();
            hide.forget();
        }
    });

    view! {
        <div>
            <Show when=move || visible.get()>
                <div class="fixed top-0 left-0 right-0 z-[9999] h-[3px] bg-transparent">
                    <div
                        class="h-full bg-gradient-to-r from-primary via-accent to-primary shadow-[0_0_12px_3px_hsla(var(--primary),0.5)] transition-all duration-200 ease-out"
                        style=move || format!("width: {}%; transition: width 20s linear;", progress.get())
                    />
                    <div
                        class="absolute top-0 h-full w-32 bg-gradient-to-l from-white/50 to-transparent animate-pulse"
                        style=move || {
                            if progress.get() < 100.0 {
                                format!("right: calc({}%); opacity: 1;", 100.0 - progress.get())
                            } else {
                                "right: 0; opacity: 0;".to_string()
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PageLoadingOverlay
//
// Terminal-aesthetic loading screen. Visually identical to the index.html
// pre-WASM loader — same triple rings, same terminal window, same .lo-* CSS.
//
// Always uses a dark opaque background so it is legible regardless of
// whether the app is in light or dark mode (same as the homepage loader).
//
// Props:
//   label — short identifier shown in the ring belly (e.g. "ANIME", "MANGA")
//   logs  — 5 log lines for the terminal window
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn PageLoadingOverlay(
    /// Short label inside the ring — keep under 8 chars, no spaces wrap badly.
    #[prop(optional)] label: Option<&'static str>,
    /// 5 terminal log lines displayed with CSS stagger.
    #[prop(optional)] logs: Option<[&'static str; 5]>,
) -> impl IntoView {
    let label_text = label.unwrap_or("LOADING");
    let log_lines = logs.unwrap_or([
        "[SYS] INITIALIZING NEURAL ENGINE",
        "[NET] ESTABLISHING UPLINK...",
        "[DB]  SYNCING DATA STREAM...",
        "[APP] MOUNTING MODULES...",
        "[OK]  PROTOCOL READY",
    ]);

    view! {
        // ── Full-screen overlay using lo-overlay (adaptive light/dark via main.css) ──
        <div class="lo-overlay fixed inset-0 z-[9998] flex flex-col items-center justify-center overflow-hidden font-mono transition-colors duration-300">

            // ── Ambient Orbs ──
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[40rem] h-[40rem] rounded-full blur-[150px] pointer-events-none bg-indigo-500/12" />
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[30rem] h-[30rem] rounded-full blur-[100px] pointer-events-none bg-purple-500/10 animate-pulse-slow" />

            // ── Holographic grid ──
            <div class="absolute inset-0 pointer-events-none opacity-[0.04] bg-[linear-gradient(rgba(99,102,241,0.25)_1px,transparent_1px),linear-gradient(90deg,rgba(99,102,241,0.25)_1px,transparent_1px)] bg-[size:40px_40px] [transform:perspective(500px)_rotateX(60deg)_translateY(-100px)_scale(2)]" />

            // ── Main interface ──
            <div class="relative z-10 flex flex-col items-center justify-center gap-12 w-full max-w-2xl px-6">

                // ── Triple concentric ring spinner ──
                <div class="relative flex items-center justify-center w-64 h-64 md:w-80 md:h-80">
                    // Outer — clockwise 4s
                    <div class="absolute inset-0 rounded-full border border-indigo-500/20 border-t-indigo-500 shadow-[0_0_30px_rgba(99,102,241,0.35)] animate-[spin_4s_linear_infinite]" />
                    // Mid — counter-clockwise 3s
                    <div class="absolute inset-6 rounded-full border border-purple-500/25 border-b-purple-400 border-l-purple-400 animate-[spin_3s_linear_infinite_reverse]" />
                    // Inner dashed — slow clockwise 8s
                    <div class="absolute inset-12 rounded-full border border-dashed border-blue-400/40 animate-[spin_8s_linear_infinite]" />

                    // ── Centre info ──
                    <div class="relative z-10 flex flex-col items-center justify-center text-center px-2">
                        // Short label — no wrapping
                        <div class="text-xs font-black tracking-[0.35em] mb-3 uppercase truncate max-w-[10rem] text-indigo-400">
                            {label_text}
                        </div>
                        // Animated cursor instead of broken "--" ligature
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-8 bg-indigo-400 animate-[pulse_1s_ease-in-out_infinite]" />
                            <div class="w-2 h-8 bg-purple-400 animate-[pulse_1s_ease-in-out_infinite_0.3s]" />
                            <div class="w-2 h-8 bg-blue-400 animate-[pulse_1s_ease-in-out_infinite_0.6s]" />
                        </div>
                        <span class="text-[9px] text-white/40 tracking-[0.3em] mt-3 uppercase">
                            "Loading..."
                        </span>
                    </div>
                </div>

                // ── Terminal window ──
                <div class="w-full max-w-lg border border-white/10 rounded-xl p-4 md:p-6 backdrop-blur-md shadow-2xl relative overflow-hidden bg-black/50">
                    // Traffic lights
                    <div class="flex items-center gap-2 mb-4">
                        <div class="w-2.5 h-2.5 rounded-full bg-red-500/60" />
                        <div class="w-2.5 h-2.5 rounded-full bg-yellow-500/60" />
                        <div class="w-2.5 h-2.5 rounded-full bg-green-500" />
                        <span class="ml-2 text-[8px] text-white/25 tracking-widest uppercase font-bold">
                            "Terminal_Access_v4.2"
                        </span>
                    </div>

                    // Log lines — CSS stagger via lo-log-1..5 in main.css
                    <div class="space-y-1.5 h-28 flex flex-col justify-end overflow-hidden mask-image-b-fade">
                        <div class="lo-log-line lo-log-1">
                            <span class="text-white/25 mr-3">"&gt;_"</span>{log_lines[0]}
                        </div>
                        <div class="lo-log-line lo-log-2">
                            <span class="text-white/25 mr-3">"&gt;_"</span>{log_lines[1]}
                        </div>
                        <div class="lo-log-line lo-log-3">
                            <span class="text-white/25 mr-3">"&gt;_"</span>{log_lines[2]}
                        </div>
                        <div class="lo-log-line lo-log-4">
                            <span class="text-white/25 mr-3">"&gt;_"</span>{log_lines[3]}
                        </div>
                        <div class="lo-log-line lo-log-5">
                            <span class="text-white/25 mr-3">"&gt;_"</span>{log_lines[4]}
                        </div>
                    </div>

                    // Scan line
                    <div class="absolute inset-0 pointer-events-none bg-gradient-to-b from-transparent via-indigo-500/5 to-transparent h-10 animate-scan" />
                </div>
            </div>

            // Scanlines texture overlay
            <div class="absolute inset-0 pointer-events-none scanlines opacity-10" />
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ContentSkeleton — shimmer skeleton for inline content areas
// ─────────────────────────────────────────────────────────────────────────────
#[component]
pub fn ContentSkeleton(lines: Option<usize>, class: Option<String>) -> impl IntoView {
    let lines = lines.unwrap_or(3);
    view! {
        <div class=move || class.clone().unwrap_or_default()>
            {(0..lines).map(|i| view! {
                <div class="h-4 shimmer rounded" style=move || format!("width: {}%;", 100 - i * 15) />
            }).collect_view()}
        </div>
    }
}
