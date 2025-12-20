import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense, createSignal } from "solid-js";
import { Motion } from "solid-motionone";
import { httpClient } from "~/lib/http-client";

interface StreamServer {
    name: string;
    slug: string;
}

interface DownloadServer {
    server: string;
    url: string;
}

interface EpisodeData {
    title: string;
    poster: string;
    stream_url: string;
    download_urls?: Record<string, DownloadServer[]>;
    default_server: {
        name: string;
        slug: string;
        url: string;
    };
    server_list: StreamServer[];
    prev_episode?: { title: string; slug: string };
    next_episode?: { title: string; slug: string };
}

interface EpisodeResponse {
    status: string;
    data: EpisodeData;
}

async function fetchEpisode(slug: string): Promise<EpisodeResponse> {
    return httpClient.fetchJson<EpisodeResponse>(`/api/anime/full/${slug}`);
}

async function fetchServer(slug: string): Promise<{ status: string; data: { url: string } }> {
    return httpClient.fetchJson<{ status: string; data: { url: string } }>(`/api/anime/server/${slug}`);
}

export default function AnimeFullPage() {
    const params = useParams();
    const [data] = createResource(() => params.slug, fetchEpisode);
    const [currentServer, setCurrentServer] = createSignal<string>("");
    const [iframeUrl, setIframeUrl] = createSignal<string>("");

    const handleServerChange = async (serverSlug: string) => {
        setCurrentServer(serverSlug);
        try {
            const res = await fetchServer(serverSlug);
            setIframeUrl(res.data.url);
        } catch (e) {
            console.error("Failed to fetch server URL:", e);
        }
    };

    return (
        <>
            <Title>{data()?.data.title || "Streaming"} | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground">
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
                                <div class="aspect-video w-full rounded-2xl overflow-hidden bg-black mb-6 shadow-2xl">
                                    <iframe
                                        src={iframeUrl() || episodeData().data.default_server?.url || episodeData().data.stream_url}
                                        class="w-full h-full"
                                        allowfullscreen
                                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                    />
                                </div>

                                {/* Title */}
                                <h1 class="text-2xl md:text-3xl font-bold gradient-text mb-6">
                                    {episodeData().data.title}
                                </h1>

                                {/* Server Selection */}
                                <Show when={episodeData().data.server_list?.length > 0}>
                                    <div class="mb-6">
                                        <h3 class="text-sm font-medium text-muted-foreground mb-3">Servers</h3>
                                        <div class="flex flex-wrap gap-2">
                                            <For each={episodeData().data.server_list}>
                                                {(server) => (
                                                    <button
                                                        onClick={() => handleServerChange(server.slug)}
                                                        class={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${currentServer() === server.slug
                                                            ? "bg-primary text-primary-foreground shadow-lg shadow-primary/25"
                                                            : "glass-subtle hover:bg-white/10"
                                                            }`}
                                                    >
                                                        {server.name}
                                                    </button>
                                                )}
                                            </For>
                                        </div>
                                    </div>
                                </Show>

                                {/* Navigation */}
                                <div class="flex items-center justify-between gap-4 glass-card rounded-xl p-4 mb-6">
                                    <Show when={episodeData().data.prev_episode} fallback={<div />}>
                                        <A
                                            href={`/anime/full/${episodeData().data.prev_episode!.slug}`}
                                            class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
                                        >
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                            </svg>
                                            Previous
                                        </A>
                                    </Show>

                                    <Show when={episodeData().data.next_episode}>
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
                                    <div class="glass-card rounded-xl p-6">
                                        <h3 class="font-semibold mb-4 flex items-center gap-2">
                                            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                            </svg>
                                            Download
                                        </h3>
                                        <div class="space-y-4">
                                            <For each={Object.entries(episodeData().data.download_urls!)}>
                                                {([resolution, servers]) => (
                                                    <div>
                                                        <h4 class="text-sm font-medium text-muted-foreground mb-2">{resolution}</h4>
                                                        <div class="flex flex-wrap gap-2">
                                                            <For each={servers}>
                                                                {(dl) => (
                                                                    <a
                                                                        href={dl.url}
                                                                        target="_blank"
                                                                        rel="noopener noreferrer"
                                                                        class="px-4 py-2 rounded-lg glass-subtle hover:bg-white/10 text-sm transition-colors"
                                                                    >
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
        </>
    );
}
