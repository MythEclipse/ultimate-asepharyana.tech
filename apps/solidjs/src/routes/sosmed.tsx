import { Title } from "@solidjs/meta";

export default function SosmedPage() {
    return (
        <>
            <Title>Social Media | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8 lg:p-12">
                <div class="max-w-4xl mx-auto text-center">
                    <h1 class="text-4xl font-bold mb-8 bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
                        Social Media Tools
                    </h1>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="p-8 rounded-xl bg-gradient-to-br from-pink-500 to-rose-600 text-white">
                            <span class="text-5xl mb-4 block">📷</span>
                            <h3 class="text-xl font-bold mb-2">Instagram Downloader</h3>
                            <p class="text-white/80 mb-4">Download photos and videos from Instagram</p>
                            <span class="text-sm bg-white/20 px-3 py-1 rounded-full">Coming Soon</span>
                        </div>

                        <div class="p-8 rounded-xl bg-gradient-to-br from-black to-gray-800 text-white">
                            <span class="text-5xl mb-4 block">🎵</span>
                            <h3 class="text-xl font-bold mb-2">TikTok Downloader</h3>
                            <p class="text-white/80 mb-4">Download TikTok videos without watermark</p>
                            <span class="text-sm bg-white/20 px-3 py-1 rounded-full">Coming Soon</span>
                        </div>

                        <div class="p-8 rounded-xl bg-gradient-to-br from-red-500 to-red-700 text-white">
                            <span class="text-5xl mb-4 block">▶️</span>
                            <h3 class="text-xl font-bold mb-2">YouTube Downloader</h3>
                            <p class="text-white/80 mb-4">Download YouTube videos and audio</p>
                            <span class="text-sm bg-white/20 px-3 py-1 rounded-full">Coming Soon</span>
                        </div>

                        <div class="p-8 rounded-xl bg-gradient-to-br from-blue-500 to-blue-700 text-white">
                            <span class="text-5xl mb-4 block">🐦</span>
                            <h3 class="text-xl font-bold mb-2">Twitter Downloader</h3>
                            <p class="text-white/80 mb-4">Download Twitter/X videos and images</p>
                            <span class="text-sm bg-white/20 px-3 py-1 rounded-full">Coming Soon</span>
                        </div>
                    </div>
                </div>
            </main>
        </>
    );
}
