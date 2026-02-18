use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use crate::components::text::animated_header::{AnimatedHeader, WordItem};
use crate::components::logo::tech_icons::TECH_STACK;
use crate::components::logo::social_icons::{Instagram, Facebook, LinkedIn, GitHub};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let judul = vec![
        WordItem { text: "Asep", class: "text-blue-500 dark:text-blue-400" },
        WordItem { text: "Haryana", class: "text-purple-500 dark:text-purple-400" },
        WordItem { text: "Saputra", class: "text-pink-500 dark:text-pink-400" },
    ];

    view! {
        <Title text="Home | Asepharyana"/>
        <main class="min-h-screen bg-background text-foreground overflow-hidden">
            // Animated background orbs
            <div class="fixed inset-0 -z-10 overflow-hidden">
                <div class="absolute top-[-20%] left-[-10%] w-[600px] h-[600px] bg-blue-500/10 rounded-full blur-3xl animate-float-slow" />
                <div class="absolute bottom-[-20%] right-[-10%] w-[500px] h-[500px] bg-purple-500/10 rounded-full blur-3xl animate-float-medium" />
                <div class="absolute top-1/2 left-1/2 w-[400px] h-[400px] bg-pink-500/10 rounded-full blur-3xl animate-float-fast -translate-x-1/2 -translate-y-1/2" />
            </div>

            // Hero Section
            <section class="min-h-screen flex items-center justify-center px-4 md:px-8 lg:px-12 py-20">
                <div class="max-w-7xl mx-auto w-full">
                    <div class="flex flex-col lg:flex-row items-center gap-12 lg:gap-20">
                        // Text Content
                        <div class="flex-1 text-center lg:text-left animate-slide-in-right fill-mode-forwards" style="opacity: 0; animation-delay: 0.1s; animation-fill-mode: forwards;">
                             // Greeting Badge
                             <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-blue-500/20 mb-6 animate-slide-up" style="opacity: 0; animation-delay: 0.2s; animation-fill-mode: forwards;">
                                <span class="animate-wave text-2xl">"👋"</span>
                                <span class="text-sm font-medium text-muted-foreground">
                                    "Selamat datang di portfolio saya"
                                </span>
                             </div>

                             // Main Title
                             <h1 class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black leading-tight mb-6 animate-slide-up" style="opacity: 0; animation-delay: 0.3s; animation-fill-mode: forwards;">
                                <span class="text-foreground">"Halo, saya"</span>
                                <br />
                                <span class="bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 bg-clip-text text-transparent">
                                    <AnimatedHeader words=judul />
                                </span>
                             </h1>

                             // Subtitle
                             <p class="text-lg md:text-xl text-muted-foreground max-w-lg mx-auto lg:mx-0 mb-8 animate-fade-in" style="opacity: 0; animation-delay: 0.5s; animation-fill-mode: forwards;">
                                <span class="text-foreground font-semibold">
                                    "Full-Stack Developer"
                                </span>" "
                                "yang passionate dalam membangun aplikasi web & mobile dengan teknologi modern."
                             </p>

                             // CTA Buttons
                             <div class="flex flex-wrap gap-4 justify-center lg:justify-start mb-10 animate-slide-up" style="opacity: 0; animation-delay: 0.6s; animation-fill-mode: forwards;">
                                <A
                                    href="/project"
                                    class="group relative px-8 py-4 rounded-2xl bg-gradient-to-r from-blue-500 to-purple-600 text-white font-bold shadow-lg shadow-purple-500/30 hover:shadow-xl hover:shadow-purple-500/40 hover:scale-105 transition-all duration-300 overflow-hidden"
                                >
                                    <span class="relative z-10 flex items-center gap-2">
                                        "Lihat Project"
                                        <svg
                                            class="w-5 h-5 group-hover:translate-x-1 transition-transform"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M14 5l7 7m0 0l-7 7m7-7H3"
                                            />
                                        </svg>
                                    </span>
                                    <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300" />
                                </A>
                                <a
                                    href="mailto:superaseph@gmail.com"
                                    class="px-8 py-4 rounded-2xl border-2 border-border text-foreground font-bold hover:border-primary hover:bg-primary/5 hover:scale-105 transition-all duration-300 flex items-center gap-2"
                                >
                                    <span>"📧"</span>
                                    "Contact Me"
                                </a>
                             </div>
                        </div>

                        // Profile Image
                        <div class="relative perspective-1000 animate-slide-in-right" style="opacity: 0; animation-delay: 0.1s; animation-fill-mode: forwards;">
                            // Decorative rings
                            <div class="absolute -inset-8 rounded-full border-2 border-dashed border-blue-500/20 animate-spin-slow" />
                            <div class="absolute -inset-16 rounded-full border border-purple-500/10 animate-spin-reverse" />

                            // Glow effect
                            <div class="absolute inset-0 bg-gradient-to-r from-blue-500/40 via-purple-500/40 to-pink-500/40 rounded-full blur-3xl opacity-50 animate-pulse" />

                            // Profile container
                            <div class="relative w-64 h-64 md:w-80 md:h-80 lg:w-96 lg:h-96 group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 rounded-full opacity-75 blur group-hover:opacity-100 transition-opacity duration-500" />
                                <img
                                    src="/public/profil.avif"
                                    alt="Asep Haryana Saputra"
                                    loading="lazy"
                                    class="relative rounded-full w-full h-full object-cover border-4 border-background shadow-2xl group-hover:scale-[1.02] transition-transform duration-500"
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Skills Section
            <section class="py-20 px-4 md:px-8 lg:px-12">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-12 animate-slide-up" style="animation-fill-mode: forwards;">
                        <h2 class="text-3xl md:text-4xl font-bold mb-4">
                            <span class="bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent">
                                "Tech Stack"
                            </span>
                        </h2>
                        <p class="text-muted-foreground max-w-md mx-auto">
                            "Teknologi yang saya gunakan untuk membangun aplikasi"
                        </p>
                    </div>

                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4">
                        {TECH_STACK.into_iter().enumerate().map(|(i, skill)| {
                            let delay = format!("animation-delay: {}s", 0.1 + (i as f64) * 0.1);
                            view! {
                                <div class="group relative animate-slide-up" style=format!("opacity: 0; animation-fill_mode: forwards; {}", delay)>
                                    <div
                                        class=format!("absolute -inset-1 bg-gradient-to-r {} rounded-2xl opacity-0 group-hover:opacity-100 blur transition-opacity duration-300", skill.color)
                                    />
                                    <div class="relative glass-card rounded-2xl p-6 text-center hover:scale-105 transition-transform duration-300 cursor-default flex flex-col items-center">
                                        <img
                                            src=skill.image
                                            alt=skill.name
                                            class="w-10 h-10 mb-3"
                                        />
                                        <span class="font-semibold text-sm">{skill.name}</span>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </section>

            // About Section
            <section class="py-20 px-4 md:px-8 lg:px-12">
                <div class="max-w-6xl mx-auto">
                    <div class="flex flex-col lg:flex-row items-center gap-12">
                        <div class="flex-1 animate-slide-in-right" style="opacity: 0; animation-fill-mode: forwards;">
                            <span class="inline-block px-4 py-1 rounded-full bg-blue-500/10 text-blue-500 text-sm font-medium mb-4">
                                "Tentang Saya"
                            </span>
                            <h2 class="text-3xl md:text-4xl font-bold mb-6 text-foreground">
                                "Passionate Developer dengan semangat belajar tinggi"
                            </h2>
                            <p class="text-muted-foreground text-lg leading-relaxed mb-6">
                                "Saya adalah seorang programmer yang selalu antusias mempelajari teknologi baru. Di waktu luang, saya menikmati bermain game dan menonton anime. Saya percaya bahwa kombinasi kreativitas dan logika adalah kunci untuk membangun solusi software yang luar biasa."
                            </p>
                            <div class="flex flex-wrap gap-4">
                                <div class="glass-card rounded-xl p-4 flex items-center gap-3">
                                    <span class="text-2xl">"🎮"</span>
                                    <span class="font-medium">"Gamer"</span>
                                </div>
                                <div class="glass-card rounded-xl p-4 flex items-center gap-3">
                                    <span class="text-2xl">"📺"</span>
                                    <span class="font-medium">"Anime Lover"</span>
                                </div>
                                <div class="glass-card rounded-xl p-4 flex items-center gap-3">
                                    <span class="text-2xl">"💻"</span>
                                    <span class="font-medium">"Code Enthusiast"</span>
                                </div>
                            </div>
                        </div>

                        <div class="flex-1 animate-slide-in-right" style="opacity: 0; animation-delay: 0.2s; animation-fill-mode: forwards;">
                            <div class="glass-card rounded-3xl p-8">
                                <h3 class="text-xl font-bold mb-6 flex items-center gap-2">
                                    <span class="text-2xl">"🤝"</span>
                                    "Mari Terhubung"
                                </h3>
                                <p class="text-muted-foreground mb-6">
                                    "Tertarik untuk berkolaborasi atau sekadar ngobrol? Jangan ragu untuk menghubungi saya melalui platform berikut!"
                                </p>
                                <div class="grid grid-cols-2 gap-4">
                                     <a href="https://github.com/MythEclipse" target="_blank" rel="noopener noreferrer" class="flex items-center gap-3 p-4 rounded-xl border border-border transition-all duration-300 hover:scale-105 hover:bg-gray-800 hover:text-white">
                                        <GitHub/>
                                        <span class="font-medium">"GitHub"</span>
                                     </a>
                                     <a href="https://www.instagram.com/asepharyana18/" target="_blank" rel="noopener noreferrer" class="flex items-center gap-3 p-4 rounded-xl border border-border transition-all duration-300 hover:scale-105 hover:bg-gradient-to-r hover:from-purple-500 hover:to-pink-500 hover:text-white">
                                        <Instagram/>
                                        <span class="font-medium">"Instagram"</span>
                                     </a>
                                     <a href="https://www.linkedin.com/in/asep-haryana-saputra-2014a5294/" target="_blank" rel="noopener noreferrer" class="flex items-center gap-3 p-4 rounded-xl border border-border transition-all duration-300 hover:scale-105 hover:bg-blue-600 hover:text-white">
                                        <LinkedIn/>
                                        <span class="font-medium">"LinkedIn"</span>
                                     </a>
                                     <a href="https://www.facebook.com/asep.haryana.900/" target="_blank" rel="noopener noreferrer" class="flex items-center gap-3 p-4 rounded-xl border border-border transition-all duration-300 hover:scale-105 hover:bg-blue-500 hover:text-white">
                                        <Facebook/>
                                        <span class="font-medium">"Facebook"</span>
                                     </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
