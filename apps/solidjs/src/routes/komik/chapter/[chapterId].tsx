import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense, createSignal } from "solid-js";
import { httpClient } from "~/lib/http-client";

interface ChapterData {
    title: string;
    images: string[];
    prev_chapter?: { slug: string };
    next_chapter?: { slug: string };
}

interface ChapterResponse {
    status: string;
    data: ChapterData;
}

async function fetchChapter(slug: string): Promise<ChapterResponse> {
    return httpClient.fetchJson<ChapterResponse>(`/api/komik/chapter/${slug}`);
}

export default function KomikChapterPage() {
    const params = useParams();
    const [data] = createResource(() => params.chapterId, fetchChapter);

    return (
        <>
            <Title>{data()?.data.title || "Chapter"} | Asepharyana</Title>
            <main class="min-h-screen bg-black text-white">
                <Suspense fallback={
                    <div class="flex items-center justify-center min-h-screen">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-white" />
                    </div>
                }>
                    <Show when={data()}>
                        {(chapterData) => (
                            <>
                                {/* Header */}
                                <div class="sticky top-0 z-50 bg-black/90 backdrop-blur border-b border-white/10 p-4">
                                    <div class="max-w-4xl mx-auto flex items-center justify-between">
                                        <h1 class="text-lg font-medium truncate">{chapterData().data.title}</h1>
                                        <div class="flex gap-2">
                                            <Show when={chapterData().data.prev_chapter}>
                                                <A href={`/komik/chapter/${chapterData().data.prev_chapter!.slug}`} class="px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 transition-colors">
                                                    ← Prev
                                                </A>
                                            </Show>
                                            <Show when={chapterData().data.next_chapter}>
                                                <A href={`/komik/chapter/${chapterData().data.next_chapter!.slug}`} class="px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 transition-colors">
                                                    Next →
                                                </A>
                                            </Show>
                                        </div>
                                    </div>
                                </div>

                                {/* Images */}
                                <div class="max-w-4xl mx-auto">
                                    <For each={chapterData().data.images}>
                                        {(img, i) => (
                                            <img
                                                src={img}
                                                alt={`Page ${i() + 1}`}
                                                class="w-full"
                                                loading="lazy"
                                            />
                                        )}
                                    </For>
                                </div>

                                {/* Footer Navigation */}
                                <div class="py-8 bg-black/90 border-t border-white/10">
                                    <div class="max-w-4xl mx-auto flex justify-center gap-4 px-4">
                                        <Show when={chapterData().data.prev_chapter}>
                                            <A href={`/komik/chapter/${chapterData().data.prev_chapter!.slug}`} class="px-6 py-3 rounded-lg bg-white/10 hover:bg-white/20 transition-colors">
                                                ← Previous Chapter
                                            </A>
                                        </Show>
                                        <Show when={chapterData().data.next_chapter}>
                                            <A href={`/komik/chapter/${chapterData().data.next_chapter!.slug}`} class="px-6 py-3 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors">
                                                Next Chapter →
                                            </A>
                                        </Show>
                                    </div>
                                </div>
                            </>
                        )}
                    </Show>
                </Suspense>
            </main>
        </>
    );
}
