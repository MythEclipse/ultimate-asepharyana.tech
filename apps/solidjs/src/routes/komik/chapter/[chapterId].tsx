import { Title } from "@solidjs/meta";
import { A, useParams } from "@solidjs/router";
import { createResource, For, Show, Suspense, createSignal, onMount, onCleanup } from "solid-js";
import { Motion } from "solid-motionone";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

// Matches OpenAPI ChapterResponse -> ChapterData schema
interface ChapterData {
    title: string;
    next_chapter_id: string;
    prev_chapter_id: string;
    list_chapter: string;
    images: string[];
}

interface ChapterResponse {
    message: string;
    data: ChapterData;
}

async function fetchChapter(slug: string): Promise<ChapterResponse> {
    return httpClient.fetchJson<ChapterResponse>(`/api/komik/chapter?chapter_url=${encodeURIComponent(slug)}`);
}

export default function KomikChapterPage() {
    const params = useParams();
    const [data] = createResource(() => params.chapterId, fetchChapter);
    const [showControls, setShowControls] = createSignal(true);
    const [currentPage, setCurrentPage] = createSignal(1);

    // Auto-hide controls after 3 seconds
    let hideTimeout: ReturnType<typeof setTimeout>;
    const resetHideTimer = () => {
        setShowControls(true);
        clearTimeout(hideTimeout);
        hideTimeout = setTimeout(() => setShowControls(false), 3000);
    };

    onMount(() => {
        resetHideTimer();
        const handleMove = () => resetHideTimer();
        window.addEventListener('mousemove', handleMove);
        window.addEventListener('touchstart', handleMove);
        onCleanup(() => {
            clearTimeout(hideTimeout);
            window.removeEventListener('mousemove', handleMove);
            window.removeEventListener('touchstart', handleMove);
        });
    });

    const handleScroll = (e: Event) => {
        const container = e.target as HTMLElement;
        const images = container.querySelectorAll('img');
        let page = 1;
        images.forEach((img, i) => {
            const rect = img.getBoundingClientRect();
            if (rect.top < window.innerHeight / 2) {
                page = i + 1;
            }
        });
        setCurrentPage(page);
    };

    return (
        <>
            <Title>{data()?.data.title || "Chapter"} | Asepharyana</Title>
            <main class="min-h-screen bg-black text-white" onScroll={handleScroll}>
                <Suspense fallback={
                    <div class="min-h-screen flex items-center justify-center bg-black">
                        <div class="flex flex-col items-center gap-4">
                            <div class="w-16 h-16 border-4 border-primary/30 border-t-primary rounded-full animate-spin" />
                            <span class="text-white/60">Loading chapter...</span>
                        </div>
                    </div>
                }>
                    <Show when={data()}>
                        {(response) => (
                            <>
                                {/* Floating Header */}
                                <Motion.div
                                    initial={{ y: -100 }}
                                    animate={{ y: showControls() ? 0 : -100 }}
                                    transition={{ duration: 0.3 }}
                                    class="fixed top-0 left-0 right-0 z-50"
                                >
                                    <div class="glass-subtle bg-black/80 backdrop-blur-xl border-b border-white/10">
                                        <div class="max-w-4xl mx-auto px-4 py-3 flex items-center justify-between">
                                            {/* Back + Title */}
                                            <div class="flex items-center gap-3 flex-1 min-w-0">
                                                <A href={response().data.list_chapter || "/komik"} class="p-2 rounded-lg bg-white/5 hover:bg-white/10 transition-colors">
                                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                                    </svg>
                                                </A>
                                                <h1 class="text-sm md:text-base font-medium truncate">{response().data.title}</h1>
                                            </div>

                                            {/* Page Indicator */}
                                            <div class="px-3 py-1 rounded-full bg-white/10 text-sm">
                                                {currentPage()} / {response().data.images?.length || 0}
                                            </div>
                                        </div>
                                    </div>
                                </Motion.div>

                                {/* Navigation Controls */}
                                <Motion.div
                                    initial={{ y: 100 }}
                                    animate={{ y: showControls() ? 0 : 100 }}
                                    transition={{ duration: 0.3 }}
                                    class="fixed bottom-0 left-0 right-0 z-50"
                                >
                                    <div class="glass-subtle bg-black/80 backdrop-blur-xl border-t border-white/10">
                                        <div class="max-w-4xl mx-auto px-4 py-3 flex items-center justify-center gap-4">
                                            <Show when={response().data.prev_chapter_id}>
                                                <A
                                                    href={`/komik/chapter/${encodeURIComponent(response().data.prev_chapter_id)}`}
                                                    class="flex items-center gap-2 px-5 py-2.5 rounded-xl bg-white/10 hover:bg-white/20 transition-all duration-200 font-medium"
                                                >
                                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                                    </svg>
                                                    <span class="hidden sm:inline">Previous</span>
                                                </A>
                                            </Show>

                                            <div class="flex items-center gap-2 px-4 py-2 rounded-xl bg-primary/20 text-primary">
                                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
                                                </svg>
                                                <span class="font-medium">{currentPage()}</span>
                                            </div>

                                            <Show when={response().data.next_chapter_id}>
                                                <A
                                                    href={`/komik/chapter/${encodeURIComponent(response().data.next_chapter_id)}`}
                                                    class="flex items-center gap-2 px-5 py-2.5 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 transition-all duration-200 font-medium"
                                                >
                                                    <span class="hidden sm:inline">Next</span>
                                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                                    </svg>
                                                </A>
                                            </Show>
                                        </div>
                                    </div>
                                </Motion.div>

                                {/* Images Container */}
                                <div class="max-w-4xl mx-auto pt-20 pb-24">
                                    <For each={response().data.images}>
                                        {(img, i) => (
                                            <div class="relative w-full">
                                                <CachedImage
                                                    src={img}
                                                    alt={`Page ${i() + 1}`}
                                                    class="w-full select-none"
                                                    fallbackClass="w-full aspect-[2/3] bg-zinc-900 animate-pulse flex items-center justify-center"
                                                    loading="lazy"
                                                />
                                                {/* Page Number Overlay */}
                                                <div class="absolute bottom-4 right-4 px-3 py-1 rounded-full bg-black/60 text-white/80 text-xs backdrop-blur-sm">
                                                    {i() + 1}
                                                </div>
                                            </div>
                                        )}
                                    </For>
                                </div>

                                {/* End of Chapter */}
                                <div class="max-w-4xl mx-auto px-4 pb-32">
                                    <div class="glass-subtle bg-white/5 rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-primary/20 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                            </svg>
                                        </div>
                                        <h3 class="text-xl font-bold mb-2">End of Chapter</h3>
                                        <p class="text-white/60 mb-6">You've reached the end of this chapter</p>
                                        <div class="flex justify-center gap-4 flex-wrap">
                                            <Show when={response().data.prev_chapter_id}>
                                                <A
                                                    href={`/komik/chapter/${encodeURIComponent(response().data.prev_chapter_id)}`}
                                                    class="px-6 py-3 rounded-xl bg-white/10 hover:bg-white/20 transition-colors"
                                                >
                                                    ← Previous Chapter
                                                </A>
                                            </Show>
                                            <Show when={response().data.next_chapter_id}>
                                                <A
                                                    href={`/komik/chapter/${encodeURIComponent(response().data.next_chapter_id)}`}
                                                    class="px-6 py-3 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
                                                >
                                                    Next Chapter →
                                                </A>
                                            </Show>
                                        </div>
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
