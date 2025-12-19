import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense } from "solid-js";
import { httpClient } from "~/lib/http-client";

interface KomikDetail {
    title: string;
    poster: string;
    alternative_title: string;
    score: string;
    status: string;
    type: string;
    author: string;
    release_date: string;
    genre: Array<{ name: string; slug: string }>;
    synopsis: string;
    chapter_list: Array<{
        chapter: string;
        slug: string;
        date: string;
    }>;
}

interface DetailResponse {
    status: string;
    data: KomikDetail;
}

async function fetchKomikDetail(slug: string): Promise<DetailResponse> {
    return httpClient.fetchJson<DetailResponse>(`/api/komik/detail/${slug}`);
}

export default function KomikDetailPage() {
    const params = useParams();
    const [data] = createResource(() => params.komikId, fetchKomikDetail);

    return (
        <>
            <Title>{data()?.data.title || "Komik Detail"} | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground">
                <Suspense fallback={
                    <div class="p-8 max-w-6xl mx-auto">
                        <div class="flex flex-col md:flex-row gap-8">
                            <div class="w-full md:w-1/3 aspect-[3/4] rounded-xl bg-muted animate-pulse" />
                            <div class="flex-1 space-y-4">
                                <div class="h-8 w-3/4 bg-muted animate-pulse rounded" />
                                <div class="h-32 bg-muted animate-pulse rounded" />
                            </div>
                        </div>
                    </div>
                }>
                    <Show when={data()}>
                        {(detailData) => (
                            <div class="p-4 md:p-8 max-w-6xl mx-auto">
                                <div class="flex flex-col md:flex-row gap-8 mb-8">
                                    <div class="w-full md:w-1/3">
                                        <img src={detailData().data.poster} alt={detailData().data.title} class="w-full rounded-xl shadow-lg" />
                                    </div>

                                    <div class="flex-1">
                                        <h1 class="text-3xl font-bold mb-2">{detailData().data.title}</h1>
                                        <p class="text-muted-foreground mb-4">{detailData().data.alternative_title}</p>

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
                                                <span class="text-muted-foreground text-sm">Author</span>
                                                <p class="font-semibold">{detailData().data.author}</p>
                                            </div>
                                        </div>

                                        <div class="flex flex-wrap gap-2 mb-6">
                                            <For each={detailData().data.genre}>
                                                {(g) => (
                                                    <span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-sm">{g.name}</span>
                                                )}
                                            </For>
                                        </div>

                                        <div>
                                            <h3 class="font-semibold mb-2">Synopsis</h3>
                                            <p class="text-muted-foreground leading-relaxed">{detailData().data.synopsis}</p>
                                        </div>
                                    </div>
                                </div>

                                <div>
                                    <h2 class="text-2xl font-bold mb-4">Chapters</h2>
                                    <div class="space-y-2 max-h-96 overflow-y-auto">
                                        <For each={detailData().data.chapter_list}>
                                            {(ch) => (
                                                <A
                                                    href={`/komik/chapter/${ch.slug}`}
                                                    class="flex justify-between items-center p-3 rounded-lg bg-card border border-border hover:border-primary/50 hover:bg-accent transition-all"
                                                >
                                                    <span class="font-medium">{ch.chapter}</span>
                                                    <span class="text-sm text-muted-foreground">{ch.date}</span>
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
