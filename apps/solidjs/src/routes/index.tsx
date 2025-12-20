import { Title } from "@solidjs/meta";
import { Motion } from "solid-motionone";
import { A } from "@solidjs/router";
import { For } from "solid-js";

const features = [
  {
    title: "Anime Streaming",
    description: "Watch your favorite anime with multiple server options and high quality.",
    icon: "📺",
    gradient: "from-blue-500 via-purple-500 to-pink-500",
    link: "/anime"
  },
  {
    title: "Manga Reader",
    description: "Read manga, manhwa, and manhua with a clean reading experience.",
    icon: "📖",
    gradient: "from-orange-500 via-red-500 to-pink-500",
    link: "/komik"
  },
  {
    title: "AI Chat",
    description: "Chat with AI powered by multiple models for various tasks.",
    icon: "🤖",
    gradient: "from-green-500 via-teal-500 to-cyan-500",
    link: "/chat"
  },
  {
    title: "Image Tools",
    description: "Compress and optimize your images with our powerful tools.",
    icon: "🖼️",
    gradient: "from-cyan-500 via-blue-500 to-indigo-500",
    link: "/compressor"
  },
];

export default function Home() {
  return (
    <>
      <Title>Home | Asepharyana</Title>
      <main class="min-h-[calc(100vh-64px)] bg-background text-foreground relative overflow-hidden">
        {/* Animated Background Orbs */}
        <div class="absolute inset-0 -z-10 overflow-hidden">
          <div class="gradient-orb gradient-orb-1 w-[500px] h-[500px] left-[-10%] top-[-10%]" />
          <div class="gradient-orb gradient-orb-2 w-[600px] h-[600px] right-[-15%] top-[20%]" />
          <div class="gradient-orb gradient-orb-3 w-[400px] h-[400px] left-[30%] bottom-[-20%]" />
        </div>

        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-64px)] p-4 md:p-8 lg:p-12">
          {/* Hero Content */}
          <Motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7 }}
            class="text-center max-w-4xl mx-auto"
          >
            <Motion.div
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.6, delay: 0.1 }}
              class="mb-6"
            >
              <span class="inline-flex items-center gap-2 px-4 py-2 rounded-full glass-subtle text-sm font-medium text-muted-foreground">
                <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                Welcome to my digital space
              </span>
            </Motion.div>

            <Motion.h1
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.2 }}
              class="text-5xl md:text-7xl lg:text-8xl font-bold tracking-tight"
            >
              <span class="gradient-text-animated">
                Asep Haryana
              </span>
            </Motion.h1>

            <Motion.p
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.3 }}
              class="mt-6 text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto leading-relaxed"
            >
              A personal website featuring anime streaming, manga reading, AI chat, and more.
              Built with <span class="text-primary font-medium">SolidStart</span> for blazing fast performance.
            </Motion.p>

            <Motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.4 }}
              class="mt-10 flex flex-wrap justify-center gap-4"
            >
              <A
                href="/anime"
                class="group inline-flex items-center justify-center gap-2 rounded-xl bg-primary px-6 py-3.5 text-sm font-semibold text-primary-foreground shadow-lg shadow-primary/25 hover:shadow-xl hover:shadow-primary/30 transition-all hover:-translate-y-0.5"
              >
                Browse Anime
                <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                </svg>
              </A>
              <A
                href="/chat"
                class="inline-flex items-center justify-center rounded-xl glass-card px-6 py-3.5 text-sm font-semibold hover:bg-white/20 dark:hover:bg-white/10 transition-all hover:-translate-y-0.5"
              >
                Try AI Chat
              </A>
            </Motion.div>
          </Motion.div>

          {/* Feature Cards */}
          <Motion.div
            initial={{ opacity: 0, y: 50 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.6 }}
            class="mt-24 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 max-w-6xl mx-auto w-full"
          >
            <For each={features}>
              {(feature, index) => (
                <A href={feature.link} class="group block">
                  <div
                    class="relative overflow-hidden rounded-2xl glass-card p-6 transition-all duration-300 hover:-translate-y-2 hover:shadow-2xl"
                    style={{ "animation-delay": `${index() * 100}ms` }}
                  >
                    {/* Gradient glow on hover */}
                    <div class={`absolute inset-0 bg-gradient-to-br ${feature.gradient} opacity-0 group-hover:opacity-10 transition-opacity duration-300`} />

                    {/* Icon */}
                    <div class="relative mb-4">
                      <span class="text-4xl block group-hover:scale-110 transition-transform duration-300">{feature.icon}</span>
                    </div>

                    {/* Content */}
                    <div class="relative">
                      <h3 class="text-lg font-semibold mb-2 group-hover:text-primary transition-colors">{feature.title}</h3>
                      <p class="text-sm text-muted-foreground leading-relaxed">{feature.description}</p>
                    </div>

                    {/* Arrow indicator */}
                    <div class="absolute bottom-6 right-6 opacity-0 group-hover:opacity-100 transform translate-x-2 group-hover:translate-x-0 transition-all duration-300">
                      <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                      </svg>
                    </div>
                  </div>
                </A>
              )}
            </For>
          </Motion.div>

          {/* Stats or trust badges */}
          <Motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.6, delay: 1 }}
            class="mt-20 flex flex-wrap justify-center gap-8 text-center"
          >
            <div class="glass-subtle px-6 py-3 rounded-xl">
              <p class="text-2xl font-bold gradient-text">Fast</p>
              <p class="text-xs text-muted-foreground">SolidJS Powered</p>
            </div>
            <div class="glass-subtle px-6 py-3 rounded-xl">
              <p class="text-2xl font-bold gradient-text">Modern</p>
              <p class="text-xs text-muted-foreground">Latest Tech Stack</p>
            </div>
            <div class="glass-subtle px-6 py-3 rounded-xl">
              <p class="text-2xl font-bold gradient-text">Free</p>
              <p class="text-xs text-muted-foreground">No Cost Features</p>
            </div>
          </Motion.div>
        </div>
      </main>
    </>
  );
}

