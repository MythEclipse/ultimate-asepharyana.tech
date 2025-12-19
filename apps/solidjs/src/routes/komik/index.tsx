import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { createSignal, createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";

interface KomikItem {
    title: string;
    slug: string;
    poster: string;
    type: string;
    chapter?: string;
    score?: string;
}

interface KomikHomeData {
    status: string;
    data: {
        popular: KomikItem[];
        latest: KomikItem[];
    };
}

async function fetchKomikData(): Promise<KomikHomeData> {
    return httpClient.fetchJson<KomikHomeData>("/api/komik");
}

function KomikCard(props: { item: KomikItem }) {
    return (
        <A
            href={`/komik/detail/${props.item.slug}`}
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
            <div class="absolute top-2 right-2">
                <span class="px-2 py-1 rounded text-xs font-medium bg-primary text-primary-foreground">
                    {props.item.type}
                </span>
            </div>
            <div class="absolute bottom-0 left-0 right-0 p-3">
                <h3 class="text-white text-sm font-medium line-clamp-2">{props.item.title}</h3>
                <Show when={props.item.chapter}>
                    <span class="text-xs text-blue-300">{props.item.chapter}</span>
                </Show>
            </div>
        </A>
    );
}

function KomikGrid(props: { items: KomikItem[]; loading?: boolean }) {
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
                    {(item) => <KomikCard item={item} />}
                </For>
            </Show>
        </div>
    );
}

export default function KomikPage() {
    const [data] = createResource(fetchKomikData);

    return (
        <>
            <Title>Komik | Asepharyana</Title>
            <main class="p-4 md:p-8 lg:p-12 bg-background text-foreground min-h-screen">
                <div class="max-w-7xl mx-auto">
                    <h1 class="text-4xl font-bold mb-8 bg-gradient-to-r from-orange-600 to-red-600 bg-clip-text text-transparent">
                        Komik
                    </h1>

                    {/* Search and Filter */}
                    <div class="mb-8 flex flex-col sm:flex-row gap-4">
                        <form action="/komik/search" method="get" class="flex-1 flex gap-2">
                            <input
                                type="text"
                                name="q"
                                placeholder="Search manga, manhwa, manhua..."
                                class="flex-1 px-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary"
                            />
                            <button
                                type="submit"
                                class="px-6 py-3 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
                            >
                                Search
                            </button>
                        </form>
                        <div class="flex gap-2">
                            <A href="/komik/manga/page/1" class="px-4 py-3 rounded-lg bg-card border border-border hover:bg-accent transition-colors">Manga</A>
                            <A href="/komik/manhwa/page/1" class="px-4 py-3 rounded-lg bg-card border border-border hover:bg-accent transition-colors">Manhwa</A>
                            <A href="/komik/manhua/page/1" class="px-4 py-3 rounded-lg bg-card border border-border hover:bg-accent transition-colors">Manhua</A>
                        </div>
                    </div>

                    <Suspense fallback={
                        <div class="space-y-12">
                            <section>
                                <h2 class="text-2xl font-bold mb-6">Popular</h2>
                                <KomikGrid items={[]} loading={true} />
                            </section>
                        </div>
                    }>
                        <Show when={data.error}>
                            <div class="text-center py-12 text-destructive">
                                Failed to load komik data. Please try again later.
                            </div>
                        </Show>

                        <Show when={data()}>
                            {(komikData) => (
                                <div class="space-y-12">
                                    {/* Popular */}
                                    <section>
                                        <div class="flex items-center justify-between mb-6">
                                            <h2 class="text-2xl font-bold bg-gradient-to-r from-orange-600 to-red-600 bg-clip-text text-transparent">
                                                🔥 Popular
                                            </h2>
                                        </div>
                                        <KomikGrid items={komikData().data.popular} />
                                    </section>

                                    {/* Latest */}
                                    <section>
                                        <div class="flex items-center justify-between mb-6">
                                            <h2 class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                                📖 Latest Updates
                                            </h2>
                                        </div>
                                        <KomikGrid items={komikData().data.latest} />
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
