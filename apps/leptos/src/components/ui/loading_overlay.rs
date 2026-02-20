use leptos::*;

#[component]
pub fn LoadingOverlay(#[prop(into)] is_ready: Signal<bool>) -> impl IntoView {
    let initial_ready = is_ready.get_untracked();
    let (fade_out, set_fade_out) = create_signal(initial_ready);
    let (hidden, set_hidden) = create_signal(initial_ready);

    let (progress, set_progress) = create_signal(0.0);
    let (logs, set_logs) = create_signal(vec!["[SYS] INITIALIZING NEURAL ENGINE".to_string()]);

    create_effect(move |_| {
        if is_ready.get() && !hidden.get_untracked() {
            set_progress.set(100.0);
            set_logs.update(|l| {
                l.push("[SYS] NEURAL LINK ESTABLISHED".to_string());
                l.push("PROTOCOL_READY // ENTERING MAINFRAME...".to_string());
                if l.len() > 6 { l.remove(0); }
            });

            // Start fade out after a short delay
            let fade_timer = gloo_timers::callback::Timeout::new(800, move || {
                set_fade_out.set(true);
            });
            
            // Hide completely after animation
            let hide_timer = gloo_timers::callback::Timeout::new(2300, move || {
                set_hidden.set(true);
            });

            on_cleanup(move || {
                drop(fade_timer);
                drop(hide_timer);
            });
        }
    });

    create_effect(move |_| {
        if !is_ready.get() && !hidden.get_untracked() {
             let interval = gloo_timers::callback::Interval::new(50, move || {
                 set_progress.update(|p| {
                     if *p < 99.0 {
                         *p += (100.0 - *p) * 0.05 + 0.1; 
                         if *p > 99.9 { *p = 99.9; }
                     }
                 });
                 if js_sys::Math::random() > 0.6 {
                     set_logs.update(|l| {
                         let hex = format!("0x{:08X}", (js_sys::Math::random() * 4294967295.0) as u32);
                         let modules = ["RENDER", "AUDIO", "PHYSX", "WASM", "NET", "CACHE"];
                         let rand_mod = modules[(js_sys::Math::random() * modules.len() as f64) as usize];
                         l.push(format!("LOAD_MDL [ {} ] // {} ... OK", rand_mod, hex));
                         if l.len() > 6 {
                             l.remove(0);
                         }
                     });
                 }
             });
             on_cleanup(move || drop(interval));
        }
    });

    view! {
        <Show when=move || !hidden.get()>
            <div class=move || {
                let mut base = "fixed inset-0 z-[100] flex flex-col items-center justify-center bg-[#030305] transition-opacity duration-1500 overflow-hidden font-mono ".to_string();
                if fade_out.get() {
                    base.push_str("opacity-0 pointer-events-none");
                }
                base
            }>
                // Ambient Backglow
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[40rem] h-[40rem] bg-indigo-500/10 rounded-full blur-[150px] pointer-events-none" />
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[30rem] h-[30rem] bg-purple-500/10 rounded-full blur-[100px] pointer-events-none animate-pulse-slow" />
                
                // Holographic Grid Background
                <div class="absolute inset-0 pointer-events-none opacity-[0.03] bg-[linear-gradient(rgba(255,255,255,0.1)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.1)_1px,transparent_1px)] bg-[size:40px_40px] [transform:perspective(500px)_rotateX(60deg)_translateY(-100px)_scale(2)]" />

                // Main Central Interface
                <div class="relative z-10 flex flex-col items-center justify-center gap-12 w-full max-w-2xl px-6">
                    
                    // The "Core" - Rotating Geometric Elements
                    <div class="relative flex items-center justify-center w-64 h-64 md:w-80 md:h-80">
                        // Outer Ring
                        <div class="absolute inset-0 rounded-full border border-indigo-500/20 border-t-indigo-500 shadow-[0_0_30px_rgba(99,102,241,0.2)] animate-[spin_4s_linear_infinite]" />
                        // Middle Ring Reverse
                        <div class="absolute inset-6 rounded-full border border-purple-500/20 border-b-purple-500 border-l-purple-500 animate-[spin_3s_linear_infinite_reverse]" />
                        // Inner Dashed Ring
                        <div class="absolute inset-12 rounded-full border border-dashed border-blue-400/30 animate-[spin_8s_linear_infinite]" />
                        
                        // Central Data
                        <div class="relative z-10 flex flex-col items-center justify-center">
                            <crate::components::ui::GlitchText 
                                text="ASEP.SYS".to_string() 
                                class="text-sm font-black text-indigo-400 tracking-[0.4em] mb-2".to_string() 
                            />
                            <div class="text-6xl md:text-7xl font-black text-white tracking-tighter drop-shadow-[0_0_15px_rgba(255,255,255,0.5)]">
                                {move || format!("{:02.0}", progress.get())}<span class="text-3xl text-indigo-500/80">"%"</span>
                            </div>
                            <span class="text-[9px] text-muted-foreground tracking-[0.3em] mt-2 uppercase">
                                {move || if is_ready.get() { "Sequence Complete" } else { "System Booting" }}
                            </span>
                        </div>
                    </div>

                    // Terminal Output Window
                    <div class="w-full max-w-lg bg-black/40 border border-white/10 rounded-xl p-4 md:p-6 backdrop-blur-md shadow-2xl relative overflow-hidden group">
                        <div class="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-indigo-500 to-transparent opacity-50" />
                        
                        <div class="flex items-center gap-2 mb-4">
                            <div class="w-2 h-2 rounded-full bg-red-500/50" />
                            <div class="w-2 h-2 rounded-full bg-yellow-500/50" />
                            <div class="w-2 h-2 rounded-full bg-green-500" />
                            <span class="ml-2 text-[8px] text-white/30 tracking-widest uppercase font-bold">"Terminal_Access_v4.2"</span>
                        </div>

                        // Log Stream
                        <div class="space-y-1.5 h-32 flex flex-col justify-end overflow-hidden mask-image-b-fade">
                            {move || logs.get().into_iter().enumerate().map(|(i, log)| {
                                let opacity = 1.0 - ((logs.get().len() - i - 1) as f32 * 0.15);
                                view! {
                                    <div 
                                        class="text-[10px] md:text-xs font-medium tracking-wider animate-slide-up"
                                        style=format!("opacity: {}; color: {}", opacity, if log.contains("OK") { "#4ade80" } else if log.contains("ERROR") { "#f87171" } else { "#818cf8" })
                                    >
                                        <span class="text-white/30 mr-3">{">_"}</span>
                                        {log}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                        
                        // Scanline over terminal
                        <div class="absolute inset-0 pointer-events-none bg-gradient-to-b from-transparent via-indigo-500/5 to-transparent h-10 animate-scan" />
                    </div>
                </div>

                // Screen Glitches & Scanlines
                <div class="absolute inset-0 pointer-events-none scanlines opacity-10" />
                <div class="absolute inset-0 pointer-events-none opacity-[0.02] mix-blend-overlay bg-[url('https://grainy-gradients.vercel.app/noise.svg')]" />
            </div>
        </Show>
    }
}
