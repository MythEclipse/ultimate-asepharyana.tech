use leptos::*;
use std::rc::Rc;

#[component]
pub fn CinematicIntro(on_complete: Rc<dyn Fn()>) -> impl IntoView {
    let (is_ready, set_is_ready) = create_signal(false);
    let (fade_out, set_fade_out) = create_signal(false);

    // Listen for Bevy's Readiness Signal
    {
        let on_complete = on_complete.clone();
        create_effect(move |_| {
            let _on_complete = on_complete.clone();
            
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::JsCast;

                let handle_message = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
                    if let Some(msg) = ev.data().as_string() {
                        if msg == "PROTOCOL_READY" {
                            set_is_ready.set(true);
                        }
                    }
                }) as Box<dyn FnMut(web_sys::MessageEvent)>);

                window()
                    .add_event_listener_with_callback("message", handle_message.as_ref().unchecked_ref())
                    .unwrap();
                
                handle_message.forget(); // Keep the listener alive
            }

            // Fallback for asset readiness (if WASM fails or takes too long)
            set_timeout(
                move || {
                    if !is_ready.get_untracked() {
                        set_is_ready.set(true);
                    }
                },
                std::time::Duration::from_millis(8000),
            );
        });
    }

    create_effect({
        let on_complete = on_complete.clone();
        move |_| {
            if is_ready.get() {
                let fade_timer = gloo_timers::callback::Timeout::new(9000, move || {
                    set_fade_out.set(true);
                });

                let complete_timer = gloo_timers::callback::Timeout::new(10500, {
                    let on_complete = on_complete.clone();
                    move || {
                        on_complete();
                    }
                });

                on_cleanup(move || {
                    drop(fade_timer);
                    drop(complete_timer);
                });
            }
        }
    });

    view! {
        <div class=move || {
            let mut base = "fixed inset-0 z-[100] flex flex-col items-center justify-center bg-black transition-opacity duration-1500 ".to_string();
            if fade_out.get() {
                base.push_str("opacity-0 pointer-events-none");
            }
            base
        }>
            // Technical Loading Interface
            <Show
                when=move || is_ready.get()
                fallback=|| view! {
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
                            <div class="flex justify-between text-[7px] tracking-[0.1em] text-white/40">
                                <span>"SHADERS"</span>
                                <span class="text-emerald-500">"OPTIMIZED"</span>
                            </div>
                            <div class="flex justify-between text-[7px] tracking-[0.1em] text-white/40">
                                <span>"WASM_BUFFER"</span>
                                <span class="animate-pulse">"MOUNTING..."</span>
                            </div>
                            <div class="flex justify-between text-[7px] tracking-[0.1em] text-white/40">
                                <span>"SPATIAL_SYNC"</span>
                                <span class="text-indigo-400">"WAITING_HANDSHAKE"</span>
                            </div>
                        </div>
                    </div>
                }
            >
                // Bevy 3D Solar System (Intro Stage)
                <div class="absolute inset-0 z-0 animate-fade-in transition-opacity duration-1000">
                    <iframe 
                        src="http://localhost:3001/" 
                        class="w-full h-full border-0 brightness-110"
                        title="3D Cinematic Visuals"
                    />
                </div>
            </Show>

            // Scanlines Overlay
            <div class="absolute inset-0 pointer-events-none scanlines opacity-5" />
            
        </div>
    }
}
