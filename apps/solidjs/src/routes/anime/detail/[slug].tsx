import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { Motion } from "solid-motionone";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

interface Genre {
    name: string;
    slug: string;
    anime_url: string;
}

interface EpisodeList {
    episode: string;
    slug: string;
}

interface Recommendation {
    title: string;
    slug: string;
    poster: string;
    status?: string | null;
    type?: string | null;
}

// Matches OpenAPI AnimeDetailData schema
interface AnimeDetail {
    title: string;
    alternative_title: string;
    poster: string;
    release_date: string;
    studio: string;
    synopsis: string;
    episode_lists: EpisodeList[];
    recommendations: Recommendation[];
    batch?: EpisodeList[];
    genres?: Genre[];
    producers?: string[];
    status?: string | null;
    type?: string | null;
}

interface DetailResponse {
    status?: string | null;
    data: AnimeDetail;
}

async function fetchAnimeDetail(slug: string): Promise<DetailResponse> {
    return httpClient.fetchJson<DetailResponse>(`/api/anime/detail/${slug}`);
}

export default function AnimeDetailPage() {
    const params = useParams();
    const [data] = createResource(() => params.slug, fetchAnimeDetail);

    return (
        <>
            <Title>{data()?.data.title || "Anime Detail"} | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground overflow-hidden relative">
                {/* Animated background */}
                <div class="fixed inset-0 -z-10 overflow-hidden">
                    <div class="absolute top-[-20%] left-[-10%] w-[600px] h-[600px] bg-blue-500/10 rounded-full blur-3xl animate-float-slow" />
                    <div class="absolute bottom-[-20%] right-[-10%] w-[500px] h-[500px] bg-purple-500/10 rounded-full blur-3xl animate-float-medium" />
                </div>

                <Suspense fallback={
                    <div class="p-8 max-w-6xl mx-auto">
                        <div class="flex flex-col md:flex-row gap-8">
                            <div class="w-full md:w-1/3">
                                <div class="aspect-[3/4] rounded-3xl bg-gradient-to-br from-blue-900/30 to-purple-900/30 animate-pulse relative overflow-hidden">
                                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/10 to-transparent -translate-x-full animate-shimmer" />
                                </div>
                            </div>
                            <div class="flex-1 space-y-6">
                                <div class="h-10 w-3/4 bg-muted animate-pulse rounded-xl" />
                                <div class="h-6 w-1/2 bg-muted animate-pulse rounded-lg" />
                                <div class="grid grid-cols-2 gap-4">
                                    <div class="h-20 bg-muted animate-pulse rounded-xl" />
                                    <div class="h-20 bg-muted animate-pulse rounded-xl" />
                                    <div class="h-20 bg-muted animate-pulse rounded-xl" />
                                    <div class="h-20 bg-muted animate-pulse rounded-xl" />
                                </div>
                                <div class="h-40 bg-muted animate-pulse rounded-xl" />
                            </div>
                        </div>
                    </div>
                }>
                    <Show when={data.error}>
                        <Motion.div
                            initial={{ opacity: 0, scale: 0.9 }}
                            animate={{ opacity: 1, scale: 1 }}
                            class="p-8 text-center"
                        >
                            <div class="max-w-md mx-auto glass-card rounded-3xl p-12">
                                <div class="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-red-500/20 to-orange-500/20 flex items-center justify-center">
                                    <span class="text-5xl animate-bounce">😢</span>
                                </div>
                                <p class="text-xl font-bold text-foreground mb-2">Gagal memuat data</p>
                                <p class="text-muted-foreground mb-6">Silakan coba lagi nanti</p>
                                <A href="/anime" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 transition-colors">
                                    ← Kembali
                                </A>
                            </div>
                        </Motion.div>
                    </Show>

                    <Show when={data()}>
                        {(detailData) => (
                            <div class="p-4 md:p-8 lg:p-12 max-w-7xl mx-auto">
                                {/* Hero Section with parallax effect */}
                                <div class="flex flex-col lg:flex-row gap-8 lg:gap-12 mb-12">
                                    {/* Poster with 3D effect - smaller size due to low res images */}
                                    <Motion.div
                                        initial={{ opacity: 0, x: -100, rotateY: -30 }}
                                        animate={{ opacity: 1, x: 0, rotateY: 0 }}
                                        transition={{ duration: 0.8, easing: [0.34, 1.56, 0.64, 1] }}
                                        class="w-full sm:w-2/3 md:w-1/2 lg:w-1/4 mx-auto lg:mx-0 perspective-1000"
                                    >
                                        <div class="relative group">
                                            {/* Glow effect behind poster */}
                                            <div class="absolute -inset-4 bg-gradient-to-r from-blue-500/30 via-purple-500/30 to-pink-500/30 rounded-3xl blur-2xl opacity-50 group-hover:opacity-100 transition-opacity duration-500" />

                                            {/* Poster container */}
                                            <div class="relative rounded-3xl overflow-hidden shadow-2xl transform group-hover:scale-[1.02] group-hover:-rotate-1 transition-all duration-500">
                                                <CachedImage
                                                    src={detailData().data.poster}
                                                    alt={detailData().data.title}
                                                    class="w-full"
                                                    fallbackClass="w-full aspect-[3/4] bg-muted animate-pulse"
                                                />

                                                {/* Shine effect */}
                                                <div class="absolute inset-0 bg-gradient-to-tr from-transparent via-white/20 to-transparent opacity-0 group-hover:opacity-100 -translate-x-full group-hover:translate-x-full transition-all duration-1000" />

                                                {/* Type badge */}
                                                <Show when={detailData().data.type}>
                                                    <div class="absolute top-4 right-4 px-4 py-2 rounded-2xl bg-gradient-to-r from-blue-500 to-purple-500 text-white font-bold shadow-lg flex items-center gap-2">
                                                        <span class="text-lg">{detailData().data.type}</span>
                                                    </div>
                                                </Show>
                                            </div>

                                            {/* Floating badges */}
                                            <div class="absolute -bottom-3 -right-3 flex gap-2">
                                                <Show when={detailData().data.type}>
                                                    <span class="px-4 py-2 rounded-xl bg-gradient-to-r from-blue-500 to-cyan-500 text-white font-bold text-sm shadow-lg animate-float-fast">
                                                        {detailData().data.type}
                                                    </span>
                                                </Show>
                                            </div>
                                        </div>

                                        {/* Quick Info Card */}
                                        <Motion.div
                                            initial={{ opacity: 0, y: 30 }}
                                            animate={{ opacity: 1, y: 0 }}
                                            transition={{ delay: 0.3 }}
                                            class="mt-6 glass-card rounded-2xl p-5 space-y-4"
                                        >
                                            <h3 class="font-bold text-lg flex items-center gap-2">
                                                <span class="text-xl">📋</span> Informasi
                                            </h3>

                                            <div class="space-y-3">
                                                <Show when={detailData().data.status}>
                                                    <div class="flex items-center gap-3">
                                                        <span class="w-10 h-10 rounded-xl bg-green-500/20 flex items-center justify-center text-lg">🟢</span>
                                                        <div>
                                                            <p class="text-xs text-muted-foreground">Status</p>
                                                            <p class="font-semibold">{detailData().data.status}</p>
                                                        </div>
                                                    </div>
                                                </Show>

                                                <Show when={detailData().data.episode_lists?.length}>
                                                    <div class="flex items-center gap-3">
                                                        <span class="w-10 h-10 rounded-xl bg-blue-500/20 flex items-center justify-center text-lg">🎬</span>
                                                        <div>
                                                            <p class="text-xs text-muted-foreground">Episodes</p>
                                                            <p class="font-semibold">{detailData().data.episode_lists.length} eps</p>
                                                        </div>
                                                    </div>
                                                </Show>

                                                <Show when={detailData().data.studio}>
                                                    <div class="flex items-center gap-3">
                                                        <span class="w-10 h-10 rounded-xl bg-pink-500/20 flex items-center justify-center text-lg">🏢</span>
                                                        <div>
                                                            <p class="text-xs text-muted-foreground">Studio</p>
                                                            <p class="font-semibold">{detailData().data.studio}</p>
                                                        </div>
                                                    </div>
                                                </Show>

                                                <Show when={detailData().data.release_date}>
                                                    <div class="flex items-center gap-3">
                                                        <span class="w-10 h-10 rounded-xl bg-orange-500/20 flex items-center justify-center text-lg">📅</span>
                                                        <div>
                                                            <p class="text-xs text-muted-foreground">Rilis</p>
                                                            <p class="font-semibold">{detailData().data.release_date}</p>
                                                        </div>
                                                    </div>
                                                </Show>
                                            </div>
                                        </Motion.div>
                                    </Motion.div>

                                    {/* Content */}
                                    <div class="flex-1 space-y-8">
                                        {/* Title Section */}
                                        <Motion.div
                                            initial={{ opacity: 0, y: -30 }}
                                            animate={{ opacity: 1, y: 0 }}
                                            transition={{ duration: 0.6, delay: 0.2 }}
                                        >
                                            <h1 class="text-4xl md:text-5xl lg:text-6xl font-black text-foreground leading-tight mb-3">
                                                {detailData().data.title}
                                            </h1>
                                            <Show when={detailData().data.alternative_title}>
                                                <p class="text-xl text-muted-foreground font-medium">
                                                    {detailData().data.alternative_title}
                                                </p>
                                            </Show>
                                        </Motion.div>

                                        {/* Genres with animation */}
                                        <Motion.div
                                            initial={{ opacity: 0, y: 20 }}
                                            animate={{ opacity: 1, y: 0 }}
                                            transition={{ delay: 0.3 }}
                                            class="flex flex-wrap gap-2"
                                        >
                                            <For each={detailData().data.genres}>
                                                {(g, index) => (
                                                    <Motion.span
                                                        initial={{ opacity: 0, scale: 0 }}
                                                        animate={{ opacity: 1, scale: 1 }}
                                                        transition={{ delay: 0.4 + index() * 0.05 }}
                                                        class="px-4 py-2 rounded-xl bg-gradient-to-r from-blue-500/20 to-purple-500/20 text-foreground font-medium text-sm border border-blue-500/30 hover:border-blue-500/60 hover:scale-105 transition-all duration-300 cursor-default"
                                                    >
                                                        {g.name}
                                                    </Motion.span>
                                                )}
                                            </For>
                                        </Motion.div>

                                        {/* Synopsis Card */}
                                        <Motion.div
                                            initial={{ opacity: 0, y: 30 }}
                                            animate={{ opacity: 1, y: 0 }}
                                            transition={{ delay: 0.4 }}
                                            class="glass-card rounded-3xl p-6 md:p-8"
                                        >
                                            <h3 class="font-bold text-xl mb-4 flex items-center gap-3">
                                                <span class="w-10 h-10 rounded-xl bg-gradient-to-r from-blue-500 to-purple-500 flex items-center justify-center text-white">📖</span>
                                                Sinopsis
                                            </h3>
                                            <p class="text-muted-foreground leading-relaxed text-base md:text-lg">
                                                {detailData().data.synopsis || "Tidak ada sinopsis."}
                                            </p>
                                        </Motion.div>

                                        {/* Episode List */}
                                        <Motion.div
                                            initial={{ opacity: 0, y: 30 }}
                                            animate={{ opacity: 1, y: 0 }}
                                            transition={{ delay: 0.5 }}
                                        >
                                            <h2 class="text-2xl md:text-3xl font-bold mb-6 flex items-center gap-3">
                                                <span class="w-12 h-12 rounded-2xl bg-gradient-to-r from-purple-500 to-pink-500 flex items-center justify-center text-white text-xl shadow-lg">🎬</span>
                                                Daftar Episode
                                                <span class="ml-auto text-base font-normal text-muted-foreground">
                                                    {detailData().data.episode_lists?.length || 0} Episode
                                                </span>
                                            </h2>

                                            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
                                                <For each={detailData().data.episode_lists}>
                                                    {(ep, index) => (
                                                        <Motion.div
                                                            initial={{ opacity: 0, y: 20, scale: 0.9 }}
                                                            animate={{ opacity: 1, y: 0, scale: 1 }}
                                                            transition={{ delay: 0.6 + index() * 0.02 }}
                                                        >
                                                            <A
                                                                href={`/anime/full/${ep.slug}`}
                                                                class="group block p-4 rounded-2xl bg-gradient-to-br from-card to-card/50 border border-border hover:border-blue-500/50 hover:shadow-lg hover:shadow-blue-500/10 transition-all duration-300 transform hover:-translate-y-1 hover:scale-[1.02]"
                                                            >
                                                                <div class="text-center">
                                                                    <span class="text-lg font-bold group-hover:text-blue-500 transition-colors">
                                                                        {ep.episode}
                                                                    </span>
                                                                </div>

                                                                {/* Play icon on hover */}
                                                                <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none">
                                                                    <div class="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center">
                                                                        <svg class="w-5 h-5 text-blue-500" fill="currentColor" viewBox="0 0 24 24">
                                                                            <path d="M8 5v14l11-7z" />
                                                                        </svg>
                                                                    </div>
                                                                </div>
                                                            </A>
                                                        </Motion.div>
                                                    )}
                                                </For>
                                            </div>
                                        </Motion.div>
                                    </div>
                                </div>

                                {/* Back Button */}
                                <Motion.div
                                    initial={{ opacity: 0, y: 20 }}
                                    animate={{ opacity: 1, y: 0 }}
                                    transition={{ delay: 0.8 }}
                                    class="mt-12"
                                >
                                    <A
                                        href="/anime"
                                        class="inline-flex items-center gap-3 px-6 py-3 rounded-2xl bg-gradient-to-r from-blue-500/10 to-purple-500/10 text-foreground border border-blue-500/30 hover:border-blue-500/60 hover:shadow-lg hover:shadow-blue-500/10 transition-all duration-300 font-medium"
                                    >
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                        </svg>
                                        Kembali ke Anime
                                    </A>
                                </Motion.div>
                            </div>
                        )}
                    </Show>
                </Suspense>
            </main>

            {/* Custom CSS for animations */}
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
                    0%, 100% { transform: translateY(0); }
                    50% { transform: translateY(-8px); }
                }
                .animate-float-fast {
                    animation: float-fast 3s ease-in-out infinite;
                }
                @keyframes shimmer {
                    100% { transform: translateX(200%); }
                }
                .animate-shimmer {
                    animation: shimmer 2s infinite;
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
