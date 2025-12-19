import { Title } from "@solidjs/meta";
import { Motion } from "solid-motionone";

export default function Home() {
  return (
    <>
      <Title>Home | Asepharyana</Title>
      <main class="min-h-[calc(100vh-64px)] bg-background text-foreground">
        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-64px)] p-4 md:p-8 lg:p-12">
          {/* Background Gradient */}
          <div class="absolute inset-0 -z-10 overflow-hidden">
            <div class="absolute left-[50%] top-0 h-[500px] w-[500px] -translate-x-1/2 rounded-full bg-gradient-to-br from-primary/20 to-accent/20 blur-3xl" />
            <div class="absolute right-[20%] top-[20%] h-[300px] w-[300px] rounded-full bg-gradient-to-br from-secondary/30 to-muted/30 blur-3xl" />
          </div>

          {/* Content */}
          <Motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            class="text-center max-w-4xl mx-auto"
          >
            <Motion.h1
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.1 }}
              class="text-4xl md:text-6xl lg:text-7xl font-bold tracking-tight"
            >
              <span class="bg-gradient-to-r from-primary via-primary/80 to-accent bg-clip-text text-transparent">
                Asep Haryana
              </span>
            </Motion.h1>

            <Motion.p
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.2 }}
              class="mt-6 text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto"
            >
              A personal website featuring anime streaming, manga reading, AI chat, and more.
              Built with SolidStart for blazing fast performance.
            </Motion.p>

            <Motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.3 }}
              class="mt-10 flex flex-wrap justify-center gap-4"
            >
              <a
                href="/anime"
                class="inline-flex items-center justify-center rounded-lg bg-primary px-6 py-3 text-sm font-medium text-primary-foreground shadow-lg hover:bg-primary/90 transition-all hover:scale-105"
              >
                Browse Anime
              </a>
              <a
                href="/komik"
                class="inline-flex items-center justify-center rounded-lg border border-input bg-background px-6 py-3 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-all hover:scale-105"
              >
                Read Manga
              </a>
              <a
                href="/chat"
                class="inline-flex items-center justify-center rounded-lg border border-input bg-background px-6 py-3 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-all hover:scale-105"
              >
                AI Chat
              </a>
            </Motion.div>
          </Motion.div>

          {/* Feature Cards */}
          <Motion.div
            initial={{ opacity: 0, y: 40 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.5 }}
            class="mt-20 grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl mx-auto"
          >
            <FeatureCard
              title="Anime Streaming"
              description="Watch your favorite anime with multiple server options and high quality."
              icon="📺"
            />
            <FeatureCard
              title="Manga Reader"
              description="Read manga, manhwa, and manhua with a clean reading experience."
              icon="📖"
            />
            <FeatureCard
              title="AI Chat"
              description="Chat with AI powered by multiple models for various tasks."
              icon="🤖"
            />
          </Motion.div>
        </div>
      </main>
    </>
  );
}

function FeatureCard(props: { title: string; description: string; icon: string }) {
  return (
    <div class="group relative overflow-hidden rounded-xl border border-border bg-card p-6 shadow-sm transition-all hover:shadow-lg hover:border-primary/50">
      <div class="absolute inset-0 bg-gradient-to-br from-primary/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
      <div class="relative">
        <span class="text-4xl">{props.icon}</span>
        <h3 class="mt-4 text-lg font-semibold text-card-foreground">{props.title}</h3>
        <p class="mt-2 text-sm text-muted-foreground">{props.description}</p>
      </div>
    </div>
  );
}
