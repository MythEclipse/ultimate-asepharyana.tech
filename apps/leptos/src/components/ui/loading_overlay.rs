use leptos::*;

#[component]
pub fn LoadingOverlay(#[prop(into)] is_ready: Signal<bool>) -> impl IntoView {
    let initial_ready = is_ready.get_untracked();
    let (fade_out, set_fade_out) = create_signal(initial_ready);
    let (hidden, set_hidden) = create_signal(initial_ready);

    create_effect(move |_| {
        if is_ready.get() && !hidden.get_untracked() {
            // Start fade out after a short delay to ensure smooth transition
            let fade_timer = gloo_timers::callback::Timeout::new(500, move || {
                set_fade_out.set(true);
            });
            
            // Hide completely after animation
            let hide_timer = gloo_timers::callback::Timeout::new(2000, move || {
                set_hidden.set(true);
            });

            on_cleanup(move || {
                drop(fade_timer);
                drop(hide_timer);
            });
        }
    });

    view! {
        <Show when=move || !hidden.get()>
            <div class=move || {
                let mut base = "fixed inset-0 z-[100] flex flex-col items-center justify-center bg-black transition-opacity duration-1500 ".to_string();
                if fade_out.get() {
                    base.push_str("opacity-0 pointer-events-none");
                }
                base
            }>
                // Technical Loading Interface
                <div class="w-full max-w-md px-12 flex flex-col gap-6 font-mono">
                    // Vertical Scanning Line
                    <div class="absolute inset-0 z-50 pointer-events-none bg-gradient-to-b from-transparent via-indigo-500/10 to-transparent h-20 animate-scan" />
                    
                    // Main Progress Header
                    <div class="flex justify-between items-end border-b border-indigo-500/20 pb-2">
                        <span class="text-[10px] text-indigo-400 font-black tracking-[0.3em]">"INITIALIZING_CORE"</span>
                        <span class="text-[8px] text-indigo-500/50">"v4.0.2"</span>
                    </div>

                    // Animated Progress Bar (Multi-Segment)
                    <div class="flex gap-1.5 h-1.5">
                        { (0..15).map(|i| view! {
                            <div 
                                class="flex-1 bg-indigo-500/20 rounded-sm animate-pulse-segments"
                                style=format!("animation-delay: {}s", i as f32 * 0.1)
                            />
                        }).collect_view() }
                    </div>

                    // Technical Readouts
                    <div class="flex flex-col gap-1">
                        <div class="flex justify-between text-[7px] tracking-[0.1em] text-indigo-300/40">
                            <span>"SHADERS"</span>
                            <span class="text-cyan-400">"OPTIMIZED"</span>
                        </div>
                        <div class="flex justify-between text-[7px] tracking-[0.1em] text-indigo-300/40">
                            <span>"WASM_BUFFER"</span>
                            <span class="animate-pulse">"MOUNTING..."</span>
                        </div>
                        <div class="flex justify-between text-[7px] tracking-[0.1em] text-indigo-300/40">
                            <span>"SPATIAL_SYNC"</span>
                            <span class="text-indigo-400">"WAITING_HANDSHAKE"</span>
                        </div>
                    </div>
                </div>

                // Scanlines Overlay
                <div class="absolute inset-0 pointer-events-none scanlines opacity-5" />
            </div>
        </Show>
    }
}
