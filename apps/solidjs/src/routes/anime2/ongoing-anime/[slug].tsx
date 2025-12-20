import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";

interface ListItem {
    title: string;
    slug: string;
    poster: string;
    current_episode?: string;
}

interface ListResponse {
    status: string;
    data: ListItem[];
    pagination?: {
        current_page: number;
        total_pages: number;
    };
}

async function fetchOngoingAnime(page: string): Promise<ListResponse> {
    return httpClient.fetchJson<ListResponse>(`/api/anime2/ongoing-anime/${page}`);
}

export default function Anime2OngoingPage() {
    const params = useParams();
    const page = () => params.slug || "1";
    const [data] = createResource(page, fetchOngoingAnime);

    return (
        <>
            <Title>Ongoing Anime2 - Page {page()} | Asepharyana</Title>
            <main class="p-4 md:p-8 min-h-screen bg-background text-foreground">
                <div class="max-w-7xl mx-auto">
                    <h1 class="text-3xl font-bold mb-8 bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
                        Ongoing Anime (AlQanime)
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
                                                        <img src={item.poster} alt={item.title} class="w-full h-full object-cover group-hover:scale-105 transition-transform" loading="lazy" />
                                                    </div>
                                                    <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
                                                    <div class="absolute bottom-0 left-0 right-0 p-3">
                                                        <h3 class="text-white text-sm font-medium line-clamp-2">{item.title}</h3>
                                                        <Show when={item.current_episode}>
                                                            <span class="text-xs text-purple-300">{item.current_episode}</span>
                                                        </Show>
                                                    </div>
                                                </A>
                                            )}
                                        </For>
                                    </div>

                                    <Show when={listData().pagination}>
                                        <div class="flex justify-center gap-2">
                                            <Show when={parseInt(page()) > 1}>
                                                <A href={`/anime2/ongoing-anime/${parseInt(page()) - 1}`} class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent">
                                                    Previous
                                                </A>
                                            </Show>
                                            <span class="px-4 py-2">Page {page()}</span>
                                            <A href={`/anime2/ongoing-anime/${parseInt(page()) + 1}`} class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent">
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
