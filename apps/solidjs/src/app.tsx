// @refresh reload
import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense, type ParentProps } from "solid-js";
import "./app.css";
import { ClientLayout } from "./components/layout/ClientLayout";

function RootLayout(props: ParentProps) {
  return (
    <MetaProvider>
      <Title>Asepharyana</Title>
      <ClientLayout>
        <Suspense fallback={<div class="p-8 text-center">Loading...</div>}>
          {props.children}
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
