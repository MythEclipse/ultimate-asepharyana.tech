import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";

interface AnimeDetail {
    title: string;
    poster: string;
    japanese_title: string;
    score: string;
    producer: string;
    type: string;
    status: string;
    episode_count: string;
    duration: string;
    release_date: string;
    studio: string;
    genre: Array<{ name: string; slug: string }>;
    synopsis: string;
    episode_lists: Array<{
        episode: string;
        slug: string;
        date: string;
    }>;
}

interface DetailResponse {
    status: string;
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
                                    {/* Poster */}
                                    <div class="w-full md:w-1/3">
                                        <img
                                            src={detailData().data.poster}
                                            alt={detailData().data.title}
                                            class="w-full rounded-xl shadow-lg"
                                        />
                                    </div>

                                    {/* Info */}
                                    <div class="flex-1">
                                        <h1 class="text-3xl md:text-4xl font-bold mb-2">
                                            {detailData().data.title}
                                        </h1>
                                        <p class="text-muted-foreground mb-4">
                                            {detailData().data.japanese_title}
                                        </p>

                                        {/* Meta Info */}
                                        <div class="grid grid-cols-2 gap-4 mb-6">
                                            <div>
                                                <span class="text-muted-foreground text-sm">Score</span>
                                                <p class="font-semibold">{detailData().data.score}</p>
                                            </div>
                                            <div>
                                                <span class="text-muted-foreground text-sm">Type</span>
                                                <p class="font-semibold">{detailData().data.type}</p>
                                            </div>
                                            <div>
                                                <span class="text-muted-foreground text-sm">Status</span>
                                                <p class="font-semibold">{detailData().data.status}</p>
                                            </div>
                                            <div>
                                                <span class="text-muted-foreground text-sm">Episodes</span>
                                                <p class="font-semibold">{detailData().data.episode_count}</p>
                                            </div>
                                            <div>
                                                <span class="text-muted-foreground text-sm">Studio</span>
                                                <p class="font-semibold">{detailData().data.studio}</p>
                                            </div>
                                            <div>
                                                <span class="text-muted-foreground text-sm">Duration</span>
                                                <p class="font-semibold">{detailData().data.duration}</p>
                                            </div>
                                        </div>

                                        {/* Genres */}
                                        <div class="flex flex-wrap gap-2 mb-6">
                                            <For each={detailData().data.genre}>
                                                {(g) => (
                                                    <span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-sm">
                                                        {g.name}
                                                    </span>
                                                )}
                                            </For>
                                        </div>

                                        {/* Synopsis */}
                                        <div>
                                            <h3 class="font-semibold mb-2">Synopsis</h3>
                                            <p class="text-muted-foreground leading-relaxed">
                                                {detailData().data.synopsis}
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                {/* Episode List */}
                                <div>
                                    <h2 class="text-2xl font-bold mb-4">Episodes</h2>
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3">
                                        <For each={detailData().data.episode_lists}>
                                            {(ep) => (
                                                <A
                                                    href={`/anime/full/${ep.slug}`}
                                                    class="p-3 rounded-lg bg-card border border-border hover:border-primary/50 hover:bg-accent transition-all text-center"
                                                >
                                                    <span class="font-medium">{ep.episode}</span>
                                                    <p class="text-xs text-muted-foreground mt-1">{ep.date}</p>
                                                </A>
                                            )}
                                        </For>
                                    </div>
                                </div>
                            </div>
                        )}
                    </Show>
                </Suspense>
            </main>
        </>
    );
}
