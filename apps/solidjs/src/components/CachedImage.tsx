/**
 * CachedImage Component
 *
 * A component that displays images. Since the API now automatically returns
 * CDN URLs, this component no longer needs to make POST requests.
 * It simply displays the image with proper loading states and error handling.
 */
import { createSignal, Show } from 'solid-js';

interface CachedImageProps {
  src: string;
  alt: string;
  class?: string;
  fallbackClass?: string;
  loading?: 'lazy' | 'eager';
}

export function CachedImage(props: CachedImageProps) {
  const [error, setError] = createSignal(false);
  const [loaded, setLoaded] = createSignal(false);

  return (
    <Show
      when={!error()}
      fallback={
        <div
          class={props.fallbackClass || props.class || 'bg-muted animate-pulse'}
        >
          <div class="flex items-center justify-center h-full text-muted-foreground">
            Failed to load image
          </div>
        </div>
      }
    >
      <Show when={!loaded()}>
        <div
          class={`absolute inset-0 ${props.fallbackClass || 'bg-muted animate-pulse'}`}
        />
      </Show>
      <img
        src={props.src}
        alt={props.alt}
        class={props.class}
        loading={props.loading || 'lazy'}
        onLoad={() => setLoaded(true)}
        onError={() => setError(true)}
      />
    </Show>
  );
}

/**
 * Simple image with lazy loading and fallback
 * Since API returns CDN URLs directly, no additional processing needed
 */
export function LazyImage(props: CachedImageProps) {
  const [loaded, setLoaded] = createSignal(false);
  const [error, setError] = createSignal(false);

  return (
    <div class="relative">
      <Show when={!loaded() && !error()}>
        <div
          class={`absolute inset-0 ${props.fallbackClass || 'bg-muted animate-pulse rounded'}`}
        />
      </Show>
      <Show
        when={!error()}
        fallback={
          <div
            class={
              props.fallbackClass ||
              props.class ||
              'bg-muted rounded flex items-center justify-center'
            }
          >
            <span class="text-sm text-muted-foreground">Image unavailable</span>
          </div>
        }
      >
        <img
          src={props.src}
          alt={props.alt}
          class={props.class}
          loading={props.loading || 'lazy'}
          onLoad={() => setLoaded(true)}
          onError={() => setError(true)}
        />
      </Show>
    </div>
  );
}
