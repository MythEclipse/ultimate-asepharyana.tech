import { useIsRouting } from '@solidjs/router';
import { Show, createEffect, createSignal, onCleanup, For } from 'solid-js';

/**
 * A top navigation progress bar that shows during route transitions
 * Similar to NProgress or YouTube's loading bar
 */
export function NavigationProgress() {
  const isRouting = useIsRouting();
  const [progress, setProgress] = createSignal(0);
  const [visible, setVisible] = createSignal(false);

  createEffect(() => {
    if (isRouting()) {
      // Start showing progress
      setVisible(true);
      setProgress(0);

      // Simulate progress with easing
      let currentProgress = 0;
      const interval = setInterval(() => {
        // Slow down as we approach 90%
        const increment = (90 - currentProgress) * 0.1;
        currentProgress = Math.min(
          90,
          currentProgress + Math.max(0.5, increment),
        );
        setProgress(currentProgress);
      }, 100);

      onCleanup(() => clearInterval(interval));
    } else {
      // Complete the progress bar
      setProgress(100);

      // Hide after animation completes
      const timeout = setTimeout(() => {
        setVisible(false);
        setProgress(0);
      }, 300);

      onCleanup(() => clearTimeout(timeout));
    }
  });

  return (
    <Show when={visible()}>
      <div class="fixed top-0 left-0 right-0 z-[9999] h-1 bg-transparent">
        {/* Glow effect */}
        <div
          class="h-full bg-gradient-to-r from-primary via-accent to-primary shadow-lg shadow-primary/50 transition-all duration-200 ease-out"
          style={{ width: `${progress()}%` }}
        />
        {/* Shimmer effect */}
        <div
          class="absolute top-0 right-0 h-full w-24 bg-gradient-to-l from-white/30 to-transparent animate-pulse"
          style={{
            transform: `translateX(${progress() < 100 ? '0' : '100%'})`,
            opacity: progress() < 100 ? 1 : 0,
          }}
        />
      </div>
    </Show>
  );
}

/**
 * Full-page loading overlay for initial loads
 */
export function PageLoadingOverlay() {
  return (
    <div class="fixed inset-0 z-[9998] bg-background/80 backdrop-blur-sm flex items-center justify-center">
      <div class="flex flex-col items-center gap-4">
        {/* Animated spinner with gradient */}
        <div class="relative">
          <div class="w-16 h-16 rounded-full border-4 border-primary/20 border-t-primary animate-spin" />
          <div class="absolute inset-0 w-16 h-16 rounded-full bg-gradient-to-r from-primary/0 via-primary/10 to-primary/0 animate-pulse" />
        </div>
        <div class="text-muted-foreground font-medium animate-pulse">
          Loading...
        </div>
      </div>
    </div>
  );
}

/**
 * Skeleton loading component for content areas
 */
export function ContentSkeleton(props: { lines?: number; className?: string }) {
  const lines = props.lines || 3;
  return (
    <div class={`space-y-3 ${props.className || ''}`}>
      <For each={Array.from({ length: props.lines || 3 })}>
        {(item, index) => (
          <div
            class="h-4 shimmer rounded"
            style={{ width: `${100 - index() * 15}%` }}
          />
        )}
      </For>
    </div>
  );
}
