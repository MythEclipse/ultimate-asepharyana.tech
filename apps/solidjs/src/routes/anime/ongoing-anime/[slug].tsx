import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

// Matches OpenAPI OngoingAnimeItem schema
interface OngoingAnimeItem {
    title: string;
    slug: string;
    poster: string;
    current_episode: string;
    anime_url: string;
}

// Matches OpenAPI Pagination schema
interface Pagination {
    current_page: number;
    last_visible_page: number;
    has_next_page: boolean;
    has_previous_page: boolean;
    next_page?: number | null;
    previous_page?: number | null;
}

// Matches OpenAPI OngoingAnimeResponse schema
interface OngoingAnimeResponse {
    status: string;
    data: OngoingAnimeItem[];
    pagination: Pagination;
}

async function fetchOngoingAnime(page: string): Promise<OngoingAnimeResponse> {
    return httpClient.fetchJson<OngoingAnimeResponse>(`/api/anime/ongoing-anime/${page}`);
}

export default function OngoingAnimePage() {
    const params = useParams();
    const page = () => params.slug || "1";
    const [data] = createResource(page, fetchOngoingAnime);

    return (
        <>
            <Title>Ongoing Anime - Page {page()} | Asepharyana</Title>
            <main class="p-4 md:p-8 min-h-screen bg-background text-foreground">
                <div class="max-w-7xl mx-auto">
                    <h1 class="text-3xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        Ongoing Anime
                    </h1>

                    <Suspense fallback={
                        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
                            <For each={Array(18).fill(0)}>
                                {() => <div class="aspect-[3/4] rounded-xl bg-muted animate-pulse" />}
                            </For>
                        </div>
                    }>
                        <Show when={data()}>
                            {(listData) => (
                                <>
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4 mb-8">
                                        <For each={listData().data}>
                                            {(item) => (
                                                <A
                                                    href={`/anime/detail/${item.slug}`}
                                                    class="group relative overflow-hidden rounded-xl bg-card border border-border hover:border-primary/50 transition-all"
                                                >
                                                    <div class="aspect-[3/4] overflow-hidden">
                                                        <CachedImage src={item.poster} alt={item.title} class="w-full h-full object-cover group-hover:scale-105 transition-transform" fallbackClass="w-full h-full bg-muted animate-pulse" loading="lazy" />
                                                    </div>
                                                    <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
                                                    <div class="absolute bottom-0 left-0 right-0 p-3">
                                                        <h3 class="text-white text-sm font-medium line-clamp-2">{item.title}</h3>
                                                        <Show when={item.current_episode}>
                                                            <span class="text-xs text-blue-300">{item.current_episode}</span>
                                                        </Show>
                                                    </div>
                                                </A>
                                            )}
                                        </For>
                                    </div>

                                    {/* Pagination */}
                                    <Show when={listData().pagination}>
                                        <div class="flex justify-center gap-2">
                                            <Show when={parseInt(page()) > 1}>
                                                <A href={`/anime/ongoing-anime/${parseInt(page()) - 1}`} class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent">
                                                    Previous
                                                </A>
                                            </Show>
                                            <span class="px-4 py-2">Page {page()}</span>
                                            <A href={`/anime/ongoing-anime/${parseInt(page()) + 1}`} class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent">
                                                Next
                                            </A>
                                        </div>
                                    </Show>
                                </>
                            )}
                        </Show>
                    </Suspense>
                </div>
            </main>
        </>
    );
}
