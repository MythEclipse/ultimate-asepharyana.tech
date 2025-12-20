import { Title } from "@solidjs/meta";
import { For } from "solid-js";
import { Motion } from "solid-motionone";

const tools = [
    {
        icon: "📷",
        title: "Instagram Downloader",
        description: "Download photos and videos from Instagram",
        gradient: "from-pink-500 via-rose-500 to-orange-500",
        shadow: "shadow-pink-500/30"
    },
    {
        icon: "🎵",
        title: "TikTok Downloader",
        description: "Download TikTok videos without watermark",
        gradient: "from-gray-900 via-gray-800 to-gray-700",
        shadow: "shadow-gray-500/30"
    },
    {
        icon: "▶️",
        title: "YouTube Downloader",
        description: "Download YouTube videos and audio",
        gradient: "from-red-500 via-red-600 to-red-700",
        shadow: "shadow-red-500/30"
    },
    {
        icon: "🐦",
        title: "Twitter Downloader",
        description: "Download Twitter/X videos and images",
        gradient: "from-blue-400 via-blue-500 to-blue-600",
        shadow: "shadow-blue-500/30"
    }
];

export default function SosmedPage() {
    return (
        <>
            <Title>Social Media | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8 lg:p-12 relative overflow-hidden">
                {/* Animated background orbs */}
                <div class="fixed inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute -top-40 -right-40 w-96 h-96 bg-gradient-to-br from-pink-500/20 to-purple-500/20 rounded-full blur-3xl animate-float" />
                    <div class="absolute -bottom-40 -left-40 w-96 h-96 bg-gradient-to-tr from-blue-500/20 to-cyan-500/20 rounded-full blur-3xl animate-float" style={{ "animation-delay": "-3s" }} />
                </div>

                <div class="max-w-4xl mx-auto text-center relative z-10">
                    <Motion.div
                        initial={{ opacity: 0, y: -20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5 }}
                    >
                        <span class="inline-block px-4 py-1.5 rounded-full text-sm font-medium bg-gradient-to-r from-purple-500/20 to-pink-500/20 text-primary border border-primary/20 mb-4">
                            🔧 Tools
                        </span>
                        <h1 class="text-4xl md:text-5xl font-bold mb-4 gradient-text">
                            Social Media Tools
                        </h1>
                        <p class="text-muted-foreground max-w-xl mx-auto mb-12">
                            Download content from your favorite social media platforms. Free and easy to use.
                        </p>
                    </Motion.div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <For each={tools}>
                            {(tool, index) => (
                                <Motion.div
                                    initial={{ opacity: 0, y: 30, scale: 0.95 }}
                                    animate={{ opacity: 1, y: 0, scale: 1 }}
                                    transition={{ duration: 0.5, delay: 0.1 * index() }}
                                    class="group"
                                >
                                    <div class={`relative p-8 rounded-2xl bg-gradient-to-br ${tool.gradient} text-white overflow-hidden shadow-xl ${tool.shadow} hover:scale-[1.03] transition-all duration-300`}>
                                        {/* Animated shine effect */}
                                        <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 overflow-hidden">
                                            <div class="absolute inset-0 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-1000 bg-gradient-to-r from-transparent via-white/20 to-transparent" />
                                        </div>

                                        {/* Decorative circles */}
                                        <div class="absolute -top-12 -right-12 w-32 h-32 rounded-full bg-white/10 blur-2xl" />
                                        <div class="absolute -bottom-12 -left-12 w-32 h-32 rounded-full bg-white/10 blur-2xl" />

                                        <div class="relative">
                                            <span class="text-5xl mb-4 block group-hover:scale-110 transition-transform duration-300">
                                                {tool.icon}
                                            </span>
                                            <h3 class="text-xl font-bold mb-2">{tool.title}</h3>
                                            <p class="text-white/80 mb-6">{tool.description}</p>

                                            {/* Coming Soon Badge */}
                                            <span class="inline-flex items-center gap-2 text-sm bg-white/20 backdrop-blur-sm px-4 py-2 rounded-full border border-white/30">
                                                <span class="w-2 h-2 bg-yellow-400 rounded-full animate-pulse" />
                                                Coming Soon
                                            </span>
                                        </div>
                                    </div>
                                </Motion.div>
                            )}
                        </For>
                    </div>

                    {/* Newsletter/Notify section */}
                    <Motion.div
                        initial={{ opacity: 0, y: 30 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5, delay: 0.5 }}
                        class="mt-16"
                    >
                        <div class="glass-card rounded-2xl p-8 max-w-md mx-auto">
                            <h3 class="text-xl font-bold mb-2">Get notified</h3>
                            <p class="text-muted-foreground text-sm mb-6">
                                Be the first to know when these tools are available.
                            </p>
                            <div class="flex gap-3">
                                <input
                                    type="email"
                                    placeholder="Enter your email"
                                    class="flex-1 px-4 py-3 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all placeholder:text-muted-foreground/50"
                                />
                                <button class="px-6 py-3 rounded-xl bg-gradient-to-r from-primary to-purple-600 text-white font-medium hover:opacity-90 transition-opacity shadow-lg shadow-primary/30">
                                    Notify Me
                                </button>
                            </div>
                        </div>
                    </Motion.div>
                </div>
            </main>
        </>
    );
}
