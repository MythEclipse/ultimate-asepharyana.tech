import { Title } from "@solidjs/meta";
import { Motion } from "solid-motionone";
import { A } from "@solidjs/router";
import { For } from "solid-js";
import BackgroundBeamsWithCollision from "~/components/background/BackgroundBeamsWithCollision";
import { AnimatedHeader } from "~/components/text/AnimatedHeader";
import { Instagram, Facebook, LinkedIn, GitHub } from "~/components/logo/SocialIcons";
import { techStackWithImages } from "~/components/logo/TechIcons";

const judul = [
  { text: "Asep", class: "text-blue-500 dark:text-blue-400" },
  { text: "Haryana", class: "text-purple-500 dark:text-purple-400" },
  { text: "Saputra", class: "text-pink-500 dark:text-pink-400" },
];

const skills = techStackWithImages;

const socialLinks = [
  { href: "https://github.com/MythEclipse", icon: GitHub, label: "GitHub", color: "hover:bg-gray-800 hover:text-white" },
  { href: "https://www.instagram.com/asepharyana18/", icon: Instagram, label: "Instagram", color: "hover:bg-gradient-to-r hover:from-purple-500 hover:to-pink-500 hover:text-white" },
  { href: "https://www.linkedin.com/in/asep-haryana-saputra-2014a5294/", icon: LinkedIn, label: "LinkedIn", color: "hover:bg-blue-600 hover:text-white" },
  { href: "https://www.facebook.com/asep.haryana.900/", icon: Facebook, label: "Facebook", color: "hover:bg-blue-500 hover:text-white" },
];

export default function Home() {
  return (
    <>
      <Title>Home | Asepharyana</Title>
      <main class="min-h-screen bg-background text-foreground overflow-hidden">
        {/* Animated background orbs */}
        <div class="fixed inset-0 -z-10 overflow-hidden">
          <div class="absolute top-[-20%] left-[-10%] w-[600px] h-[600px] bg-blue-500/10 rounded-full blur-3xl animate-float-slow" />
          <div class="absolute bottom-[-20%] right-[-10%] w-[500px] h-[500px] bg-purple-500/10 rounded-full blur-3xl animate-float-medium" />
          <div class="absolute top-1/2 left-1/2 w-[400px] h-[400px] bg-pink-500/10 rounded-full blur-3xl animate-float-fast -translate-x-1/2 -translate-y-1/2" />
        </div>

        <BackgroundBeamsWithCollision>
          {/* Hero Section */}
          <section class="min-h-screen flex items-center justify-center px-4 md:px-8 lg:px-12 py-20">
            <div class="max-w-7xl mx-auto w-full">
              <div class="flex flex-col lg:flex-row items-center gap-12 lg:gap-20">
                {/* Text Content */}
                <Motion.div
                  initial={{ opacity: 0, x: -80 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.8, easing: [0.34, 1.56, 0.64, 1] }}
                  class="flex-1 text-center lg:text-left"
                >
                  {/* Greeting Badge */}
                  <Motion.div
                    initial={{ opacity: 0, y: -20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.2 }}
                    class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-blue-500/20 mb-6"
                  >
                    <span class="animate-wave text-2xl">👋</span>
                    <span class="text-sm font-medium text-muted-foreground">Selamat datang di portfolio saya</span>
                  </Motion.div>

                  {/* Main Title */}
                  <Motion.h1
                    initial={{ opacity: 0, y: 30 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.3, duration: 0.8 }}
                    class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black leading-tight mb-6"
                  >
                    <span class="text-foreground">Halo, saya</span>
                    <br />
                    <span class="bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 bg-clip-text text-transparent">
                      <AnimatedHeader words={judul} />
                    </span>
                  </Motion.h1>

                  {/* Subtitle */}
                  <Motion.p
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ delay: 0.5 }}
                    class="text-lg md:text-xl text-muted-foreground max-w-lg mx-auto lg:mx-0 mb-8"
                  >
                    <span class="text-foreground font-semibold">Full-Stack Developer</span> yang passionate dalam membangun aplikasi web & mobile dengan teknologi modern.
                  </Motion.p>

                  {/* CTA Buttons */}
                  <Motion.div
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.6 }}
                    class="flex flex-wrap gap-4 justify-center lg:justify-start mb-10"
                  >
                    <A
                      href="/project"
                      class="group relative px-8 py-4 rounded-2xl bg-gradient-to-r from-blue-500 to-purple-600 text-white font-bold shadow-lg shadow-purple-500/30 hover:shadow-xl hover:shadow-purple-500/40 hover:scale-105 transition-all duration-300 overflow-hidden"
                    >
                      <span class="relative z-10 flex items-center gap-2">
                        Lihat Project
                        <svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                        </svg>
                      </span>
                      <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300" />
                    </A>
                    <a
                      href="mailto:superaseph@gmail.com"
                      class="px-8 py-4 rounded-2xl border-2 border-border text-foreground font-bold hover:border-primary hover:bg-primary/5 hover:scale-105 transition-all duration-300 flex items-center gap-2"
                    >
                      <span>📧</span>
                      Contact Me
                    </a>
                  </Motion.div>
                </Motion.div>

                {/* Profile Image */}
                <Motion.div
                  initial={{ opacity: 0, x: 80, rotateY: 30 }}
                  animate={{ opacity: 1, x: 0, rotateY: 0 }}
                  transition={{ duration: 1, easing: [0.34, 1.56, 0.64, 1] }}
                  class="relative perspective-1000"
                >
                  {/* Decorative rings */}
                  <div class="absolute -inset-8 rounded-full border-2 border-dashed border-blue-500/20 animate-spin-slow" />
                  <div class="absolute -inset-16 rounded-full border border-purple-500/10 animate-spin-reverse" />

                  {/* Glow effect */}
                  <div class="absolute inset-0 bg-gradient-to-r from-blue-500/40 via-purple-500/40 to-pink-500/40 rounded-full blur-3xl opacity-50 animate-pulse" />

                  {/* Profile container */}
                  <div class="relative w-64 h-64 md:w-80 md:h-80 lg:w-96 lg:h-96 group">
                    <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 rounded-full opacity-75 blur group-hover:opacity-100 transition-opacity duration-500" />
                    {/* Loading placeholder */}
                    <div class="absolute inset-0 rounded-full bg-gradient-to-br from-blue-900/30 to-purple-900/30 animate-pulse" />
                    <img
                      src="/profil.avif"
                      alt="Asep Haryana Saputra"
                      loading="lazy"
                      decoding="async"
                      fetchpriority="low"
                      class="relative rounded-full w-full h-full object-cover border-4 border-background shadow-2xl group-hover:scale-[1.02] transition-transform duration-500"
                      onLoad={(e) => {
                        // Remove placeholder when image loads
                        const placeholder = e.currentTarget.previousElementSibling;
                        if (placeholder) placeholder.classList.add('opacity-0');
                      }}
                    />
                  </div>
                </Motion.div>
              </div>
            </div>
          </section>

          {/* Skills Section */}
          <section class="py-20 px-4 md:px-8 lg:px-12">
            <div class="max-w-6xl mx-auto">
              <Motion.div
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6 }}
                class="text-center mb-12"
              >
                <h2 class="text-3xl md:text-4xl font-bold mb-4">
                  <span class="bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent">
                    Tech Stack
                  </span>
                </h2>
                <p class="text-muted-foreground max-w-md mx-auto">
                  Teknologi yang saya gunakan untuk membangun aplikasi
                </p>
              </Motion.div>

              <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4">
                <For each={skills}>
                  {(skill, index) => (
                    <Motion.div
                      initial={{ opacity: 0, y: 30, scale: 0.8 }}
                      animate={{ opacity: 1, y: 0, scale: 1 }}
                      transition={{ delay: 0.1 + index() * 0.1 }}
                      class="group relative"
                    >
                      <div class={`absolute -inset-1 bg-gradient-to-r ${skill.color} rounded-2xl opacity-0 group-hover:opacity-100 blur transition-opacity duration-300`} />
                      <div class="relative glass-card rounded-2xl p-6 text-center hover:scale-105 transition-transform duration-300 cursor-default flex flex-col items-center">
                        <img src={skill.image} alt={skill.name} class="w-10 h-10 mb-3" />
                        <span class="font-semibold text-sm">{skill.name}</span>
                      </div>
                    </Motion.div>
                  )}
                </For>
              </div>
            </div>
          </section>

          {/* About Section */}
          <section class="py-20 px-4 md:px-8 lg:px-12">
            <div class="max-w-6xl mx-auto">
              <div class="flex flex-col lg:flex-row items-center gap-12">
                <Motion.div
                  initial={{ opacity: 0, x: -50 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.6 }}
                  class="flex-1"
                >
                  <span class="inline-block px-4 py-1 rounded-full bg-blue-500/10 text-blue-500 text-sm font-medium mb-4">
                    Tentang Saya
                  </span>
                  <h2 class="text-3xl md:text-4xl font-bold mb-6 text-foreground">
                    Passionate Developer dengan semangat belajar tinggi
                  </h2>
                  <p class="text-muted-foreground text-lg leading-relaxed mb-6">
                    Saya adalah seorang programmer yang selalu antusias mempelajari teknologi baru.
                    Di waktu luang, saya menikmati bermain game dan menonton anime.
                    Saya percaya bahwa kombinasi kreativitas dan logika adalah kunci untuk membangun
                    solusi software yang luar biasa.
                  </p>
                  <div class="flex flex-wrap gap-4">
                    <div class="glass-card rounded-xl p-4 flex items-center gap-3">
                      <span class="text-2xl">🎮</span>
                      <span class="font-medium">Gamer</span>
                    </div>
                    <div class="glass-card rounded-xl p-4 flex items-center gap-3">
                      <span class="text-2xl">📺</span>
                      <span class="font-medium">Anime Lover</span>
                    </div>
                    <div class="glass-card rounded-xl p-4 flex items-center gap-3">
                      <span class="text-2xl">💻</span>
                      <span class="font-medium">Code Enthusiast</span>
                    </div>
                  </div>
                </Motion.div>

                <Motion.div
                  initial={{ opacity: 0, x: 50 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.6 }}
                  class="flex-1"
                >
                  <div class="glass-card rounded-3xl p-8">
                    <h3 class="text-xl font-bold mb-6 flex items-center gap-2">
                      <span class="text-2xl">🤝</span>
                      Mari Terhubung
                    </h3>
                    <p class="text-muted-foreground mb-6">
                      Tertarik untuk berkolaborasi atau sekadar ngobrol?
                      Jangan ragu untuk menghubungi saya melalui platform berikut!
                    </p>
                    <div class="grid grid-cols-2 gap-4">
                      <For each={socialLinks}>
                        {(social) => (
                          <a
                            href={social.href}
                            target="_blank"
                            rel="noopener noreferrer"
                            class={`flex items-center gap-3 p-4 rounded-xl border border-border transition-all duration-300 hover:scale-105 ${social.color}`}
                          >
                            <social.icon />
                            <span class="font-medium">{social.label}</span>
                          </a>
                        )}
                      </For>
                    </div>
                  </div>
                </Motion.div>
              </div>
            </div>
          </section>
        </BackgroundBeamsWithCollision>
      </main>

      {/* Custom CSS */}
      <style>{`
                @keyframes float-slow {
                    0%, 100% { transform: translateY(0) translateX(0); }
                    50% { transform: translateY(-30px) translateX(15px); }
                }
                .animate-float-slow {
                    animation: float-slow 10s ease-in-out infinite;
                }
                @keyframes float-medium {
                    0%, 100% { transform: translateY(0) scale(1); }
                    50% { transform: translateY(-20px) scale(1.05); }
                }
                .animate-float-medium {
                    animation: float-medium 7s ease-in-out infinite;
                }
                @keyframes float-fast {
                    0%, 100% { transform: translateY(0) translateX(-50%) translateY(-50%); }
                    50% { transform: translateY(-15px) translateX(-50%) translateY(-50%); }
                }
                .animate-float-fast {
                    animation: float-fast 5s ease-in-out infinite;
                }
                @keyframes wave {
                    0%, 100% { transform: rotate(0deg); }
                    25% { transform: rotate(20deg); }
                    75% { transform: rotate(-10deg); }
                }
                .animate-wave {
                    animation: wave 2s ease-in-out infinite;
                    transform-origin: 70% 70%;
                }
                @keyframes spin-slow {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
                .animate-spin-slow {
                    animation: spin-slow 20s linear infinite;
                }
                @keyframes spin-reverse {
                    from { transform: rotate(360deg); }
                    to { transform: rotate(0deg); }
                }
                .animate-spin-reverse {
                    animation: spin-reverse 30s linear infinite;
                }
                .perspective-1000 {
                    perspective: 1000px;
                }
                .glass-card {
                    background: rgba(255, 255, 255, 0.05);
                    backdrop-filter: blur(10px);
                    border: 1px solid rgba(255, 255, 255, 0.1);
                }
            `}</style>
    </>
  );
}
