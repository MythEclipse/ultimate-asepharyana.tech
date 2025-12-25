import { Title } from "@solidjs/meta";
import { A, useParams, useSearchParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

interface SearchResult {
    title: string;
    slug: string;
    poster: string;
    current_episode?: string;
}

interface SearchResponse {
    status: string;
    data: SearchResult[];
}

async function searchAnime(query: string): Promise<SearchResponse> {
    return httpClient.fetchJson<SearchResponse>(`/api/anime/search?q=${encodeURIComponent(query)}`);
}

export default function AnimeSearchPage() {
    const params = useParams();
    const [searchParams] = useSearchParams();
    const query = () => params.slug || searchParams.q || "";

    const [data] = createResource(query, searchAnime);

    return (
        <>
            <Title>Search: {query()} | Anime | Asepharyana</Title>
            <main class="p-4 md:p-8 min-h-screen bg-background text-foreground">
                <div class="max-w-7xl mx-auto">
                    <h1 class="text-3xl font-bold mb-2">Search Results</h1>
                    <p class="text-muted-foreground mb-8">Results for "{query()}"</p>

                    <Suspense fallback={
                        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                            <For each={Array(10).fill(0)}>
                                {() => <div class="aspect-[3/4] rounded-xl bg-muted animate-pulse" />}
                            </For>
                        </div>
                    }>
                        <Show when={data.error}>
                            <div class="text-center py-12 text-destructive">Search failed</div>
                        </Show>
                        <Show when={data()}>
                            {(searchData) => (
                                <Show when={searchData().data.length > 0} fallback={
                                    <div class="text-center py-12 text-muted-foreground">
                                        No results found for "{query()}"
                                    </div>
                                }>
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                                        <For each={searchData().data}>
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
                                                    </div>
                                                </A>
                                            )}
                                        </For>
                                    </div>
                                </Show>
                            )}
                        </Show>
                    </Suspense>
                </div>
            </main>
        </>
    );
}
