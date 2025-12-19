import { Title } from "@solidjs/meta";

export default function CompressorPage() {
    return (
        <>
            <Title>Image Compressor | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8 lg:p-12">
                <div class="max-w-4xl mx-auto text-center">
                    <h1 class="text-4xl font-bold mb-4 bg-gradient-to-r from-cyan-600 to-blue-600 bg-clip-text text-transparent">
                        🖼️ Image Compressor
                    </h1>
                    <p class="text-muted-foreground mb-12">
                        Compress your images to reduce file size while maintaining quality.
                    </p>

                    <div class="p-12 rounded-xl border-2 border-dashed border-border bg-card hover:border-primary/50 transition-colors cursor-pointer">
                        <svg class="w-16 h-16 mx-auto mb-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                        </svg>
                        <p class="text-lg font-medium mb-2">Drop your images here</p>
                        <p class="text-sm text-muted-foreground">or click to browse</p>
                        <p class="text-xs text-muted-foreground mt-4">Supports: JPG, PNG, WebP, GIF</p>
                    </div>

                    <div class="mt-8 text-sm text-muted-foreground">
                        <p>✅ Free to use • ✅ No signup required • ✅ Files are not stored</p>
                    </div>
                </div>
            </main>
        </>
    );
}
