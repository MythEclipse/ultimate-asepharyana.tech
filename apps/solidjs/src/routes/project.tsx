import { Title } from "@solidjs/meta";
import { For, createSignal, onMount } from "solid-js";
import { Motion } from "solid-motionone";
import { A } from "@solidjs/router";
import { useTheme } from "~/components/providers/theme-provider";

interface Project {
    title: string;
    description: string;
    images: {
        light: string;
        dark: string;
    };
    linkUrl: string;
}

const PROJECTS: Project[] = [
    {
        title: "Elysia API",
        description: "Backend API dengan ElysiaJS - Dokumentasi Swagger",
        images: { light: "/project-elysia.png", dark: "/project-elysia.png" },
        linkUrl: "https://elysia.asepharyana.tech/docs",
    },
    {
        title: "Rust API",
        description: "High-performance API dengan Rust - Dokumentasi Swagger",
        images: { light: "/project-rust.png", dark: "/project-rust.png" },
        linkUrl: "https://ws.asepharyana.tech/docs",
    },
    {
        title: "Anime",
        description: "Anime scraping dari otakudesu.cloud",
        images: { light: "/project-anime.png", dark: "/project-anime.png" },
        linkUrl: "/anime",
    },
    {
        title: "Anime2",
        description: "Anime scraping dari alqanime.net",
        images: { light: "/project-anime2.png", dark: "/project-anime2.png" },
        linkUrl: "/anime2",
    },
    {
        title: "Komik",
        description: "Komik scraping dari komikindo1.com",
        images: { light: "/project-komik.png", dark: "/project-komik.png" },
        linkUrl: "/komik",
    },
    {
        title: "Sosmed",
        description: "Autentikasi & CRUD dasar",
        images: { light: "/project-sosmed.png", dark: "/project-sosmed.png" },
        linkUrl: "/sosmed",
    },
    {
        title: "Chat",
        description: "Chat realtime dengan WebSocket",
        images: { light: "/project-chat.png", dark: "/project-chat.png" },
        linkUrl: "/chat",
    },
    {
        title: "Compressor",
        description: "Kompressor gambar dan video",
        images: { light: "/project-compressor.png", dark: "/project-compressor.png" },
        linkUrl: "/compressor",
    },
];

function ProjectCard(props: { project: Project; imageSrc: string }) {
    const isExternal = () => props.project.linkUrl.startsWith('http');

    const CardContent = () => (
        <article class="relative bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-2xl shadow-xl overflow-hidden border border-gray-200 dark:border-gray-700 hover:shadow-2xl hover:border-blue-400 dark:hover:border-blue-500 transition-all duration-500 transform group-hover:scale-[1.03] group-hover:-translate-y-1">
            {/* Image Container with Overlay */}
            <div class="relative h-56 overflow-hidden bg-gradient-to-br from-blue-500 to-purple-600">
                <img
                    src={props.imageSrc}
                    alt={props.project.title}
                    loading="lazy"
                    class="w-full h-full object-cover transition-all duration-700 group-hover:scale-110 group-hover:brightness-110"
                />
                {/* Gradient Overlay */}
                <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-black/20 to-transparent opacity-60 group-hover:opacity-40 transition-opacity duration-300" />
                {/* External badge */}
                {isExternal() && (
                    <div class="absolute top-3 right-3 px-2 py-1 bg-blue-500 text-white text-xs font-bold rounded-full flex items-center gap-1">
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                        </svg>
                        API
                    </div>
                )}
            </div>

            {/* Content */}
            <div class="p-6 space-y-3">
                <h3 class="text-2xl font-bold text-gray-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors duration-300 flex items-center gap-2">
                    {props.project.title}
                    <svg
                        class="w-5 h-5 transform transition-transform duration-300 group-hover:translate-x-1"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M13 7l5 5m0 0l-5 5m5-5H6"
                        />
                    </svg>
                </h3>
                <p class="text-gray-600 dark:text-gray-400 text-sm leading-relaxed">
                    {props.project.description}
                </p>
            </div>

            {/* Bottom Accent Line */}
            <div class="h-1 w-0 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 group-hover:w-full transition-all duration-500 ease-out" />
        </article>
    );

    return isExternal() ? (
        <a href={props.project.linkUrl} target="_blank" rel="noopener noreferrer" class="block group">
            <CardContent />
        </a>
    ) : (
        <A href={props.project.linkUrl} class="block group">
            <CardContent />
        </A>
    );
}

function CardSkeleton() {
    return (
        <div class="bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800 rounded-2xl shadow-xl overflow-hidden border border-gray-200 dark:border-gray-700 animate-pulse">
            <div class="h-56 bg-gradient-to-br from-gray-300 to-gray-400 dark:from-gray-600 dark:to-gray-700" />
            <div class="p-6 space-y-4">
                <div class="h-7 bg-gray-300 dark:bg-gray-600 rounded-lg w-2/3" />
                <div class="space-y-2">
                    <div class="h-4 bg-gray-300 dark:bg-gray-600 rounded w-full" />
                    <div class="h-4 bg-gray-300 dark:bg-gray-600 rounded w-4/5" />
                </div>
            </div>
        </div>
    );
}

export default function ProjectPage() {
    const [mounted, setMounted] = createSignal(false);
    const { resolvedTheme } = useTheme();

    onMount(() => {
        setMounted(true);
    });

    const isLightTheme = () => resolvedTheme() === "light";

    return (
        <>
            <Title>Project Terbaru | Asepharyana</Title>
            <main class="min-h-screen p-6 bg-background text-foreground">
                <div class="max-w-7xl mx-auto space-y-8">
                    {/* Header */}
                    <Motion.div
                        initial={{ opacity: 0, y: -20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5 }}
                        class="flex items-center gap-4 mb-8"
                    >
                        <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center shadow-lg">
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                            </svg>
                        </div>
                        <div>
                            <h1 class="text-3xl md:text-4xl font-bold text-foreground">Project Terbaru</h1>
                            <p class="text-muted-foreground">Berikut adalah kumpulan project yang saya buat</p>
                        </div>
                    </Motion.div>

                    {/* Project Grid */}
                    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
                        {mounted() ? (
                            <For each={PROJECTS}>
                                {(project, index) => (
                                    <Motion.div
                                        initial={{ opacity: 0, y: 30 }}
                                        animate={{ opacity: 1, y: 0 }}
                                        transition={{ duration: 0.5, delay: 0.1 * index() }}
                                    >
                                        <ProjectCard
                                            project={project}
                                            imageSrc={isLightTheme() ? project.images.light : project.images.dark}
                                        />
                                    </Motion.div>
                                )}
                            </For>
                        ) : (
                            <For each={Array.from({ length: 6 })}>
                                {() => <CardSkeleton />}
                            </For>
                        )}
                    </div>
                </div>
            </main>
        </>
    );
}
