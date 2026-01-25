import { Title } from '@solidjs/meta';
import { createSignal, Show, For } from 'solid-js';
import { Motion } from 'solid-motionone';
import toast from 'solid-toast';

interface CompressedImage {
  original: File;
  compressed: Blob;
  originalSize: number;
  compressedSize: number;
  preview: string;
}

export default function CompressorPage() {
  const [isDragging, setIsDragging] = createSignal(false);
  const [quality, setQuality] = createSignal(80);
  const [images, setImages] = createSignal<CompressedImage[]>([]);
  const [isProcessing, setIsProcessing] = createSignal(false);

  const formatSize = (bytes: number): string => {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
  };

  const compressImage = async (
    file: File,
    quality: number,
  ): Promise<CompressedImage> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        const img = new Image();
        img.onload = () => {
          const canvas = document.createElement('canvas');
          canvas.width = img.width;
          canvas.height = img.height;

          const ctx = canvas.getContext('2d');
          if (!ctx) {
            reject(new Error('Failed to get canvas context'));
            return;
          }

          ctx.drawImage(img, 0, 0);

          canvas.toBlob(
            (blob) => {
              if (!blob) {
                reject(new Error('Failed to compress image'));
                return;
              }
              resolve({
                original: file,
                compressed: blob,
                originalSize: file.size,
                compressedSize: blob.size,
                preview: URL.createObjectURL(blob),
              });
            },
            'image/jpeg',
            quality / 100,
          );
        };
        img.onerror = () => reject(new Error('Failed to load image'));
        img.src = e.target?.result as string;
      };
      reader.onerror = () => reject(new Error('Failed to read file'));
      reader.readAsDataURL(file);
    });
  };

  const processFiles = async (files: FileList | File[]) => {
    const imageFiles = Array.from(files).filter((f) =>
      f.type.startsWith('image/'),
    );
    if (imageFiles.length === 0) {
      toast.error('Please select image files');
      return;
    }

    setIsProcessing(true);
    const results: CompressedImage[] = [];

    for (const file of imageFiles) {
      try {
        const result = await compressImage(file, quality());
        results.push(result);
      } catch (err) {
        toast.error(`Failed to compress ${file.name}`);
      }
    }

    setImages((prev) => [...prev, ...results]);
    setIsProcessing(false);
    if (results.length > 0) {
      toast.success(`Compressed ${results.length} image(s)`);
    }
  };

  const handleDrop = (e: DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    if (e.dataTransfer?.files) {
      processFiles(e.dataTransfer.files);
    }
  };

  const handleFileInput = (e: Event) => {
    const input = e.target as HTMLInputElement;
    if (input.files) {
      processFiles(input.files);
    }
  };

  const downloadImage = (img: CompressedImage) => {
    const link = document.createElement('a');
    link.href = img.preview;
    link.download = `compressed_${img.original.name.replace(/\.[^.]+$/, '')}.jpg`;
    link.click();
  };

  const downloadAll = () => {
    images().forEach(downloadImage);
  };

  const clearAll = () => {
    images().forEach((img) => URL.revokeObjectURL(img.preview));
    setImages([]);
  };

  const totalSaved = () => {
    const imgs = images();
    if (imgs.length === 0) return 0;
    const original = imgs.reduce((acc, img) => acc + img.originalSize, 0);
    const compressed = imgs.reduce((acc, img) => acc + img.compressedSize, 0);
    return ((original - compressed) / original) * 100;
  };

  return (
    <>
      <Title>Image Compressor | Asepharyana</Title>
      <main class="min-h-screen bg-background text-foreground p-4 md:p-8 lg:p-12">
        <div class="max-w-4xl mx-auto">
          {/* Header */}
          <Motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            class="text-center mb-8"
          >
            <div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-gradient-to-br from-cyan-500 via-blue-500 to-indigo-500 flex items-center justify-center shadow-lg">
              <span class="text-3xl">🖼️</span>
            </div>
            <h1 class="text-4xl font-bold gradient-text mb-2">
              Image Compressor
            </h1>
            <p class="text-muted-foreground">
              Compress your images to reduce file size while maintaining
              quality.
            </p>
          </Motion.div>

          {/* Quality Slider */}
          <Motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            class="glass-card rounded-xl p-6 mb-6"
          >
            <div class="flex items-center justify-between mb-4">
              <label class="text-sm font-medium">Quality</label>
              <span class="text-2xl font-bold gradient-text">{quality()}%</span>
            </div>
            <input
              type="range"
              min="10"
              max="100"
              value={quality()}
              onInput={(e) => setQuality(parseInt(e.target.value))}
              class="w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer accent-primary"
            />
            <div class="flex justify-between text-xs text-muted-foreground mt-2">
              <span>Smaller file</span>
              <span>Higher quality</span>
            </div>
          </Motion.div>

          {/* Drop Zone */}
          <Motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <label
              class={`relative block p-12 rounded-2xl border-2 border-dashed transition-all cursor-pointer ${
                isDragging()
                  ? 'border-primary bg-primary/10'
                  : 'border-border glass-card hover:border-primary/50'
              } ${isProcessing() ? 'pointer-events-none opacity-60' : ''}`}
              onDragOver={(e) => {
                e.preventDefault();
                setIsDragging(true);
              }}
              onDragLeave={() => setIsDragging(false)}
              onDrop={handleDrop}
            >
              <input
                type="file"
                accept="image/*"
                multiple
                class="hidden"
                onChange={handleFileInput}
                disabled={isProcessing()}
              />
              <div class="text-center">
                <Show
                  when={isProcessing()}
                  fallback={
                    <>
                      <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-primary/10 flex items-center justify-center">
                        <svg
                          class="w-8 h-8 text-primary"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                          />
                        </svg>
                      </div>
                      <p class="text-lg font-medium mb-1">
                        Drop your images here
                      </p>
                      <p class="text-sm text-muted-foreground">
                        or click to browse
                      </p>
                      <p class="text-xs text-muted-foreground mt-4">
                        Supports: JPG, PNG, WebP, GIF
                      </p>
                    </>
                  }
                >
                  <div class="w-12 h-12 mx-auto mb-4 rounded-full border-2 border-primary border-t-transparent animate-spin" />
                  <p class="text-lg font-medium">Processing images...</p>
                </Show>
              </div>
            </label>
          </Motion.div>

          {/* Results */}
          <Show when={images().length > 0}>
            <Motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              class="mt-8"
            >
              {/* Summary */}
              <div class="flex items-center justify-between mb-6">
                <div class="flex items-center gap-4">
                  <div class="glass-subtle px-4 py-2 rounded-lg">
                    <span class="text-sm text-muted-foreground">Saved: </span>
                    <span class="font-bold text-green-500">
                      {totalSaved().toFixed(1)}%
                    </span>
                  </div>
                  <span class="text-sm text-muted-foreground">
                    {images().length} image(s)
                  </span>
                </div>
                <div class="flex gap-2">
                  <button
                    onClick={downloadAll}
                    class="px-4 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 text-sm font-medium transition-colors"
                  >
                    Download All
                  </button>
                  <button
                    onClick={clearAll}
                    class="px-4 py-2 rounded-lg bg-destructive/10 text-destructive hover:bg-destructive/20 text-sm font-medium transition-colors"
                  >
                    Clear
                  </button>
                </div>
              </div>

              {/* Image List */}
              <div class="space-y-4">
                <For each={images()}>
                  {(img) => (
                    <div class="glass-card rounded-xl p-4 flex items-center gap-4">
                      <img
                        src={img.preview}
                        alt={img.original.name}
                        class="w-16 h-16 rounded-lg object-cover"
                      />
                      <div class="flex-1 min-w-0">
                        <p class="font-medium truncate">{img.original.name}</p>
                        <div class="flex items-center gap-3 text-sm">
                          <span class="text-muted-foreground line-through">
                            {formatSize(img.originalSize)}
                          </span>
                          <svg
                            class="w-4 h-4 text-green-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M14 5l7 7m0 0l-7 7m7-7H3"
                            />
                          </svg>
                          <span class="text-green-500 font-medium">
                            {formatSize(img.compressedSize)}
                          </span>
                          <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/10 text-green-500">
                            -
                            {(
                              ((img.originalSize - img.compressedSize) /
                                img.originalSize) *
                              100
                            ).toFixed(0)}
                            %
                          </span>
                        </div>
                      </div>
                      <button
                        onClick={() => downloadImage(img)}
                        class="p-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
                      >
                        <svg
                          class="w-5 h-5"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                          />
                        </svg>
                      </button>
                    </div>
                  )}
                </For>
              </div>
            </Motion.div>
          </Show>

          {/* Features */}
          <Motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3 }}
            class="mt-12 text-center text-sm text-muted-foreground"
          >
            <div class="flex flex-wrap justify-center gap-6">
              <span class="flex items-center gap-2">
                <svg
                  class="w-4 h-4 text-green-500"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                Free to use
              </span>
              <span class="flex items-center gap-2">
                <svg
                  class="w-4 h-4 text-green-500"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                No signup required
              </span>
              <span class="flex items-center gap-2">
                <svg
                  class="w-4 h-4 text-green-500"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                Processed locally
              </span>
            </div>
          </Motion.div>
        </div>
      </main>
    </>
  );
}
