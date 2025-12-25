/**
 * CachedImage Component
 * 
 * A component that automatically caches images via the CDN service.
 * Falls back to original URL if caching fails.
 */
import { createResource, createSignal, Show, onMount } from "solid-js";
import { getCachedImageUrl } from "~/lib/image-cache";

interface CachedImageProps {
    src: string;
    alt: string;
    class?: string;
    fallbackClass?: string;
    loading?: "lazy" | "eager";
}

export function CachedImage(props: CachedImageProps) {
    const [cachedSrc, setCachedSrc] = createSignal<string | null>(null);
    const [error, setError] = createSignal(false);

    onMount(async () => {
        if (!props.src) {
            setError(true);
            return;
        }

        try {
            const cdn = await getCachedImageUrl(props.src);
            setCachedSrc(cdn);
        } catch {
            // Fallback to original
            setCachedSrc(props.src);
        }
    });

    return (
        <Show
            when={cachedSrc() && !error()}
            fallback={
                <div class={props.fallbackClass || props.class || "bg-muted animate-pulse"} />
            }
        >
            <img
                src={cachedSrc()!}
                alt={props.alt}
                class={props.class}
                loading={props.loading || "lazy"}
                onError={() => {
                    // If CDN fails, try original
                    if (cachedSrc() !== props.src) {
                        setCachedSrc(props.src);
                    } else {
                        setError(true);
                    }
                }}
            />
        </Show>
    );
}

/**
 * Simple image with lazy loading and fallback
 * Uses CDN cache automatically when available
 */
export function LazyImage(props: CachedImageProps) {
    const [loaded, setLoaded] = createSignal(false);
    const [finalSrc, setFinalSrc] = createSignal(props.src);

    // Try to get cached URL in background
    onMount(async () => {
        try {
            const cdn = await getCachedImageUrl(props.src);
            if (cdn && cdn !== props.src) {
                setFinalSrc(cdn);
            }
        } catch {
            // Keep original
        }
    });

    return (
        <div class="relative">
            <Show when={!loaded()}>
                <div class={`absolute inset-0 ${props.fallbackClass || "bg-muted animate-pulse rounded"}`} />
            </Show>
            <img
                src={finalSrc()}
                alt={props.alt}
                class={props.class}
                loading={props.loading || "lazy"}
                onLoad={() => setLoaded(true)}
                onError={() => {
                    // Fallback to original if CDN fails
                    if (finalSrc() !== props.src) {
                        setFinalSrc(props.src);
                    }
                }}
            />
        </div>
    );
}
