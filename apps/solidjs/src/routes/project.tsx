import { Title } from "@solidjs/meta";
import { A } from "@solidjs/router";
import { For } from "solid-js";

const projects = [
    {
        title: "Ultimate Asepharyana",
        description: "Personal website with anime streaming, manga reading, AI chat, and more.",
        tags: ["SolidStart", "TypeScript", "TailwindCSS"],
        link: "https://github.com/asepharyana",
        color: "from-blue-500 to-purple-600",
    },
    {
        title: "Elysia API",
        description: "High-performance API backend built with ElysiaJS and Bun runtime.",
        tags: ["ElysiaJS", "Bun", "TypeScript"],
        link: "https://api.asepharyana.cloud",
        color: "from-green-500 to-teal-600",
    },
    {
        title: "Rust Backend",
        description: "Blazing fast Rust backend for performance-critical operations.",
        tags: ["Rust", "Actix", "WebSocket"],
        link: "https://rust.asepharyana.cloud",
        color: "from-orange-500 to-red-600",
    },
];

export default function ProjectPage() {
    return (
        <>
            <Title>Projects | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8 lg:p-12">
                <div class="max-w-6xl mx-auto">
                    <h1 class="text-4xl font-bold mb-4 bg-gradient-to-r from-pink-600 to-rose-600 bg-clip-text text-transparent">
                        Projects
                    </h1>
                    <p class="text-muted-foreground mb-12 max-w-2xl">
                        A showcase of my personal and open-source projects. Built with passion using modern technologies.
                    </p>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <For each={projects}>
                            {(project) => (
                                <a
                                    href={project.link}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="group block p-6 rounded-xl bg-card border border-border hover:border-primary/50 hover:shadow-lg transition-all"
                                >
                                    <div class={`w-12 h-12 rounded-lg bg-gradient-to-br ${project.color} mb-4`} />
                                    <h3 class="text-xl font-semibold mb-2 group-hover:text-primary transition-colors">
                                        {project.title}
                                    </h3>
                                    <p class="text-muted-foreground text-sm mb-4">
                                        {project.description}
                                    </p>
                                    <div class="flex flex-wrap gap-2">
                                        <For each={project.tags}>
                                            {(tag) => (
                                                <span class="px-2 py-1 rounded-full bg-primary/10 text-primary text-xs">
                                                    {tag}
                                                </span>
                                            )}
                                        </For>
                                    </div>
                                </a>
                            )}
                        </For>
                    </div>

                    {/* Social Links */}
                    <div class="mt-16 text-center">
                        <h2 class="text-2xl font-bold mb-6">Connect with me</h2>
                        <div class="flex justify-center gap-4">
                            <a
                                href="https://github.com/asepharyana"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="p-4 rounded-xl bg-card border border-border hover:bg-accent transition-colors"
                            >
                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                </svg>
                            </a>
                            <a
                                href="https://linkedin.com/in/asepharyana"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="p-4 rounded-xl bg-card border border-border hover:bg-accent transition-colors"
                            >
                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </main>
        </>
    );
}
