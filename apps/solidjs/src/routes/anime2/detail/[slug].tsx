import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense, createMemo, createSignal, onMount } from "solid-js";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

interface Genre {
    name: string;
    slug: string;
    anime_url: string;
}

interface Link {
    name: string;
    url: string;
}

interface DownloadItem {
    resolution: string;
    links: Link[];
}

interface Recommendation {
    title: string;
    slug: string;
    poster: string;
    status: string;
    type: string;
}

interface AnimeDetailData {
    title: string;
    alternative_title: string;
    poster: string;
    poster2: string;
    type: string;
    release_date: string;
    status: string;
    synopsis: string;
    studio: string;
    genres: Genre[];
    producers: string[];
    recommendations: Recommendation[];
    batch: DownloadItem[];
    ova: DownloadItem[];
    downloads: DownloadItem[];
}

interface DetailResponse {
    status: string;
    data: AnimeDetailData;
}



async function fetchAnimeDetail(slug: string): Promise<DetailResponse> {
    return httpClient.fetchJson<DetailResponse>(`/api/anime2/detail/${slug}`);
}

// Process downloads to group by episode number
function processDownloads(downloads: DownloadItem[] = []): Record<string, DownloadItem[]> {
    const episodes: Record<string, DownloadItem[]> = {};

    downloads.forEach((download) => {
        let episodeNumber = "unknown";
        for (const link of download.links) {
            // Match patterns like BD01, EP01, _01_, etc.
            const episodeMatch = link.url.match(/(?:BD|EP|_)(\d+)(?:_|\.|$)/i);
            if (episodeMatch) {
                episodeNumber = episodeMatch[1];
                break;
            }
        }

        if (!episodes[episodeNumber]) {
            episodes[episodeNumber] = [];
        }
        episodes[episodeNumber].push(download);
    });

    return episodes;
}

export default function Anime2DetailPage() {
    const params = useParams();
    const [enabled, setEnabled] = createSignal(false);
    onMount(() => setEnabled(true));

    const [data] = createResource(() => enabled() ? params.slug : null, fetchAnimeDetail);

    const groupedDownloads = createMemo(() => {
        if (!data()?.data.downloads) return {};
        return processDownloads(data()!.data.downloads);
    });

    const sortedEpisodes = createMemo(() => {
        const episodes = groupedDownloads();
        return Object.entries(episodes).sort(([a], [b]) => {
            const numA = parseInt(a) || 0;
            const numB = parseInt(b) || 0;
            return numA - numB;
        });
    });

    return (
        <>
            <Title>{data()?.data.title || "Anime Detail"} | Anime2 | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground">
                <Suspense fallback={
                    <div class="p-8 max-w-6xl mx-auto">
                        <div class="flex flex-col md:flex-row gap-8">
                            <div class="w-full md:w-1/3 aspect-[3/4] rounded-xl bg-muted animate-pulse" />
                            <div class="flex-1 space-y-4">
                                <div class="h-8 w-3/4 bg-muted animate-pulse rounded" />
                                <div class="h-4 w-1/2 bg-muted animate-pulse rounded" />
                                <div class="h-32 bg-muted animate-pulse rounded" />
                            </div>
                        </div>
                    </div>
                }>
                    <Show when={data.error}>
                        <div class="p-8 text-center text-destructive">
                            Failed to load anime details. Please try again later.
                        </div>
                    </Show>

                    <Show when={data()}>
                        {(detailData) => (
                            <div class="p-4 md:p-8 max-w-6xl mx-auto">
                                {/* Hero Section */}
                                <div class="flex flex-col md:flex-row gap-8 mb-8">
                                    {/* Left: Poster & Info Card */}
                                    <div class="w-full md:w-1/3 space-y-4 md:sticky top-8">
                                        {/* Poster */}
                                        <div class="rounded-xl overflow-hidden shadow-lg">
                                            <CachedImage
                                                src={detailData().data.poster || detailData().data.poster2}
                                                alt={detailData().data.title}
                                                class="w-full"
                                                fallbackClass="w-full aspect-[3/4] bg-muted animate-pulse"
                                            />
                                        </div>

                                        {/* Info Card */}
                                        <div class="glass-card rounded-xl p-4 space-y-3 text-sm">
                                            <h3 class="font-semibold">Informasi</h3>
                                            <Show when={detailData().data.type}>
                                                <div class="flex items-center gap-3">
                                                    <span class="p-2 bg-primary/10 rounded-lg">📺</span>
                                                    <div>
                                                        <p class="text-muted-foreground text-xs">Tipe</p>
                                                        <p class="font-medium">{detailData().data.type}</p>
                                                    </div>
                                                </div>
                                            </Show>
                                            <Show when={detailData().data.status}>
                                                <div class="flex items-center gap-3">
                                                    <span class="p-2 bg-green-500/10 rounded-lg">🟢</span>
                                                    <div>
                                                        <p class="text-muted-foreground text-xs">Status</p>
                                                        <p class="font-medium">{detailData().data.status}</p>
                                                    </div>
                                                </div>
                                            </Show>
                                            <Show when={detailData().data.release_date}>
                                                <div class="flex items-center gap-3">
                                                    <span class="p-2 bg-red-500/10 rounded-lg">📅</span>
                                                    <div>
                                                        <p class="text-muted-foreground text-xs">Rilis</p>
                                                        <p class="font-medium">{detailData().data.release_date}</p>
                                                    </div>
                                                </div>
                                            </Show>
                                            <Show when={detailData().data.studio}>
                                                <div class="flex items-center gap-3">
                                                    <span class="p-2 bg-purple-500/10 rounded-lg">🎬</span>
                                                    <div>
                                                        <p class="text-muted-foreground text-xs">Studio</p>
                                                        <p class="font-medium">{detailData().data.studio}</p>
                                                    </div>
                                                </div>
                                            </Show>
                                        </div>
                                    </div>

                                    {/* Right: Content */}
                                    <div class="w-full md:w-2/3 space-y-6">
                                        {/* Title */}
                                        <div class="space-y-2">
                                            <h1 class="text-3xl md:text-4xl font-bold text-foreground">
                                                {detailData().data.title}
                                            </h1>
                                            <Show when={detailData().data.alternative_title}>
                                                <p class="text-xl text-muted-foreground">
                                                    {detailData().data.alternative_title}
                                                </p>
                                            </Show>
                                        </div>

                                        {/* Genres */}
                                        <Show when={detailData().data.genres?.length > 0}>
                                            <div class="flex flex-wrap gap-2">
                                                <For each={detailData().data.genres}>
                                                    {(g) => (
                                                        <span class="px-3 py-1 rounded-full bg-purple-500/10 text-purple-600 dark:text-purple-400 text-sm font-medium">
                                                            {g.name}
                                                        </span>
                                                    )}
                                                </For>
                                            </div>
                                        </Show>

                                        {/* Synopsis */}
                                        <Show when={detailData().data.synopsis}>
                                            <div class="glass-card rounded-xl p-6">
                                                <h3 class="font-semibold mb-2">Sinopsis</h3>
                                                <p class="text-muted-foreground leading-relaxed">
                                                    {detailData().data.synopsis}
                                                </p>
                                            </div>
                                        </Show>

                                        {/* Batch Downloads */}
                                        <Show when={detailData().data.batch?.length > 0}>
                                            <div class="glass-card rounded-xl p-6">
                                                <h3 class="font-semibold mb-4 flex items-center gap-2">
                                                    📦 Unduhan Batch
                                                </h3>
                                                <div class="space-y-4">
                                                    <For each={detailData().data.batch}>
                                                        {(item) => (
                                                            <div>
                                                                <h4 class="font-medium mb-2">{item.resolution}</h4>
                                                                <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
                                                                    <For each={item.links}>
                                                                        {(link) => (
                                                                            <a
                                                                                href={link.url}
                                                                                target="_blank"
                                                                                rel="noopener noreferrer"
                                                                                class="flex items-center justify-center gap-2 px-3 py-2 rounded-lg bg-green-500/10 text-green-600 dark:text-green-400 hover:bg-green-500/20 text-sm font-medium transition-colors"
                                                                            >
                                                                                🖥️ {link.name}
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

                                        {/* Episode Downloads (sorted by episode number) */}
                                        <Show when={sortedEpisodes().length > 0}>
                                            <div class="glass-card rounded-xl p-6">
                                                <h3 class="font-semibold mb-4 flex items-center gap-2">
                                                    🎬 Daftar Episode
                                                </h3>
                                                <div class="space-y-4">
                                                    <For each={sortedEpisodes()}>
                                                        {([episodeNumber, resolutions]) => (
                                                            <details class="group">
                                                                <summary class="cursor-pointer list-none flex items-center justify-between p-3 rounded-lg bg-muted/50 hover:bg-muted transition-colors">
                                                                    <span class="font-medium">
                                                                        Episode {episodeNumber === "unknown" ? "?" : episodeNumber}
                                                                    </span>
                                                                    <svg class="w-5 h-5 transform group-open:rotate-180 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                                                                    </svg>
                                                                </summary>
                                                                <div class="mt-3 space-y-3 pl-4">
                                                                    <For each={resolutions}>
                                                                        {(res) => (
                                                                            <div>
                                                                                <h4 class="text-sm font-medium text-muted-foreground mb-2">
                                                                                    {res.resolution}
                                                                                </h4>
                                                                                <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
                                                                                    <For each={res.links}>
                                                                                        {(link) => (
                                                                                            <a
                                                                                                href={link.url}
                                                                                                target="_blank"
                                                                                                rel="noopener noreferrer"
                                                                                                class="flex items-center justify-center gap-2 px-3 py-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 text-sm transition-colors"
                                                                                            >
                                                                                                ⬇️ {link.name}
                                                                                            </a>
                                                                                        )}
                                                                                    </For>
                                                                                </div>
                                                                            </div>
                                                                        )}
                                                                    </For>
                                                                </div>
                                                            </details>
                                                        )}
                                                    </For>
                                                </div>
                                            </div>
                                        </Show>

                                        {/* OVA Downloads */}
                                        <Show when={detailData().data.ova?.length > 0}>
                                            <div class="glass-card rounded-xl p-6">
                                                <h3 class="font-semibold mb-4 flex items-center gap-2">
                                                    🎥 OVA
                                                </h3>
                                                <div class="space-y-4">
                                                    <For each={detailData().data.ova}>
                                                        {(item) => (
                                                            <div>
                                                                <h4 class="font-medium mb-2">{item.resolution}</h4>
                                                                <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
                                                                    <For each={item.links}>
                                                                        {(link) => (
                                                                            <a
                                                                                href={link.url}
                                                                                target="_blank"
                                                                                rel="noopener noreferrer"
                                                                                class="flex items-center justify-center gap-2 px-3 py-2 rounded-lg bg-blue-500/10 text-blue-600 dark:text-blue-400 hover:bg-blue-500/20 text-sm font-medium transition-colors"
                                                                            >
                                                                                🖥️ {link.name}
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

                                        {/* Recommendations */}
                                        <Show when={detailData().data.recommendations?.length > 0}>
                                            <div>
                                                <h2 class="text-2xl font-bold mb-4">Rekomendasi</h2>
                                                <div class="flex overflow-x-auto pb-4 gap-4 -mx-2 px-2">
                                                    <For each={detailData().data.recommendations}>
                                                        {(rec) => (
                                                            <A
                                                                href={`/anime2/detail/${rec.slug}`}
                                                                class="flex-shrink-0 w-36 md:w-44 group"
                                                            >
                                                                <div class="relative overflow-hidden rounded-xl bg-card border border-border hover:border-primary/50 transition-all">
                                                                    <div class="aspect-[3/4] overflow-hidden">
                                                                        <CachedImage
                                                                            src={rec.poster}
                                                                            alt={rec.title}
                                                                            class="w-full h-full object-cover group-hover:scale-105 transition-transform"
                                                                            fallbackClass="w-full h-full bg-muted animate-pulse"
                                                                            loading="lazy"
                                                                        />
                                                                    </div>
                                                                    <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
                                                                    <div class="absolute top-2 left-2">
                                                                        <Show when={rec.type}>
                                                                            <span class="px-2 py-0.5 rounded bg-primary text-primary-foreground text-xs">
                                                                                {rec.type}
                                                                            </span>
                                                                        </Show>
                                                                    </div>
                                                                    <div class="absolute bottom-0 left-0 right-0 p-3">
                                                                        <h3 class="text-white text-sm font-medium line-clamp-2">{rec.title}</h3>
                                                                    </div>
                                                                </div>
                                                            </A>
                                                        )}
                                                    </For>
                                                </div>
                                            </div>
                                        </Show>
                                    </div>
                                </div>

                                {/* Back Button */}
                                <div class="mt-8">
                                    <A
                                        href="/anime2"
                                        class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
                                    >
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                        </svg>
                                        Kembali ke Anime2
                                    </A>
                                </div>
                            </div>
                        )}
                    </Show>
                </Suspense>
            </main>
        </>
    );
}
