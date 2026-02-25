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
                </div>
            </Show>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PageLoadingOverlay — Premium terminal-aesthetic loading screen.
// Used as Suspense fallback for navigation between pages.
// Visually matches the index.html initial WASM loader.
// ─────────────────────────────────────────────────────────────────────────────
#[component]
pub fn PageLoadingOverlay(
    /// Short context label shown below the ring (e.g. "ANIME", "MANGA").
    #[prop(optional)] label: Option<&'static str>,
    /// 5 terminal log lines. Defaults to generic system messages.
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

    // Reactive counter 0 → 99 animating while overlay is shown.
    // Eases fast at first then slows (simulates real async load).
    let (counter, set_counter) = create_signal(0u32);

    create_effect(move |_| {
        // Reset on mount
        set_counter.set(0);

        // Tick every ~60ms → reaches ~99 in ~6s; stops at 99 until Suspense resolves.
        let progress_schedule: &[u32] = &[
            2, 5, 9, 14, 19, 24, 29, 34, 38, 43, 47, 51, 55, 59, 62,
            65, 68, 71, 74, 76, 78, 80, 82, 84, 86, 87, 88, 89, 90, 91,
            92, 93, 94, 95, 96, 97, 97, 98, 98, 99,
        ];
        let win = web_sys::window().unwrap();
        for (i, &val) in progress_schedule.iter().enumerate() {
            let set_c = set_counter.clone();
            let cb = Closure::once(move || set_c.set(val));
            let delay = (i as i32) * 120 + 50;
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(), delay);
            cb.forget();
        }
    });

    view! {
        // Full-screen overlay — adaptive light/dark via lo-overlay class
        <div class="lo-overlay fixed inset-0 z-[9998] flex flex-col items-center justify-center overflow-hidden font-mono transition-colors duration-300">

            // ── Ambient orbs ──
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[40rem] h-[40rem] rounded-full blur-[150px] pointer-events-none bg-indigo-500/[0.12]" />
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[30rem] h-[30rem] rounded-full blur-[100px] pointer-events-none bg-purple-500/10 animate-pulse-slow" />

            // ── Holographic grid ──
            <div class="absolute inset-0 pointer-events-none opacity-[0.04] bg-[linear-gradient(rgba(99,102,241,0.25)_1px,transparent_1px),linear-gradient(90deg,rgba(99,102,241,0.25)_1px,transparent_1px)] bg-[size:40px_40px] [transform:perspective(500px)_rotateX(60deg)_translateY(-100px)_scale(2)]" />

            <div class="relative z-10 flex flex-col items-center gap-10 w-full max-w-2xl px-6">

                // ── Triple concentric rings + counter ──
                <div class="relative flex items-center justify-center w-64 h-64 md:w-80 md:h-80">
                    // Outer — clockwise 4s + coloured glow
                    <div class="absolute inset-0 rounded-full border border-indigo-500/20 border-t-indigo-500 shadow-[0_0_40px_rgba(99,102,241,0.35)] animate-[spin_4s_linear_infinite]" />
                    // Mid — counter-clockwise 3s
                    <div class="absolute inset-[1.5rem] rounded-full border border-purple-500/25 border-b-purple-400 border-l-purple-400 shadow-[0_0_20px_rgba(168,85,247,0.25)] animate-[spin_3s_linear_infinite_reverse]" />
                    // Inner dashed — slow 8s
                    <div class="absolute inset-[3rem] rounded-full border border-dashed border-blue-400/35 animate-[spin_8s_linear_infinite]" />
                    // Innermost dot: pulsing core glow
                    <div class="absolute inset-[5.5rem] rounded-full bg-indigo-500/10 blur-md animate-pulse" />

                    // ── Centre — animated percentage counter ──
                    <div class="relative z-10 flex flex-col items-center justify-center pointer-events-none select-none">
                        // Big number (matches homepage 00%)
                        <div class="flex items-end leading-none drop-shadow-[0_0_20px_rgba(99,102,241,0.5)]">
                            <span class="text-[3.5rem] md:text-[4rem] font-black tracking-tighter text-white">
                                {move || format!("{:02}", counter.get())}
                            </span>
                            <span class="text-2xl font-black text-indigo-400/80 mb-1 ml-0.5">
                                "%"
                            </span>
                        </div>
                        // Status line below number
                        <span class="text-[8px] tracking-[0.35em] uppercase mt-2 text-white/35">
                            "Syncing..."
                        </span>
                    </div>
                </div>

                // ── Label badge — what page is loading ──
                <div class="flex flex-col items-center gap-3">
                    <div class="inline-flex items-center gap-2 px-5 py-2 rounded-full border border-indigo-500/30 bg-indigo-500/10 backdrop-blur-md">
                        <div class="w-1.5 h-1.5 rounded-full bg-indigo-400 animate-pulse" />
                        <span class="text-[10px] font-black tracking-[0.4em] uppercase text-indigo-300">
                            {label_text}
                        </span>
                    </div>
                </div>

                // ── Terminal window ──
                <div class="w-full max-w-lg border border-white/10 rounded-xl p-4 md:p-5 backdrop-blur-md shadow-2xl relative overflow-hidden bg-black/50">
                    // Traffic lights
                    <div class="flex items-center gap-2 mb-3">
                        <div class="w-2.5 h-2.5 rounded-full bg-red-500/60" />
                        <div class="w-2.5 h-2.5 rounded-full bg-yellow-500/60" />
                        <div class="w-2.5 h-2.5 rounded-full bg-green-500" />
                        <span class="ml-2 text-[8px] text-white/20 tracking-widest uppercase font-bold">
                            "Terminal_Access_v4.2"
                        </span>
                    </div>
                    // Log lines with CSS stagger
                    <div class="space-y-1 h-24 flex flex-col justify-end overflow-hidden mask-image-b-fade">
                        <div class="lo-log-line lo-log-1">
                            <span class="text-white/25 mr-2">"&gt;_"</span>{log_lines[0]}
                        </div>
                        <div class="lo-log-line lo-log-2">
                            <span class="text-white/25 mr-2">"&gt;_"</span>{log_lines[1]}
                        </div>
                        <div class="lo-log-line lo-log-3">
                            <span class="text-white/25 mr-2">"&gt;_"</span>{log_lines[2]}
                        </div>
                        <div class="lo-log-line lo-log-4">
                            <span class="text-white/25 mr-2">"&gt;_"</span>{log_lines[3]}
                        </div>
                        <div class="lo-log-line lo-log-5">
                            <span class="text-white/25 mr-2">"&gt;_"</span>{log_lines[4]}
                        </div>
                    </div>
                    // Scan line effect
                    <div class="absolute inset-0 pointer-events-none bg-gradient-to-b from-transparent via-indigo-500/5 to-transparent h-10 animate-scan" />
                </div>
            </div>

            // Scanlines texture
            <div class="absolute inset-0 pointer-events-none scanlines opacity-10" />
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ContentSkeleton — lightweight shimmer for inline content areas
// ─────────────────────────────────────────────────────────────────────────────
#[component]
pub fn ContentSkeleton(lines: Option<usize>, class: Option<String>) -> impl IntoView {
    let lines = lines.unwrap_or(3);
    view! {
        <div class=move || class.clone().unwrap_or_default()>
            {(0..lines).map(|i| view! {
                <div class="h-4 rounded animate-pulse bg-muted/60"
                     style=move || format!("width: {}%;", 90 - i * 15) />
            }).collect_view()}
        </div>
    }
}
