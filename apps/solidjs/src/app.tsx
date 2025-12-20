// @refresh reload
import { MetaProvider, Title } from "@solidjs/meta";
import { Router, useLocation } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense, type ParentProps, createEffect, createSignal, Show } from "solid-js";
import { Motion, Presence } from "solid-motionone";
import "./app.css";
import { ClientLayout } from "./components/layout/ClientLayout";

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
      <ClientLayout>
        <Suspense fallback={
          <div class="p-8 text-center flex items-center justify-center min-h-[50vh]">
            <div class="flex flex-col items-center gap-4">
              <div class="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin" />
              <span class="text-muted-foreground">Loading...</span>
            </div>
          </div>
        }>
          <PageTransition>
            {props.children}
          </PageTransition>
        </Suspense>
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
