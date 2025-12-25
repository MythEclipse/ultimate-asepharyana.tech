import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { Motion, Presence } from "solid-motionone";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

interface AnimeItem {
    title: string;
    slug: string;
    poster: string;
    current_episode?: string;
    episode_count?: string;
}

interface HomeData {
    status: string;
    data: {
        ongoing_anime: AnimeItem[];
        complete_anime: AnimeItem[];
    };
}

async function fetchAnimeData(): Promise<HomeData> {
    return httpClient.fetchJson<HomeData>("/api/anime");
}

function AnimeCard(props: { item: AnimeItem; index: number }) {
    return (
        <Motion.div
            initial={{ opacity: 0, y: 60, scale: 0.8, rotateX: -15 }}
            animate={{ opacity: 1, y: 0, scale: 1, rotateX: 0 }}
            transition={{
                duration: 0.6,
                delay: props.index * 0.08,
                easing: [0.25, 0.46, 0.45, 0.94]
            }}
            class="group perspective-1000"
        >
            <A
                href={`/anime/detail/${props.item.slug}`}
                class="block relative overflow-hidden rounded-2xl bg-card border border-border shadow-lg hover:shadow-2xl hover:shadow-primary/20 transition-all duration-500 transform-gpu hover:-translate-y-3 hover:scale-[1.02]"
            >
                {/* Glow effect on hover */}
                <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-t from-primary/20 via-transparent to-transparent blur-xl" />

                {/* Animated border glow */}
                <div class="absolute -inset-[1px] rounded-2xl bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10 blur-sm" />

                <div class="aspect-[3/4] overflow-hidden relative">
                    <CachedImage
                        src={props.item.poster}
                        alt={props.item.title}
                        class="w-full h-full object-cover transform-gpu group-hover:scale-110 transition-transform duration-700 ease-out"
                        fallbackClass="w-full h-full bg-muted animate-pulse"
                        loading="lazy"
                    />

                    {/* Shine effect on hover */}
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-1000 ease-out" />
                </div>

                {/* Gradient overlay with animation */}
                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/40 to-transparent opacity-80 group-hover:opacity-90 transition-opacity duration-300" />

                {/* Content */}
                <div class="absolute bottom-0 left-0 right-0 p-4 transform group-hover:translate-y-0 transition-transform duration-300">
                    <h3 class="text-white text-sm font-bold line-clamp-2 drop-shadow-lg mb-1 group-hover:text-blue-200 transition-colors duration-300">
                        {props.item.title}
                    </h3>
                    <Show when={props.item.current_episode}>
                        <Motion.span
                            initial={{ opacity: 0, x: -10 }}
                            animate={{ opacity: 1, x: 0 }}
                            class="inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full bg-blue-500/80 text-white backdrop-blur-sm"
                        >
                            <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse" />
                            {props.item.current_episode}
                        </Motion.span>
                    </Show>
                </div>

                {/* Play button overlay */}
                <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300">
                    <div class="w-16 h-16 rounded-full bg-white/20 backdrop-blur-md flex items-center justify-center transform scale-50 group-hover:scale-100 transition-transform duration-500">
                        <svg class="w-8 h-8 text-white ml-1" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M8 5v14l11-7z" />
                        </svg>
                    </div>
                </div>
            </A>
        </Motion.div>
    );
}

function AnimeGrid(props: { items: AnimeItem[]; loading?: boolean }) {
    return (
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-5">
            <Show when={props.loading}>
                <For each={Array(12).fill(0)}>
                    {(_, index) => (
                        <Motion.div
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            transition={{ delay: index() * 0.05 }}
                            class="aspect-[3/4] rounded-2xl bg-gradient-to-br from-muted to-muted/50 animate-pulse relative overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/5 to-transparent -translate-x-full animate-shimmer" />
                        </Motion.div>
                    )}
                </For>
            </Show>
            <Show when={!props.loading}>
                <For each={props.items}>
                    {(item, index) => <AnimeCard item={item} index={index()} />}
                </For>
            </Show>
        </div>
    );
}

function SectionHeader(props: {
    title: string;
    icon: string;
    gradient: string;
    link: string;
    linkColor: string;
}) {
    return (
        <Motion.div
            initial={{ opacity: 0, x: -30 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.5 }}
            class="flex items-center justify-between mb-8"
        >
            <div class="flex items-center gap-4">
                <Motion.div
                    initial={{ scale: 0, rotate: -180 }}
                    animate={{ scale: 1, rotate: 0 }}
                    transition={{ duration: 0.6, easing: [0.34, 1.56, 0.64, 1] }}
                    class={`p-4 rounded-2xl bg-gradient-to-br ${props.gradient} shadow-lg`}
                >
                    <span class="text-2xl">{props.icon}</span>
                </Motion.div>
                <div>
                    <Motion.h2
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: 0.2 }}
                        class="text-2xl md:text-3xl font-bold text-foreground"
                    >
                        {props.title}
                    </Motion.h2>
                </div>
            </div>
            <Motion.div
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: 0.3 }}
            >
                <A
                    href={props.link}
                    class={`group flex items-center gap-2 px-4 py-2 rounded-xl ${props.linkColor} hover:scale-105 transition-all duration-300 font-medium`}
                >
                    View All
                    <svg class="w-4 h-4 transform group-hover:translate-x-1 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                    </svg>
                </A>
            </Motion.div>
        </Motion.div>
    );
}

export default function AnimePage() {
    const [data] = createResource(fetchAnimeData);

    return (
        <>
            <Title>Anime | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground overflow-hidden">
                {/* Animated background */}
                <div class="fixed inset-0 -z-10 overflow-hidden">
                    <div class="absolute top-0 left-1/4 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-pulse" />
                    <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-pulse delay-1000" />
                    <div class="absolute top-1/2 left-1/2 w-64 h-64 bg-pink-500/10 rounded-full blur-3xl animate-pulse delay-500" />
                </div>

                <div class="p-4 md:p-8 lg:p-12">
                    <div class="max-w-7xl mx-auto">
                        {/* Hero Header */}
                        <Motion.div
                            initial={{ opacity: 0, y: -50 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.8, easing: [0.25, 0.46, 0.45, 0.94] }}
                            class="text-center mb-12"
                        >
                            <Motion.h1
                                initial={{ opacity: 0, scale: 0.5 }}
                                animate={{ opacity: 1, scale: 1 }}
                                transition={{ duration: 0.6, delay: 0.2 }}
                                class="text-5xl md:text-7xl font-black mb-4"
                            >
                                <span class="bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 bg-clip-text text-transparent animate-gradient-x bg-[length:200%_auto]">
                                    Anime
                                </span>
                            </Motion.h1>
                            <Motion.p
                                initial={{ opacity: 0 }}
                                animate={{ opacity: 1 }}
                                transition={{ delay: 0.4 }}
                                class="text-muted-foreground text-lg"
                            >
                                Streaming dari Otakudesu
                            </Motion.p>
                        </Motion.div>

                        {/* Search Bar */}
                        <Motion.div
                            initial={{ opacity: 0, y: 30, scale: 0.9 }}
                            animate={{ opacity: 1, y: 0, scale: 1 }}
                            transition={{ duration: 0.5, delay: 0.3 }}
                            class="mb-12"
                        >
                            <form action="/anime/search" method="get" class="flex gap-3 max-w-2xl mx-auto">
                                <div class="relative flex-1 group">
                                    <input
                                        type="text"
                                        name="q"
                                        placeholder="🔍 Search anime..."
                                        class="w-full px-6 py-4 rounded-2xl border-2 border-border bg-background/50 backdrop-blur-sm focus:outline-none focus:border-primary focus:ring-4 focus:ring-primary/20 transition-all duration-300 text-lg"
                                    />
                                    <div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-blue-500/20 via-purple-500/20 to-pink-500/20 opacity-0 group-focus-within:opacity-100 transition-opacity duration-300 -z-10 blur-xl" />
                                </div>
                                <button
                                    type="submit"
                                    class="px-8 py-4 rounded-2xl bg-gradient-to-r from-blue-500 to-purple-600 text-white font-bold shadow-lg shadow-purple-500/30 hover:shadow-xl hover:shadow-purple-500/40 hover:scale-105 active:scale-95 transition-all duration-300"
                                >
                                    Search
                                </button>
                            </form>
                        </Motion.div>

                        <Suspense fallback={
                            <div class="space-y-16">
                                <section>
                                    <SectionHeader
                                        title="Ongoing Anime"
                                        icon="🎬"
                                        gradient="from-blue-500 to-blue-600"
                                        link="/anime/ongoing-anime/1"
                                        linkColor="text-blue-500 hover:bg-blue-500/10"
                                    />
                                    <AnimeGrid items={[]} loading={true} />
                                </section>
                            </div>
                        }>
                            <Show when={data.error}>
                                <Motion.div
                                    initial={{ opacity: 0, scale: 0.9 }}
                                    animate={{ opacity: 1, scale: 1 }}
                                    class="text-center py-16"
                                >
                                    <div class="w-24 h-24 mx-auto mb-6 rounded-full bg-destructive/10 flex items-center justify-center">
                                        <span class="text-4xl">😢</span>
                                    </div>
                                    <p class="text-destructive text-xl font-medium">Failed to load anime data</p>
                                    <p class="text-muted-foreground mt-2">Please try again later</p>
                                </Motion.div>
                            </Show>

                            <Show when={data()}>
                                {(animeData) => (
                                    <div class="space-y-20">
                                        {/* Ongoing Anime */}
                                        <section>
                                            <SectionHeader
                                                title="Ongoing Anime"
                                                icon="🎬"
                                                gradient="from-blue-500 to-cyan-500"
                                                link="/anime/ongoing-anime/1"
                                                linkColor="text-blue-500 hover:bg-blue-500/10"
                                            />
                                            <AnimeGrid items={animeData().data.ongoing_anime} />
                                        </section>

                                        {/* Complete Anime */}
                                        <section>
                                            <SectionHeader
                                                title="Complete Anime"
                                                icon="✅"
                                                gradient="from-green-500 to-emerald-500"
                                                link="/anime/complete-anime/1"
                                                linkColor="text-green-500 hover:bg-green-500/10"
                                            />
                                            <AnimeGrid items={animeData().data.complete_anime} />
                                        </section>
                                    </div>
                                )}
                            </Show>
                        </Suspense>
                    </div>
                </div>
            </main>

            {/* Custom CSS for animations */}
            <style>{`
                @keyframes gradient-x {
                    0%, 100% { background-position: 0% 50%; }
                    50% { background-position: 100% 50%; }
                }
                .animate-gradient-x {
                    animation: gradient-x 3s ease infinite;
                }
                @keyframes shimmer {
                    100% { transform: translateX(100%); }
                }
                .animate-shimmer {
                    animation: shimmer 2s infinite;
                }
                .perspective-1000 {
                    perspective: 1000px;
                }
            `}</style>
        </>
    );
}
