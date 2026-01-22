import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { createSignal, createResource, For, Show, Suspense, onMount } from "solid-js";
import { Motion } from "solid-motionone";
import { httpClient } from "~/lib/http-client";
import { CachedImage } from "~/components/CachedImage";

interface KomikItem {
    title: string;
    slug: string;
    poster: string;
    type: string;
    chapter?: string;
    score?: string;
    date?: string;
    reader_count?: string;
}

interface KomikListResponse {
    data: KomikItem[];
    pagination: {
        current_page: number;
        last_visible_page: number;
        has_next_page: boolean;
        next_page: number | null;
        has_previous_page: boolean;
        previous_page: number | null;
    };
}


interface HomeData {
    manga: KomikListResponse;
    manhwa: KomikListResponse;
    manhua: KomikListResponse;
}

// Unified fetcher to match anime's single-resource pattern
async function fetchKomikData(): Promise<HomeData> {
    const [manga, manhwa, manhua] = await Promise.all([
        httpClient.fetchJson<KomikListResponse>("/api/komik/manga?page=1"),
        httpClient.fetchJson<KomikListResponse>("/api/komik/manhwa?page=1"),
        httpClient.fetchJson<KomikListResponse>("/api/komik/manhua?page=1")
    ]);

    return { manga, manhwa, manhua };
}

// ... unchanged components ...

export default function KomikPage() {
    const [enabled, setEnabled] = createSignal(false);
    onMount(() => setEnabled(true));

    const [data] = createResource(enabled, fetchKomikData);
    const [searchQuery, setSearchQuery] = createSignal("");


    return (
        <>
            <Title>Komik | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground">
                {/* Hero Section */}
                <div class="relative overflow-hidden">
                    {/* Background Orbs */}
                    <div class="absolute inset-0 overflow-hidden pointer-events-none">
                        <div class="absolute -top-40 -right-40 w-80 h-80 rounded-full bg-gradient-to-br from-orange-500/20 to-red-500/20 blur-3xl" />
                        <div class="absolute top-1/2 -left-40 w-60 h-60 rounded-full bg-gradient-to-br from-blue-500/20 to-purple-500/20 blur-3xl" />
                    </div>

                    <div class="relative z-10 p-4 md:p-8 lg:p-12 max-w-7xl mx-auto">
                        {/* Header */}
                        <Motion.div
                            initial={{ opacity: 0, y: -30 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.6 }}
                            class="text-center mb-10"
                        >
                            <h1 class="text-5xl md:text-6xl font-bold mb-4">
                                <span class="gradient-text">Komik</span>
                            </h1>
                            <p class="text-muted-foreground text-lg">
                                Read your favorite manga, manhwa, and manhua
                            </p>
                        </Motion.div>

                        {/* Search Bar */}
                        <Motion.form
                            initial={{ opacity: 0, y: 20 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.5, delay: 0.2 }}
                            action="/komik/search"
                            method="get"
                            class="mb-12"
                        >
                            <div class="max-w-2xl mx-auto relative">
                                <div class="absolute inset-0 bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 rounded-2xl blur-sm opacity-50" />
                                <div class="relative flex gap-3 p-2 rounded-2xl glass-card">
                                    <input
                                        type="text"
                                        name="q"
                                        value={searchQuery()}
                                        onInput={(e) => setSearchQuery(e.currentTarget.value)}
                                        placeholder="Search manga, manhwa, manhua..."
                                        class="flex-1 px-5 py-3 rounded-xl bg-background/50 border-0 focus:outline-none focus:ring-2 focus:ring-primary placeholder:text-muted-foreground"
                                    />
                                    <button
                                        type="submit"
                                        class="px-6 py-3 rounded-xl bg-gradient-to-r from-orange-500 to-red-500 text-white font-semibold hover:opacity-90 transition-opacity flex items-center gap-2"
                                    >
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                        </svg>
                                        Search
                                    </button>
                                </div>
                            </div>
                        </Motion.form>

                        {/* Category Tabs */}
                        <Motion.div
                            initial={{ opacity: 0, y: 20 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.5, delay: 0.3 }}
                            class="flex justify-center gap-3 mb-12 flex-wrap"
                        >
                            <A href="/komik/manga/page/1" class="px-6 py-3 rounded-xl glass-subtle hover:bg-orange-500/20 hover:text-orange-500 transition-all duration-300 font-medium">
                                📚 All Manga
                            </A>
                            <A href="/komik/manhwa/page/1" class="px-6 py-3 rounded-xl glass-subtle hover:bg-blue-500/20 hover:text-blue-500 transition-all duration-300 font-medium">
                                🇰🇷 All Manhwa
                            </A>
                            <A href="/komik/manhua/page/1" class="px-6 py-3 rounded-xl glass-subtle hover:bg-red-500/20 hover:text-red-500 transition-all duration-300 font-medium">
                                🇨🇳 All Manhua
                            </A>
                        </Motion.div>


                        {/* Manga Section */}
                        <section class="mb-16">
                            <SectionHeader
                                title="Manga"
                                gradient="from-orange-500 to-red-500"
                                emoji="📚"
                                href="/komik/manga/page/1"
                            />
                            <Suspense fallback={<KomikGrid items={[]} loading={true} />}>
                                <Show when={data.error}>
                                    <div class="glass-card rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </div>
                                        <p class="text-destructive">Failed to load manga. Please try again later.</p>
                                    </div>
                                </Show>
                                <Show when={data()?.manga}>
                                    {(mangaData) => <KomikGrid items={mangaData.data.slice(0, 6)} />}
                                </Show>
                            </Suspense>
                        </section>

                        {/* Manhwa Section */}
                        <section class="mb-16">
                            <SectionHeader
                                title="Manhwa"
                                gradient="from-blue-500 to-purple-500"
                                emoji="🇰🇷"
                                href="/komik/manhwa/page/1"
                            />
                            <Suspense fallback={<KomikGrid items={[]} loading={true} />}>
                                <Show when={data.error}>
                                    <div class="glass-card rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </div>
                                        <p class="text-destructive">Failed to load manhwa. Please try again later.</p>
                                    </div>
                                </Show>
                                <Show when={data()?.manhwa}>
                                    {(manhwaData) => <KomikGrid items={manhwaData.data.slice(0, 6)} />}
                                </Show>
                            </Suspense>
                        </section>

                        {/* Manhua Section */}
                        <section class="mb-16">
                            <SectionHeader
                                title="Manhua"
                                gradient="from-red-500 to-yellow-500"
                                emoji="🇨🇳"
                                href="/komik/manhua/page/1"
                            />
                            <Suspense fallback={<KomikGrid items={[]} loading={true} />}>
                                <Show when={data.error}>
                                    <div class="glass-card rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </div>
                                        <p class="text-destructive">Failed to load manhua. Please try again later.</p>
                                    </div>
                                </Show>
                                <Show when={data()?.manhua}>
                                    {(manhuaData) => <KomikGrid items={manhuaData.data.slice(0, 6)} />}
                                </Show>
                            </Suspense>
                        </section>

                    </div>
                </div>
            </main>
        </>
    );
}
