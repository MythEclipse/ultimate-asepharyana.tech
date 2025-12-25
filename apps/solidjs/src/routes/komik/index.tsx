import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { createSignal, createResource, For, Show, Suspense } from "solid-js";
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

async function fetchManga(): Promise<KomikListResponse> {
    return httpClient.fetchJson<KomikListResponse>("/api/komik/manga?page=1");
}

async function fetchManhwa(): Promise<KomikListResponse> {
    return httpClient.fetchJson<KomikListResponse>("/api/komik/manhwa?page=1");
}

async function fetchManhua(): Promise<KomikListResponse> {
    return httpClient.fetchJson<KomikListResponse>("/api/komik/manhua?page=1");
}

function KomikCard(props: { item: KomikItem; index: number }) {
    return (
        <Motion.div
            initial={{ opacity: 0, y: 60, scale: 0.8 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            transition={{
                duration: 0.6,
                delay: props.index * 0.08,
                easing: [0.25, 0.46, 0.45, 0.94]
            }}
            class="group perspective-1000"
        >
            <A
                href={`/komik/detail/${props.item.slug}`}
                class="block relative overflow-hidden rounded-2xl bg-card border border-border shadow-lg hover:shadow-2xl hover:shadow-primary/20 transition-all duration-500 transform-gpu hover:-translate-y-3 hover:scale-[1.02]"
            >
                {/* Glow effect on hover */}
                <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-t from-primary/20 via-transparent to-transparent blur-xl" />

                {/* Animated border glow */}
                <div class="absolute -inset-[1px] rounded-2xl bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10 blur-sm" />

                <div class="aspect-[3/4] overflow-hidden relative">
                    <CachedImage
                        src={props.item.poster}
                        alt={props.item.title}
                        class="w-full h-full object-cover transform-gpu group-hover:scale-110 transition-transform duration-700 ease-out"
                        fallbackClass="w-full h-full bg-muted animate-pulse"
                        loading="lazy"
                    />

                    {/* Shine effect on hover */}
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-1000 ease-out" />
                </div>

                {/* Gradient overlay */}
                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/40 to-transparent opacity-80 group-hover:opacity-90 transition-opacity duration-300" />

                {/* Type Badge */}
                <div class="absolute top-3 right-3">
                    <span class={`px-3 py-1.5 rounded-lg text-xs font-bold shadow-lg backdrop-blur-sm ${props.item.type === 'Manga' ? 'bg-orange-500/90 text-white' :
                            props.item.type === 'Manhwa' ? 'bg-blue-500/90 text-white' :
                                props.item.type === 'Manhua' ? 'bg-red-500/90 text-white' :
                                    'bg-primary/90 text-primary-foreground'
                        }`}>
                        {props.item.type}
                    </span>
                </div>

                {/* Score Badge */}
                <Show when={props.item.score}>
                    <div class="absolute top-3 left-3">
                        <span class="px-2 py-1 rounded-lg text-xs font-bold bg-yellow-500/90 text-black shadow-lg backdrop-blur-sm flex items-center gap-1">
                            ⭐ {props.item.score}
                        </span>
                    </div>
                </Show>

                {/* Content */}
                <div class="absolute bottom-0 left-0 right-0 p-4">
                    <h3 class="text-white text-sm font-bold line-clamp-2 drop-shadow-lg mb-2 group-hover:text-orange-200 transition-colors duration-300">
                        {props.item.title}
                    </h3>
                    <Show when={props.item.chapter}>
                        <span class="inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full bg-primary/80 text-white backdrop-blur-sm">
                            <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse" />
                            {props.item.chapter}
                        </span>
                    </Show>
                </div>

                {/* Read button overlay */}
                <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300">
                    <div class="w-16 h-16 rounded-full bg-white/20 backdrop-blur-md flex items-center justify-center transform scale-50 group-hover:scale-100 transition-transform duration-500">
                        <svg class="w-8 h-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                        </svg>
                    </div>
                </div>
            </A>
        </Motion.div>
    );
}

function KomikGrid(props: { items: KomikItem[]; loading?: boolean }) {
    return (
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-5">
            <Show when={props.loading}>
                <For each={Array(6).fill(0)}>
                    {(_, i) => (
                        <Motion.div
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            transition={{ delay: i() * 0.05 }}
                            class="aspect-[3/4] rounded-2xl shimmer"
                        />
                    )}
                </For>
            </Show>
            <Show when={!props.loading}>
                <For each={props.items}>
                    {(item, i) => <KomikCard item={item} index={i()} />}
                </For>
            </Show>
        </div>
    );
}

function SectionHeader(props: { title: string; gradient: string; emoji: string; href: string }) {
    return (
        <Motion.div
            initial={{ opacity: 0, x: -30 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.5 }}
            class="flex items-center justify-between mb-6"
        >
            <div class="flex items-center gap-3">
                <div class={`w-12 h-12 rounded-xl bg-gradient-to-br ${props.gradient} flex items-center justify-center text-2xl shadow-lg`}>
                    {props.emoji}
                </div>
                <h2 class={`text-2xl font-bold bg-gradient-to-r ${props.gradient} bg-clip-text text-transparent`}>
                    {props.title}
                </h2>
            </div>
            <A
                href={props.href}
                class="group flex items-center gap-2 px-4 py-2 rounded-xl bg-primary/10 text-primary hover:bg-primary hover:text-primary-foreground transition-all duration-300"
            >
                View All
                <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
            </A>
        </Motion.div>
    );
}

export default function KomikPage() {
    const [manga] = createResource(fetchManga);
    const [manhwa] = createResource(fetchManhwa);
    const [manhua] = createResource(fetchManhua);
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
                                <Show when={manga.error}>
                                    <div class="glass-card rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </div>
                                        <p class="text-destructive">Failed to load manga. Please try again later.</p>
                                    </div>
                                </Show>
                                <Show when={manga()}>
                                    {(data) => <KomikGrid items={data().data.slice(0, 6)} />}
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
                                <Show when={manhwa.error}>
                                    <div class="glass-card rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </div>
                                        <p class="text-destructive">Failed to load manhwa. Please try again later.</p>
                                    </div>
                                </Show>
                                <Show when={manhwa()}>
                                    {(data) => <KomikGrid items={data().data.slice(0, 6)} />}
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
                                <Show when={manhua.error}>
                                    <div class="glass-card rounded-2xl p-8 text-center">
                                        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-destructive/10 flex items-center justify-center">
                                            <svg class="w-8 h-8 text-destructive" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                        </div>
                                        <p class="text-destructive">Failed to load manhua. Please try again later.</p>
                                    </div>
                                </Show>
                                <Show when={manhua()}>
                                    {(data) => <KomikGrid items={data().data.slice(0, 6)} />}
                                </Show>
                            </Suspense>
                        </section>
                    </div>
                </div>
            </main>
        </>
    );
}
