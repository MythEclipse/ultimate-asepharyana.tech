use leptos::*;
use leptos_meta::{Title, Meta};
use crate::components::logo::social_icons::{Instagram, LinkedIn, GitHub};
use std::sync::atomic::{AtomicBool, Ordering};

pub mod hero;
pub mod arsenal;
pub mod projects;

use crate::pages::home::hero::Hero;
use crate::pages::home::arsenal::Arsenal;
use crate::pages::home::projects::Projects;

static VISUALS_READY: AtomicBool = AtomicBool::new(false);

#[component]
pub fn HomePage() -> impl IntoView {
    let (visuals_ready, set_visuals_ready) = create_signal(VISUALS_READY.load(Ordering::Relaxed));

    // Listen for Bevy's Readiness Signal
    create_effect(move |_| {
        if visuals_ready.get_untracked() {
            return;
        }

        #[cfg(feature = "csr")]
        {
            use web_sys::window;
            if let Some(win) = window() {
                let is_mobile = win
                    .match_media("(max-width: 767px)")
                    .ok()
                    .flatten()
                    .map(|m| m.matches())
                    .unwrap_or(false);
                if is_mobile {
                    set_visuals_ready.set(true);
                    VISUALS_READY.store(true, Ordering::Relaxed);
                    return;
                }
            }
        }

        #[cfg(feature = "csr")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use web_sys::window;

            let handle_message = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
                if let Some(msg) = ev.data().as_string() {
                    if msg == "PROTOCOL_READY" {
                        set_visuals_ready.set(true);
                        VISUALS_READY.store(true, Ordering::Relaxed);
                    }
                }
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);

            window().unwrap()
                .add_event_listener_with_callback("message", handle_message.as_ref().unchecked_ref())
                .unwrap();

            handle_message.forget();
        }

        set_timeout(
            move || {
                if !visuals_ready.get_untracked() {
                    set_visuals_ready.set(true);
                    VISUALS_READY.store(true, Ordering::Relaxed);
                }
            },
            std::time::Duration::from_millis(8000),
        );
    });

    let visuals_url = option_env!("VISUALS_URL").unwrap_or("https://visuals.asepharyana.tech/");

    view! {
        <Title text="Full-Stack Developer | Asep Haryana"/>
        <Meta name="description" content="Asep Haryana - Full-Stack Developer specializing in Rust, WASM, and high-performance web applications."/>
        <Meta property="og:title" content="Asep Haryana | Full-Stack Developer"/>
        <Meta property="og:description" content="Crafting robust backend systems and immersive frontend solutions."/>
        <Meta name="twitter:card" content="summary_large_image"/>

        <main class="relative z-10 w-full overflow-hidden">
            <Hero visuals_url=visuals_url.to_string() />
            <Arsenal />
            <Projects />

            // Connection Section
            <section class="py-24 md:py-40 lg:py-56 px-6 overflow-hidden relative">
                <div class="absolute inset-0 pointer-events-none">
                    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full h-[1px] bg-gradient-to-r from-transparent via-primary/30 to-transparent" />
                </div>

                <div class="max-w-7xl mx-auto rounded-[3rem] md:rounded-[5rem] p-8 md:p-32 relative overflow-hidden glass border border-border/10 shadow-[0_80px_160px_rgba(0,0,0,0.2)] dark:shadow-[0_120px_250px_rgba(0,0,0,0.6)]">
                    <div class="absolute -right-60 -top-60 w-[50rem] h-[50rem] bg-primary/10 rounded-full blur-[180px] animate-tilt-slow opacity-60" />
                    <div class="absolute -left-60 -bottom-60 w-[50rem] h-[50rem] bg-accent/10 rounded-full blur-[180px] animate-tilt-reverse-slow opacity-40" />

                    <div class="relative z-10 flex flex-col items-center text-center space-y-20 md:space-y-24">
                        <div class="space-y-10 md:space-y-14">
                            <div class="space-y-8">
                                <span class="px-6 md:py-2.5 rounded-full glass-subtle text-[11px] font-black uppercase tracking-[0.6em] text-primary shadow-xl">
                                    "Communication"
                                </span>
                                <h2 class="text-5xl md:text-9xl font-black italic tracking-tighter leading-none uppercase">
                                    "Get In " <br/> <crate::components::ui::GlitchText text="Touch" class="text-primary" />
                                </h2>
                                <p class="text-lg md:text-3xl text-muted-foreground leading-relaxed max-w-2xl mx-auto font-medium italic tracking-tight">
                                    "I am always open to discussing new projects, creative ideas or professional opportunities."
                                </p>
                            </div>

                            <div class="flex flex-wrap items-center justify-center gap-6 md:gap-8">
                                <SocialLink href="https://github.com/MythEclipse" icon=view! { <GitHub/> } label="GitHub" />
                                <SocialLink href="https://www.linkedin.com/in/asep-haryana-saputra-2014a5294/" icon=view! { <LinkedIn/> } label="LinkedIn" />
                                <SocialLink href="https://www.instagram.com/asepharyana18/" icon=view! { <Instagram/> } label="Instagram" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn SocialLink(href: &'static str, icon: impl IntoView, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            class="group relative p-5 md:p-6 rounded-[1.5rem] md:rounded-3xl glass border border-border/10 hover:bg-muted font-display transition-all hover:scale-110 active:scale-95 shadow-xl"
        >
            <div class="absolute inset-0 bg-primary/10 rounded-3xl scale-0 group-hover:scale-110 transition-transform blur-xl" />
            <div class="relative z-10 w-6 h-6 flex items-center justify-center grayscale group-hover:grayscale-0 transition-all duration-500">
                {icon}
            </div>
            <span class="absolute -top-10 left-1/2 -translate-x-1/2 px-3 py-1 rounded-lg bg-primary text-[8px] font-black uppercase tracking-widest text-white opacity-0 group-hover:opacity-100 transition-all pointer-events-none">
                {label}
            </span>
        </a>
    }
}
