// @refresh reload
import { MetaProvider, Title } from '@solidjs/meta';
import { Router, useLocation } from '@solidjs/router';
import { FileRoutes } from '@solidjs/start/router';
import {
  Suspense,
  type ParentProps,
  createEffect,
  createSignal,
  Show,
  ErrorBoundary,
} from 'solid-js';
import { Motion, Presence } from 'solid-motionone';
import './app.css';
import { ClientLayout } from './components/layout/ClientLayout';
import { NavigationProgress } from './components/NavigationProgress';

// Error fallback component with auto-refresh and API error detection
function ErrorFallback(props: { error: Error; reset: () => void }) {
  const [countdown, setCountdown] = createSignal(5);

  // Detect API/network errors
  const isApiError = () => {
    const msg = props.error.message.toLowerCase();
    return (
      msg.includes('fetch') ||
      msg.includes('network') ||
      msg.includes('http') ||
      msg.includes('api') ||
      msg.includes('cors') ||
      msg.includes('timeout') ||
      msg.includes('failed to fetch')
    );
  };

  // Auto-refresh countdown
  createEffect(() => {
    const timer = setInterval(() => {
      setCountdown((c) => {
        if (c <= 1) {
          clearInterval(timer);
          window.location.reload();
          return 0;
        }
        return c - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  });

  return (
    <div class="min-h-[50vh] flex items-center justify-center p-8">
      <div
        class={`max-w-md w-full border rounded-xl p-6 text-center ${
          isApiError()
            ? 'bg-orange-500/10 border-orange-500/30'
            : 'bg-destructive/10 border-destructive/20'
        }`}
      >
        <div
          class={`w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center ${
            isApiError() ? 'bg-orange-500/20' : 'bg-destructive/20'
          }`}
        >
          {isApiError() ? (
            <svg
              class="w-8 h-8 text-orange-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0"
              />
            </svg>
          ) : (
            <svg
              class="w-8 h-8 text-destructive"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
          )}
        </div>

        <h2 class="text-xl font-semibold text-foreground mb-2">
          {isApiError()
            ? '🔌 API Connection Error'
            : 'Oops! Something went wrong'}
        </h2>

        <p class="text-muted-foreground mb-2 text-sm">
          {props.error.message || 'An unexpected error occurred'}
        </p>

        {isApiError() && (
          <p class="text-orange-500/80 mb-4 text-xs font-mono bg-orange-500/10 px-2 py-1 rounded">
            Server may be temporarily unavailable
          </p>
        )}

        <p class="text-muted-foreground mb-4 text-sm">
          Auto-refreshing in{' '}
          <span class="font-bold text-primary">{countdown()}</span>s...
        </p>

        <div class="flex gap-3 justify-center">
          <button
            onClick={() => window.location.reload()}
            class="px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition-opacity"
          >
            Refresh Now
          </button>
          <button
            onClick={() => (window.location.href = '/')}
            class="px-4 py-2 bg-secondary text-secondary-foreground rounded-lg hover:opacity-90 transition-opacity"
          >
            Go Home
          </button>
        </div>
      </div>
    </div>
  );
}

// Page transition wrapper component
function PageTransition(props: ParentProps) {
  const location = useLocation();
  const [currentPath, setCurrentPath] = createSignal(location.pathname);

  createEffect(() => {
    setCurrentPath(location.pathname);
  });

  return (
    <Presence exitBeforeEnter>
      <Show when={currentPath()}>
        <Motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -20 }}
          transition={{ duration: 0.3, easing: [0.4, 0, 0.2, 1] }}
          class="w-full"
        >
          {props.children}
        </Motion.div>
      </Show>
    </Presence>
  );
}

function RootLayout(props: ParentProps) {
  return (
    <MetaProvider>
      <Title>Asepharyana</Title>
      <NavigationProgress />
      <ClientLayout>
        <ErrorBoundary
          fallback={(err, reset) => <ErrorFallback error={err} reset={reset} />}
        >
          <Suspense
            fallback={
              <div class="p-8 text-center flex items-center justify-center min-h-[50vh]">
                <div class="flex flex-col items-center gap-4">
                  <div class="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin" />
                  <span class="text-muted-foreground">Loading...</span>
                </div>
              </div>
            }
          >
            <PageTransition>{props.children}</PageTransition>
          </Suspense>
        </ErrorBoundary>
      </ClientLayout>
    </MetaProvider>
  );
}

export default function App() {
  return (
    <Router root={RootLayout}>
      <FileRoutes />
    </Router>
  );
}
