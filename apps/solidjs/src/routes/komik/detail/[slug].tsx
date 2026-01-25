import { Title } from '@solidjs/meta';
import { A, useParams } from '@solidjs/router';
import {
  createResource,
  For,
  Show,
  Suspense,
  createSignal,
  onMount,
} from 'solid-js';
import { Motion } from 'solid-motionone';
import { httpClient } from '~/lib/http-client';
import { CachedImage } from '~/components/CachedImage';

// Matches OpenAPI DetailData schema
interface Chapter {
  chapter: string;
  date: string;
  chapter_id: string;
}

interface KomikDetail {
  title: string;
  poster: string;
  description: string;
  status: string;
  type: string;
  release_date: string;
  author: string;
  total_chapter: string;
  updated_on: string;
  genres: string[];
  chapters: Chapter[];
}

// API response wrapper
interface KomikDetailResponse {
  status: boolean;
  data: KomikDetail;
}

async function fetchKomikDetail(slug: string): Promise<KomikDetailResponse> {
  return httpClient.fetchJson<KomikDetailResponse>(
    `/api/komik/detail?komik_id=${encodeURIComponent(slug)}`,
  );
}

export default function KomikDetailPage() {
  const params = useParams();
  const [enabled, setEnabled] = createSignal(false);
  onMount(() => setEnabled(true));

  const [data] = createResource(
    () => (enabled() ? params.slug : null),
    fetchKomikDetail,
  );

  return (
    <>
      <Title>{data()?.data.title || 'Komik Detail'} | Asepharyana</Title>
      <main class="min-h-screen bg-background text-foreground">
        <Suspense
          fallback={
            <div class="p-4 md:p-8 max-w-6xl mx-auto">
              <div class="glass-card rounded-2xl p-6 md:p-8">
                <div class="flex flex-col md:flex-row gap-8">
                  <div class="w-full md:w-1/3 aspect-[3/4] rounded-xl shimmer" />
                  <div class="flex-1 space-y-4">
                    <div class="h-10 w-3/4 shimmer rounded-lg" />
                    <div class="h-6 w-1/2 shimmer rounded-lg" />
                    <div class="h-32 shimmer rounded-lg" />
                  </div>
                </div>
              </div>
            </div>
          }
        >
          <Show when={data()}>
            {(komikResponse) => {
              const komik = () => komikResponse().data;
              return (
                <Motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.5 }}
                  class="p-4 md:p-8 max-w-6xl mx-auto"
                >
                  {/* Hero Section with Backdrop */}
                  <div class="relative rounded-3xl overflow-hidden mb-8">
                    {/* Backdrop Blur Image */}
                    <div class="absolute inset-0 z-0">
                      <CachedImage
                        src={komik().poster}
                        alt=""
                        class="w-full h-full object-cover scale-110 blur-3xl opacity-30"
                      />
                      <div class="absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent" />
                    </div>

                    {/* Content */}
                    <div class="relative z-10 p-6 md:p-10">
                      <div class="flex flex-col md:flex-row gap-8">
                        {/* Poster */}
                        <div class="w-full md:w-1/3 lg:w-1/4">
                          <div class="relative group">
                            <CachedImage
                              src={komik().poster}
                              alt={komik().title}
                              class="w-full rounded-2xl shadow-2xl ring-1 ring-white/10 group-hover:ring-primary/50 transition-all duration-300"
                              fallbackClass="w-full aspect-[3/4] rounded-2xl shimmer"
                            />
                            {/* Type Badge */}
                            <div class="absolute -top-3 -right-3 px-4 py-2 rounded-full bg-primary text-primary-foreground font-bold shadow-lg glow-sm">
                              {komik().type}
                            </div>
                          </div>
                        </div>

                        {/* Info */}
                        <div class="flex-1 space-y-6">
                          <div>
                            <h1 class="text-3xl md:text-4xl font-bold mb-2 gradient-text">
                              {komik().title}
                            </h1>
                            <p class="text-muted-foreground">
                              by {komik().author}
                            </p>
                          </div>

                          {/* Stats Grid */}
                          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div class="glass-subtle rounded-xl p-4 text-center">
                              <div class="text-muted-foreground text-sm mb-1">
                                Status
                              </div>
                              <div class="font-semibold text-primary">
                                {komik().status}
                              </div>
                            </div>
                            <div class="glass-subtle rounded-xl p-4 text-center">
                              <div class="text-muted-foreground text-sm mb-1">
                                Type
                              </div>
                              <div class="font-semibold">{komik().type}</div>
                            </div>
                            <div class="glass-subtle rounded-xl p-4 text-center">
                              <div class="text-muted-foreground text-sm mb-1">
                                Chapters
                              </div>
                              <div class="font-semibold">
                                {komik().total_chapter}
                              </div>
                            </div>
                            <div class="glass-subtle rounded-xl p-4 text-center">
                              <div class="text-muted-foreground text-sm mb-1">
                                Updated
                              </div>
                              <div class="font-semibold text-sm">
                                {komik().updated_on || '-'}
                              </div>
                            </div>
                          </div>

                          {/* Genres */}
                          <div class="flex flex-wrap gap-2">
                            <For each={komik().genres}>
                              {(genre) => (
                                <span class="px-4 py-1.5 rounded-full bg-primary/10 text-primary text-sm font-medium hover:bg-primary/20 transition-colors cursor-default">
                                  {genre}
                                </span>
                              )}
                            </For>
                          </div>

                          {/* Description */}
                          <div class="glass-subtle rounded-xl p-5">
                            <h3 class="font-semibold text-lg mb-3 flex items-center gap-2">
                              <svg
                                class="w-5 h-5 text-primary"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                              >
                                <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                />
                              </svg>
                              Description
                            </h3>
                            <p class="text-muted-foreground leading-relaxed">
                              {komik().description}
                            </p>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>

                  {/* Chapters Section */}
                  <div class="glass-card rounded-2xl p-6 md:p-8">
                    <div class="flex items-center justify-between mb-6">
                      <h2 class="text-2xl font-bold flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center">
                          <svg
                            class="w-5 h-5 text-primary"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M4 6h16M4 10h16M4 14h16M4 18h16"
                            />
                          </svg>
                        </div>
                        Chapters
                      </h2>
                      <span class="text-muted-foreground text-sm">
                        {komik().chapters?.length || 0} chapters
                      </span>
                    </div>

                    <div class="space-y-2 max-h-[500px] overflow-y-auto pr-2 scrollbar-thin">
                      <For each={komik().chapters}>
                        {(ch, i) => (
                          <A
                            href={`/komik/chapter/${encodeURIComponent(ch.chapter_id)}`}
                            class="flex justify-between items-center p-4 rounded-xl bg-secondary/50 border border-border/50 hover:border-primary/50 hover:bg-primary/5 transition-all duration-200 group hover-lift"
                          >
                            <div class="flex items-center gap-4">
                              <span class="w-10 h-10 rounded-lg bg-primary/10 text-primary font-bold flex items-center justify-center text-sm group-hover:bg-primary group-hover:text-primary-foreground transition-colors">
                                {i() + 1}
                              </span>
                              <span class="font-medium group-hover:text-primary transition-colors">
                                {ch.chapter}
                              </span>
                            </div>
                            <div class="flex items-center gap-3">
                              <span class="text-sm text-muted-foreground">
                                {ch.date}
                              </span>
                              <svg
                                class="w-5 h-5 text-muted-foreground group-hover:text-primary group-hover:translate-x-1 transition-all"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                              >
                                <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M9 5l7 7-7 7"
                                />
                              </svg>
                            </div>
                          </A>
                        )}
                      </For>
                    </div>
                  </div>
                </Motion.div>
              );
            }}
          </Show>
        </Suspense>
      </main>
    </>
  );
}
