import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { Motion } from "solid-motionone";
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

import { isServer } from "solid-js/web";

async function fetchAnimeData(): Promise<HomeData> {
    return httpClient.fetchJson<HomeData>("/api/anime2");
}

function AnimeCard(props: { item: AnimeItem; index: number }) {
    return (
        <Motion.div
            initial={{ opacity: 0, y: 80, scale: 0.7, rotateY: -20 }}
            animate={{ opacity: 1, y: 0, scale: 1, rotateY: 0 }}
            transition={{
                duration: 0.7,
                delay: props.index * 0.06,
                easing: [0.34, 1.56, 0.64, 1]
            }}
            class="group perspective-1000"
        >
            <A
                href={`/anime2/detail/${props.item.slug}`}
                class="block relative overflow-hidden rounded-2xl bg-card border border-border shadow-lg hover:shadow-2xl hover:shadow-purple-500/20 transition-all duration-500 transform-gpu hover:-translate-y-4 hover:rotate-1"
            >
                {/* Animated gradient border */}
                <div class="absolute -inset-[2px] rounded-2xl bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10 blur-sm animate-gradient-rotate" />

                {/* Glow effect */}
                <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-700 bg-gradient-to-br from-purple-500/30 via-transparent to-pink-500/30 blur-2xl" />

                <div class="aspect-[3/4] overflow-hidden relative bg-gradient-to-br from-purple-900/20 to-pink-900/20">
                    <CachedImage
                        src={props.item.poster}
                        alt={props.item.title}
                        class="w-full h-full object-cover transform-gpu group-hover:scale-115 group-hover:rotate-2 transition-all duration-700 ease-out"
                        fallbackClass="w-full h-full bg-muted animate-pulse"
                        loading="lazy"
                    />

                    {/* Multi-layer shine effect */}
                    <div class="absolute inset-0 bg-gradient-to-tr from-transparent via-white/30 to-transparent opacity-0 group-hover:opacity-100 -translate-x-full group-hover:translate-x-full transition-all duration-1000 ease-out" />
                    <div class="absolute inset-0 bg-gradient-to-bl from-purple-500/20 via-transparent to-pink-500/20 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                </div>

                {/* Gradient overlay */}
                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/50 to-transparent opacity-80" />

                {/* Floating particles effect */}
                <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none">
                    <div class="absolute bottom-1/4 left-1/4 w-2 h-2 bg-purple-400 rounded-full animate-float-slow" />
                    <div class="absolute bottom-1/3 right-1/3 w-1.5 h-1.5 bg-pink-400 rounded-full animate-float-medium" />
                    <div class="absolute bottom-1/2 left-1/3 w-1 h-1 bg-white rounded-full animate-float-fast" />
                </div>

                {/* Content */}
                <div class="absolute bottom-0 left-0 right-0 p-4">
                    <h3 class="text-white text-sm font-bold line-clamp-2 drop-shadow-lg mb-2 group-hover:text-purple-200 transition-colors duration-300">
                        {props.item.title}
                    </h3>
                    <Show when={props.item.current_episode}>
                        <Motion.span
                            initial={{ opacity: 0, scale: 0 }}
                            animate={{ opacity: 1, scale: 1 }}
                            class="inline-flex items-center gap-1.5 text-xs font-bold px-3 py-1 rounded-full bg-gradient-to-r from-purple-500 to-pink-500 text-white shadow-lg"
                        >
                            <span class="w-2 h-2 rounded-full bg-white animate-pulse" />
                            {props.item.current_episode}
                        </Motion.span>
                    </Show>
                </div>

                {/* Play button with ripple */}
                <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-500">
                    <div class="relative">
                        <div class="absolute inset-0 bg-white/30 rounded-full animate-ping" />
                        <div class="w-20 h-20 rounded-full bg-gradient-to-r from-purple-500 to-pink-500 flex items-center justify-center transform scale-0 group-hover:scale-100 transition-transform duration-500 shadow-2xl shadow-purple-500/50">
                            <svg class="w-10 h-10 text-white ml-1.5" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M8 5v14l11-7z" />
                            </svg>
                        </div>
                    </div>
                </div>
            </A>
        </Motion.div>
    );
}

function AnimeGrid(props: { items: AnimeItem[]; loading?: boolean }) {
    return (
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6">
            <Show when={props.loading}>
                <For each={Array(12).fill(0)}>
                    {(_, index) => (
                        <Motion.div
                            initial={{ opacity: 0, y: 30 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ delay: index() * 0.05 }}
                            class="aspect-[3/4] rounded-2xl bg-gradient-to-br from-purple-900/30 to-pink-900/30 relative overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/10 to-transparent -translate-x-full animate-shimmer" />
                            <div class="absolute inset-0 flex items-center justify-center">
                                <div class="w-8 h-8 border-2 border-purple-500 border-t-transparent rounded-full animate-spin" />
                            </div>
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
    linkGradient: string;
}) {
    return (
        <Motion.div
            initial={{ opacity: 0, x: -50 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.6, easing: [0.34, 1.56, 0.64, 1] }}
            class="flex items-center justify-between mb-10"
        >
            <div class="flex items-center gap-5">
                <Motion.div
                    initial={{ scale: 0, rotate: -360 }}
                    animate={{ scale: 1, rotate: 0 }}
                    transition={{ duration: 0.8, easing: [0.34, 1.56, 0.64, 1] }}
                    class={`p-5 rounded-3xl bg-gradient-to-br ${props.gradient} shadow-2xl relative overflow-hidden`}
                >
                    <div class="absolute inset-0 bg-white/20 opacity-0 hover:opacity-100 transition-opacity duration-300" />
                    <span class="text-3xl relative z-10">{props.icon}</span>
                </Motion.div>
                <Motion.h2
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.3 }}
                    class="text-2xl md:text-4xl font-black text-foreground"
                >
                    {props.title}
                </Motion.h2>
            </div>
            <Motion.div
                initial={{ opacity: 0, x: 30 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: 0.4 }}
            >
                <A
                    href={props.link}
                    class={`group relative overflow-hidden flex items-center gap-2 px-6 py-3 rounded-2xl bg-gradient-to-r ${props.linkGradient} text-white font-bold shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105`}
                >
                    <span class="relative z-10">View All</span>
                    <svg class="w-5 h-5 relative z-10 transform group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                    </svg>
                    <div class="absolute inset-0 bg-white/20 translate-x-[-100%] group-hover:translate-x-0 transition-transform duration-500" />
                </A>
            </Motion.div>
        </Motion.div>
    );
}

export default function Anime2Page() {
    const [enabled, setEnabled] = createSignal(false);
    onMount(() => setEnabled(true));

    const [data] = createResource(enabled, fetchAnimeData);

    return (
        <>
            <Title>Anime2 | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground overflow-hidden relative">
                {/* Animated background with floating orbs */}
                <div class="fixed inset-0 -z-10 overflow-hidden">
                    <div class="absolute top-[-10%] left-[-5%] w-[500px] h-[500px] bg-purple-500/15 rounded-full blur-3xl animate-float-slow" />
                    <div class="absolute bottom-[-10%] right-[-5%] w-[600px] h-[600px] bg-pink-500/15 rounded-full blur-3xl animate-float-medium" />
                    <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[400px] h-[400px] bg-orange-500/10 rounded-full blur-3xl animate-float-fast" />
                    {/* Grid pattern overlay */}
                    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,.02)_1px,transparent_1px)] bg-[size:50px_50px]" />
                </div>

                <div class="p-4 md:p-8 lg:p-12 relative">
                    <div class="max-w-7xl mx-auto">
                        {/* Hero Header with 3D effect */}
                        <Motion.div
                            initial={{ opacity: 0, y: -80, rotateX: 20 }}
                            animate={{ opacity: 1, y: 0, rotateX: 0 }}
                            transition={{ duration: 1, easing: [0.34, 1.56, 0.64, 1] }}
                            class="text-center mb-16 perspective-1000"
                        >
                            <Motion.div
                                initial={{ scale: 0 }}
                                animate={{ scale: 1 }}
                                transition={{ duration: 0.8, delay: 0.2, easing: [0.34, 1.56, 0.64, 1] }}
                                class="inline-block mb-6"
                            >
                                <span class="px-6 py-2 rounded-full bg-gradient-to-r from-purple-500/20 to-pink-500/20 border border-purple-500/30 text-purple-400 font-medium text-sm">
                                    ✨ AlQanime Source
                                </span>
                            </Motion.div>

                            <Motion.h1
                                initial={{ opacity: 0, scale: 0.3 }}
                                animate={{ opacity: 1, scale: 1 }}
                                transition={{ duration: 0.8, delay: 0.3 }}
                                class="text-6xl md:text-8xl font-black mb-6"
                            >
                                <span class="bg-gradient-to-r from-purple-400 via-pink-500 to-orange-400 bg-clip-text text-transparent animate-gradient-x bg-[length:200%_auto] drop-shadow-2xl">
                                    Anime2
                                </span>
                            </Motion.h1>

                            <Motion.p
                                initial={{ opacity: 0, y: 20 }}
                                animate={{ opacity: 1, y: 0 }}
                                transition={{ delay: 0.5 }}
                                class="text-muted-foreground text-xl max-w-md mx-auto"
                            >
                                Download anime dari AlQanime dengan kualitas terbaik
                            </Motion.p>
                        </Motion.div>

                        {/* Search Bar with glassmorphism */}
                        <Motion.div
                            initial={{ opacity: 0, y: 50, scale: 0.8 }}
                            animate={{ opacity: 1, y: 0, scale: 1 }}
                            transition={{ duration: 0.6, delay: 0.4 }}
                            class="mb-16"
                        >
                            <form action="/anime2/search" method="get" class="flex gap-4 max-w-3xl mx-auto">
                                <div class="relative flex-1 group">
                                    <div class="absolute -inset-1 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 rounded-3xl opacity-50 group-focus-within:opacity-100 blur-lg transition-all duration-500" />
                                    <input
                                        type="text"
                                        name="q"
                                        placeholder="🔍 Cari anime favoritmu..."
                                        class="relative w-full px-8 py-5 rounded-2xl border-2 border-white/10 bg-background/80 backdrop-blur-xl focus:outline-none focus:border-purple-500 transition-all duration-300 text-lg font-medium"
                                    />
                                </div>
                                <button
                                    type="submit"
                                    class="px-10 py-5 rounded-2xl bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 text-white font-bold text-lg shadow-2xl shadow-purple-500/40 hover:shadow-pink-500/50 hover:scale-110 hover:rotate-2 active:scale-95 transition-all duration-300 relative overflow-hidden"
                                >
                                    <span class="relative z-10">Search</span>
                                    <div class="absolute inset-0 bg-white/20 translate-y-full hover:translate-y-0 transition-transform duration-300" />
                                </button>
                            </form>
                        </Motion.div>

                        <Suspense fallback={
                            <div class="space-y-20">
                                <section>
                                    <SectionHeader
                                        title="Ongoing Anime"
                                        icon="🔥"
                                        gradient="from-purple-500 to-pink-500"
                                        link="/anime2/ongoing-anime/1"
                                        linkGradient="from-purple-500 to-pink-500"
                                    />
                                    <AnimeGrid items={[]} loading={true} />
                                </section>
                            </div>
                        }>
                            <Show when={data.error}>
                                <Motion.div
                                    initial={{ opacity: 0, scale: 0.8, rotateY: 20 }}
                                    animate={{ opacity: 1, scale: 1, rotateY: 0 }}
                                    class="text-center py-20"
                                >
                                    <div class="w-32 h-32 mx-auto mb-8 rounded-full bg-gradient-to-br from-purple-500/20 to-pink-500/20 flex items-center justify-center">
                                        <span class="text-6xl animate-bounce">😢</span>
                                    </div>
                                    <p class="text-2xl font-bold text-foreground mb-2">Gagal memuat data</p>
                                    <p class="text-muted-foreground">Silakan coba lagi nanti</p>
                                </Motion.div>
                            </Show>

                            <Show when={data()}>
                                {(animeData) => (
                                    <div class="space-y-24">
                                        {/* Ongoing Anime */}
                                        <section>
                                            <SectionHeader
                                                title="Ongoing Anime"
                                                icon="🔥"
                                                gradient="from-purple-500 to-pink-500"
                                                link="/anime2/ongoing-anime/1"
                                                linkGradient="from-purple-500 to-pink-500"
                                            />
                                            <AnimeGrid items={animeData().data.ongoing_anime} />
                                        </section>

                                        {/* Complete Anime */}
                                        <section>
                                            <SectionHeader
                                                title="Complete Anime"
                                                icon="✨"
                                                gradient="from-orange-500 to-yellow-500"
                                                link="/anime2/complete-anime/1"
                                                linkGradient="from-orange-500 to-yellow-500"
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

            {/* Custom CSS for complex animations */}
            <style>{`
                @keyframes gradient-x {
                    0%, 100% { background-position: 0% 50%; }
                    50% { background-position: 100% 50%; }
                }
                .animate-gradient-x {
                    animation: gradient-x 4s ease infinite;
                }
                @keyframes gradient-rotate {
                    0% { transform: rotate(0deg); }
                    100% { transform: rotate(360deg); }
                }
                .animate-gradient-rotate {
                    animation: gradient-rotate 3s linear infinite;
                }
                @keyframes shimmer {
                    100% { transform: translateX(200%); }
                }
                .animate-shimmer {
                    animation: shimmer 2s infinite;
                }
                @keyframes float-slow {
                    0%, 100% { transform: translateY(0) translateX(0); }
                    25% { transform: translateY(-20px) translateX(10px); }
                    50% { transform: translateY(-10px) translateX(-5px); }
                    75% { transform: translateY(-25px) translateX(5px); }
                }
                .animate-float-slow {
                    animation: float-slow 8s ease-in-out infinite;
                }
                @keyframes float-medium {
                    0%, 100% { transform: translateY(0) scale(1); }
                    50% { transform: translateY(-30px) scale(1.05); }
                }
                .animate-float-medium {
                    animation: float-medium 6s ease-in-out infinite;
                }
                @keyframes float-fast {
                    0%, 100% { transform: translateY(0) rotate(0deg); }
                    50% { transform: translateY(-15px) rotate(180deg); }
                }
                .animate-float-fast {
                    animation: float-fast 4s ease-in-out infinite;
                }
                .perspective-1000 {
                    perspective: 1000px;
                }
                .group:hover .group-hover\\:scale-115 {
                    transform: scale(1.15);
                }
            `}</style>
        </>
    );
}
