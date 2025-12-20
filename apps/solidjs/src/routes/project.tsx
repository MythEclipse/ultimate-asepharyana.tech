import { Title } from "@solidjs/meta";
import { For } from "solid-js";
import { Motion } from "solid-motionone";

const projects = [
    {
        title: "Ultimate Asepharyana",
        description: "Personal website with anime streaming, manga reading, AI chat, and more.",
        tags: ["SolidStart", "TypeScript", "TailwindCSS"],
        link: "https://github.com/asepharyana",
        gradient: "from-blue-500 via-purple-500 to-pink-500",
        icon: "🌐"
    },
    {
        title: "Elysia API",
        description: "High-performance API backend built with ElysiaJS and Bun runtime.",
        tags: ["ElysiaJS", "Bun", "TypeScript"],
        link: "https://api.asepharyana.cloud",
        gradient: "from-green-500 via-emerald-500 to-teal-500",
        icon: "⚡"
    },
    {
        title: "Rust Backend",
        description: "Blazing fast Rust backend for performance-critical operations.",
        tags: ["Rust", "Actix", "WebSocket"],
        link: "https://rust.asepharyana.cloud",
        gradient: "from-orange-500 via-red-500 to-rose-500",
        icon: "🦀"
    },
];

const socialLinks = [
    {
        href: "https://github.com/asepharyana",
        label: "GitHub",
        icon: (
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
        ),
        gradient: "from-gray-700 to-gray-900"
    },
    {
        href: "https://linkedin.com/in/asepharyana",
        label: "LinkedIn",
        icon: (
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
            </svg>
        ),
        gradient: "from-blue-600 to-blue-800"
    },
    {
        href: "https://twitter.com/asepharyana",
        label: "Twitter",
        icon: (
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
            </svg>
        ),
        gradient: "from-gray-800 to-black"
    }
];

export default function ProjectPage() {
    return (
        <>
            <Title>Projects | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8 lg:p-12 relative overflow-hidden">
                {/* Animated background orbs */}
                <div class="fixed inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute -top-40 -left-40 w-96 h-96 bg-gradient-to-br from-pink-500/20 to-purple-500/20 rounded-full blur-3xl animate-float" />
                    <div class="absolute -bottom-40 -right-40 w-96 h-96 bg-gradient-to-tr from-blue-500/20 to-cyan-500/20 rounded-full blur-3xl animate-float" style={{ "animation-delay": "-4s" }} />
                </div>

                <div class="max-w-6xl mx-auto relative z-10">
                    <Motion.div
                        initial={{ opacity: 0, y: -20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5 }}
                    >
                        <span class="inline-block px-4 py-1.5 rounded-full text-sm font-medium bg-gradient-to-r from-pink-500/20 to-rose-500/20 text-primary border border-primary/20 mb-4">
                            ✨ Portfolio
                        </span>
                        <h1 class="text-4xl md:text-5xl font-bold mb-4 gradient-text">
                            Projects
                        </h1>
                        <p class="text-muted-foreground mb-12 max-w-2xl">
                            A showcase of my personal and open-source projects. Built with passion using modern technologies.
                        </p>
                    </Motion.div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-16">
                        <For each={projects}>
                            {(project, index) => (
                                <Motion.div
                                    initial={{ opacity: 0, y: 30, rotateX: 10 }}
                                    animate={{ opacity: 1, y: 0, rotateX: 0 }}
                                    transition={{ duration: 0.5, delay: 0.1 * index() }}
                                >
                                    <a
                                        href={project.link}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="group block glass-card rounded-2xl p-6 hover:scale-[1.03] transition-all duration-300"
                                        style={{ "transform-style": "preserve-3d" }}
                                    >
                                        {/* Gradient glow on hover */}
                                        <div class={`absolute inset-0 rounded-2xl bg-gradient-to-br ${project.gradient} opacity-0 group-hover:opacity-10 blur-xl transition-opacity duration-500`} />

                                        <div class="relative">
                                            {/* Icon with gradient background */}
                                            <div class={`w-14 h-14 rounded-xl bg-gradient-to-br ${project.gradient} flex items-center justify-center mb-4 shadow-lg group-hover:scale-110 transition-transform duration-300`}>
                                                <span class="text-2xl">{project.icon}</span>
                                            </div>

                                            <h3 class="text-xl font-semibold mb-2 group-hover:text-primary transition-colors flex items-center gap-2">
                                                {project.title}
                                                <svg class="w-4 h-4 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                                                </svg>
                                            </h3>
                                            <p class="text-muted-foreground text-sm mb-4 line-clamp-2">
                                                {project.description}
                                            </p>

                                            {/* Tags */}
                                            <div class="flex flex-wrap gap-2">
                                                <For each={project.tags}>
                                                    {(tag) => (
                                                        <span class="px-3 py-1 rounded-full bg-gradient-to-r from-primary/10 to-purple-500/10 text-primary/80 text-xs font-medium border border-primary/20">
                                                            {tag}
                                                        </span>
                                                    )}
                                                </For>
                                            </div>
                                        </div>
                                    </a>
                                </Motion.div>
                            )}
                        </For>
                    </div>

                    {/* Social Links */}
                    <Motion.div
                        initial={{ opacity: 0, y: 30 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5, delay: 0.4 }}
                        class="text-center"
                    >
                        <h2 class="text-2xl font-bold mb-2">Connect with me</h2>
                        <p class="text-muted-foreground mb-8">Let's build something amazing together</p>

                        <div class="flex justify-center gap-4">
                            <For each={socialLinks}>
                                {(link, index) => (
                                    <Motion.a
                                        initial={{ opacity: 0, scale: 0.8 }}
                                        animate={{ opacity: 1, scale: 1 }}
                                        transition={{ duration: 0.3, delay: 0.5 + 0.1 * index() }}
                                        href={link.href}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class={`group p-4 rounded-2xl bg-gradient-to-br ${link.gradient} text-white hover:scale-110 transition-all duration-300 shadow-lg hover:shadow-xl`}
                                        title={link.label}
                                    >
                                        <span class="group-hover:scale-110 transition-transform duration-300 block">
                                            {link.icon}
                                        </span>
                                    </Motion.a>
                                )}
                            </For>
                        </div>
                    </Motion.div>
                </div>
            </main>
        </>
    );
}
