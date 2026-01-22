import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { Motion } from "solid-motionone";
import { httpClient } from "~/lib/http-client";

// Matches OpenAPI DownloadLink schema
interface DownloadLink {
    server: string;
    url: string;
}

// Matches OpenAPI EpisodeInfo schema
interface EpisodeInfo {
    slug: string;
}

// Matches OpenAPI AnimeInfo schema
interface AnimeInfo {
    slug: string;
}

// Matches OpenAPI AnimeFullData schema
interface AnimeFullData {
    episode: string;
    episode_number: string;
    anime: AnimeInfo;
    has_next_episode: boolean;
    has_previous_episode: boolean;
    stream_url: string;
    download_urls: Record<string, DownloadLink[]>;
    image_url: string;
    next_episode?: EpisodeInfo | null;
    previous_episode?: EpisodeInfo | null;
}

// Matches OpenAPI FullResponse schema
interface FullResponse {
    status: string;
    data: AnimeFullData;
}

async function fetchEpisode(slug: string): Promise<FullResponse> {
    return httpClient.fetchJson<FullResponse>(`/api/anime/full/${slug}`);
}

export default function AnimeFullPage() {
    const params = useParams();
    const [data] = createResource(() => params.slug, fetchEpisode);

    return (
        <>
            <Title>{data()?.data.episode || "Streaming"} | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground overflow-hidden relative">
                {/* Animated background */}
                <div class="fixed inset-0 -z-10 overflow-hidden">
                    <div class="absolute top-[-20%] left-[-10%] w-[600px] h-[600px] bg-blue-500/10 rounded-full blur-3xl animate-float-slow" />
                    <div class="absolute bottom-[-20%] right-[-10%] w-[500px] h-[500px] bg-purple-500/10 rounded-full blur-3xl animate-float-medium" />
                    {/* Grid pattern overlay */}
                    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,.02)_1px,transparent_1px)] bg-[size:50px_50px]" />
                </div>

                <Suspense fallback={
                    <div class="p-4 md:p-8 max-w-6xl mx-auto">
                        <div class="aspect-video w-full rounded-xl bg-muted animate-pulse mb-6" />
                        <div class="h-8 w-3/4 bg-muted animate-pulse rounded mb-4" />
                        <div class="flex gap-3">
                            <div class="h-10 w-32 bg-muted animate-pulse rounded" />
                            <div class="h-10 w-32 bg-muted animate-pulse rounded" />
                        </div>
                    </div>
                }>
                    <Show when={data.error}>
                        <div class="p-8 text-center">
                            <div class="max-w-md mx-auto glass-card rounded-2xl p-8">
                                <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                    <svg class="w-8 h-8 text-destructive" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                </div>
                                <p class="text-destructive font-medium mb-4">Failed to load episode</p>
                                <A href="/anime" class="text-primary hover:underline">← Back to Anime</A>
                            </div>
                        </div>
                    </Show>

                    <Show when={data()}>
                        {(episodeData) => (
                            <Motion.div
                                initial={{ opacity: 0, y: 20 }}
                                animate={{ opacity: 1, y: 0 }}
                                transition={{ duration: 0.5 }}
                                class="p-4 md:p-8 max-w-6xl mx-auto"
                            >
                                {/* Video Player */}
                                <div class="aspect-video w-full rounded-2xl overflow-hidden bg-black mb-6 shadow-2xl relative group">
                                    <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 to-purple-500 opacity-20 group-hover:opacity-40 blur transition-opacity duration-500" />
                                    <iframe
                                        src={episodeData().data.stream_url}
                                        class="w-full h-full relative z-10"
                                        allowfullscreen
                                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                    />
                                </div>

                                {/* Episode Title */}
                                <h1 class="text-2xl md:text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent mb-6">
                                    {episodeData().data.episode}
                                </h1>

                                {/* Navigation */}
                                <div class="flex items-center justify-between gap-4 glass-card rounded-xl p-4 mb-6">
                                    <Show when={episodeData().data.has_previous_episode && episodeData().data.previous_episode} fallback={<div />}>
                                        <A
                                            href={`/anime/full/${episodeData().data.previous_episode!.slug}`}
                                            class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
                                        >
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                            </svg>
                                            Previous
                                        </A>
                                    </Show>

                                    <div class="text-center flex-1">
                                        <span class="text-sm text-muted-foreground">Episode</span>
                                        <p class="font-bold text-lg">{episodeData().data.episode_number}</p>
                                    </div>

                                    <Show when={episodeData().data.has_next_episode && episodeData().data.next_episode}>
                                        <A
                                            href={`/anime/full/${episodeData().data.next_episode!.slug}`}
                                            class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors shadow-lg shadow-primary/25"
                                        >
                                            Next
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                            </svg>
                                        </A>
                                    </Show>
                                </div>

                                {/* Download Links */}
                                <Show when={episodeData().data.download_urls && Object.keys(episodeData().data.download_urls!).length > 0}>
                                    <div class="glass-card rounded-2xl overflow-hidden">
                                        {/* Header */}
                                        <div class="bg-gradient-to-r from-blue-500/10 via-purple-500/10 to-blue-500/10 p-5 border-b border-white/10">
                                            <h3 class="text-xl font-bold flex items-center gap-3">
                                                <div class="w-10 h-10 rounded-xl bg-blue-500/20 flex items-center justify-center">
                                                    <svg class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                                    </svg>
                                                </div>
                                                Download Episode
                                            </h3>
                                        </div>

                                        {/* Resolution Groups */}
                                        <div class="p-5 space-y-5">
                                            <For each={Object.entries(episodeData().data.download_urls!)}>
                                                {([resolution, servers]) => (
                                                    <div class="bg-white/5 rounded-xl p-4 border border-white/5">
                                                        {/* Resolution Header */}
                                                        <div class="flex items-center gap-3 mb-4">
                                                            <div class={`px-3 py-1.5 rounded-lg font-bold text-sm ${resolution.includes('1080') ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white' :
                                                                resolution.includes('720') ? 'bg-gradient-to-r from-blue-500 to-cyan-500 text-white' :
                                                                    resolution.includes('480') ? 'bg-gradient-to-r from-green-500 to-emerald-500 text-white' :
                                                                        resolution.includes('360') ? 'bg-gradient-to-r from-yellow-500 to-orange-500 text-white' :
                                                                            'bg-primary/20 text-primary'
                                                                }`}>
                                                                {resolution.includes('1080') && '🎬 '}
                                                                {resolution.includes('720') && '📺 '}
                                                                {resolution.includes('480') && '📱 '}
                                                                {resolution}
                                                            </div>
                                                            <span class="text-muted-foreground text-sm">
                                                                {servers.length} server{servers.length > 1 ? 's' : ''}
                                                            </span>
                                                        </div>

                                                        {/* Server Buttons */}
                                                        <div class="flex flex-wrap gap-2">
                                                            <For each={servers}>
                                                                {(dl) => (
                                                                    <a
                                                                        href={dl.url}
                                                                        target="_blank"
                                                                        rel="noopener noreferrer"
                                                                        class="group flex items-center gap-2 px-4 py-2.5 rounded-lg bg-secondary/80 hover:bg-primary hover:text-primary-foreground text-sm font-medium transition-all duration-200 hover-lift"
                                                                    >
                                                                        <svg class="w-4 h-4 text-muted-foreground group-hover:text-primary-foreground transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
                                                                        </svg>
                                                                        {dl.server}
                                                                    </a>
                                                                )}
                                                            </For>
                                                        </div>
                                                    </div>
                                                )}
                                            </For>
                                        </div>
                                    </div>
                                </Show>
                            </Motion.div>
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
                .glass-card {
                    background: rgba(255, 255, 255, 0.05);
                    backdrop-filter: blur(10px);
                    border: 1px solid rgba(255, 255, 255, 0.1);
                }
            `}</style>
        </>
    );
}
