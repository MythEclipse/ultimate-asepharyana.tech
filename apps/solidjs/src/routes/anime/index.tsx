import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { createSignal, createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";

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

async function fetchAnimeData(): Promise<HomeData> {
    return httpClient.fetchJson<HomeData>("/api/anime");
}

function AnimeCard(props: { item: AnimeItem }) {
    return (
        <A
            href={`/anime/detail/${props.item.slug}`}
            class="group relative overflow-hidden rounded-xl bg-card border border-border shadow-sm hover:shadow-lg transition-all hover:border-primary/50"
        >
            <div class="aspect-[3/4] overflow-hidden">
                <img
                    src={props.item.poster}
                    alt={props.item.title}
                    class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                    loading="lazy"
                />
            </div>
            <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
            <div class="absolute bottom-0 left-0 right-0 p-3">
                <h3 class="text-white text-sm font-medium line-clamp-2">{props.item.title}</h3>
                <Show when={props.item.current_episode}>
                    <span class="text-xs text-blue-300">{props.item.current_episode}</span>
                </Show>
            </div>
        </A>
    );
}

function AnimeGrid(props: { items: AnimeItem[]; loading?: boolean }) {
    return (
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
            <Show when={props.loading}>
                <For each={Array(12).fill(0)}>
                    {() => (
                        <div class="aspect-[3/4] rounded-xl bg-muted animate-pulse" />
                    )}
                </For>
            </Show>
            <Show when={!props.loading}>
                <For each={props.items}>
                    {(item) => <AnimeCard item={item} />}
                </For>
            </Show>
        </div>
    );
}

export default function AnimePage() {
    const [data] = createResource(fetchAnimeData);

    return (
        <>
            <Title>Anime | Asepharyana</Title>
            <main class="p-4 md:p-8 lg:p-12 bg-background text-foreground min-h-screen">
                <div class="max-w-7xl mx-auto">
                    <h1 class="text-4xl font-bold mb-8 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        Anime
                    </h1>

                    {/* Search Bar */}
                    <div class="mb-8">
                        <form action="/anime/search" method="get" class="flex gap-2">
                            <input
                                type="text"
                                name="q"
                                placeholder="Search anime..."
                                class="flex-1 px-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary"
                            />
                            <button
                                type="submit"
                                class="px-6 py-3 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
                            >
                                Search
                            </button>
                        </form>
                    </div>

                    <Suspense fallback={
                        <div class="space-y-12">
                            <section>
                                <div class="flex items-center gap-3 mb-6">
                                    <div class="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
                                        <svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />
                                        </svg>
                                    </div>
                                    <h2 class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                        Ongoing Anime
                                    </h2>
                                </div>
                                <AnimeGrid items={[]} loading={true} />
                            </section>
                        </div>
                    }>
                        <Show when={data.error}>
                            <div class="text-center py-12 text-destructive">
                                Failed to load anime data. Please try again later.
                            </div>
                        </Show>

                        <Show when={data()}>
                            {(animeData) => (
                                <div class="space-y-12">
                                    {/* Ongoing Anime */}
                                    <section>
                                        <div class="flex items-center justify-between mb-6">
                                            <div class="flex items-center gap-3">
                                                <div class="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-xl">
                                                    <svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />
                                                    </svg>
                                                </div>
                                                <h2 class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                                    Ongoing Anime
                                                </h2>
                                            </div>
                                            <A
                                                href="/anime/ongoing-anime/1"
                                                class="flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 transition-colors"
                                            >
                                                View All
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                                </svg>
                                            </A>
                                        </div>
                                        <AnimeGrid items={animeData().data.ongoing_anime} />
                                    </section>

                                    {/* Complete Anime */}
                                    <section>
                                        <div class="flex items-center justify-between mb-6">
                                            <div class="flex items-center gap-3">
                                                <div class="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
                                                    <svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                    </svg>
                                                </div>
                                                <h2 class="text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent">
                                                    Complete Anime
                                                </h2>
                                            </div>
                                            <A
                                                href="/anime/complete-anime/1"
                                                class="flex items-center gap-2 text-green-600 dark:text-green-400 hover:text-green-700 transition-colors"
                                            >
                                                View All
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                                </svg>
                                            </A>
                                        </div>
                                        <AnimeGrid items={animeData().data.complete_anime} />
                                    </section>
                                </div>
                            )}
                        </Show>
                    </Suspense>
                </div>
            </main>
        </>
    );
}
