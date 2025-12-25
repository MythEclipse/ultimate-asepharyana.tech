import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

interface ListItem {
    title: string;
    slug: string;
    poster: string;
    episode_count?: string;
}

interface ListResponse {
    status: string;
    data: ListItem[];
}

async function fetchCompleteAnime(page: string): Promise<ListResponse> {
    return httpClient.fetchJson<ListResponse>(`/api/anime2/complete-anime/${page}`);
}

export default function Anime2CompletePage() {
    const params = useParams();
    const page = () => params.slug || "1";
    const [data] = createResource(page, fetchCompleteAnime);

    return (
        <>
            <Title>Complete Anime2 - Page {page()} | Asepharyana</Title>
            <main class="p-4 md:p-8 min-h-screen bg-background text-foreground">
                <div class="max-w-7xl mx-auto">
                    <h1 class="text-3xl font-bold mb-8 bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent">
                        Complete Anime (AlQanime)
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
                                                    href={`/anime2/detail/${item.slug}`}
                                                    class="group relative overflow-hidden rounded-xl bg-card border border-border hover:border-primary/50 transition-all"
                                                >
                                                    <div class="aspect-[3/4] overflow-hidden">
                                                        <CachedImage src={item.poster} alt={item.title} class="w-full h-full object-cover group-hover:scale-105 transition-transform" fallbackClass="w-full h-full bg-muted animate-pulse" loading="lazy" />
                                                    </div>
                                                    <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
                                                    <div class="absolute bottom-0 left-0 right-0 p-3">
                                                        <h3 class="text-white text-sm font-medium line-clamp-2">{item.title}</h3>
                                                        <Show when={item.episode_count}>
                                                            <span class="text-xs text-pink-300">{item.episode_count} eps</span>
                                                        </Show>
                                                    </div>
                                                </A>
                                            )}
                                        </For>
                                    </div>

                                    <div class="flex justify-center gap-2">
                                        <Show when={parseInt(page()) > 1}>
                                            <A href={`/anime2/complete-anime/${parseInt(page()) - 1}`} class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent">Previous</A>
                                        </Show>
                                        <span class="px-4 py-2">Page {page()}</span>
                                        <A href={`/anime2/complete-anime/${parseInt(page()) + 1}`} class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent">Next</A>
                                    </div>
                                </>
                            )}
                        </Show>
                    </Suspense>
                </div>
            </main>
        </>
    );
}
