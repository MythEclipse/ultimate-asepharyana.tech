use crate::pages::auth::login::LoginPage;
use crate::pages::auth::register::RegisterPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::home::HomePage;
use crate::pages::project::ProjectPage;
use crate::pages::settings::SettingsPage;
use crate::pages::sosmed::SosmedPage;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::components::layout::ClientLayout;
use crate::components::ui::{ErrorFallback, PageTransition};

use crate::pages::anime::AnimePage;
use crate::pages::komik::KomikPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Dispatch `leptos-content-ready` once after the first reactive render
    // cycle completes. Index.html waits for this (not `app-ready`) to hide
    // the initial loading overlay, so the overlay covers both WASM download
    // AND the first data fetch — resulting in ONE seamless loading screen.
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            let cb = Closure::once(move || {
                if let Some(w) = web_sys::window() {
                    let _ = w.dispatch_event(
                        &web_sys::Event::new("leptos-content-ready").unwrap(),
                    );
                }
            });
            // Use a short delay so Suspense fallbacks have time to resolve
            // for fast connections; on slow ones the overlay stays until data comes
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                100,
            );
            cb.forget();
        }
    });

    view! {
        <Meta name="theme-color" content="#6366f1"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0"/>

        <Router>
            <ErrorBoundary fallback=move |err| {
                let msg = format!("{:?}", err.get());
                view! { <ErrorFallback error=msg/> }
            }>
                <ClientLayout>
                    <PageTransition>
                        <Routes>
                            <Route path="" view=HomePage/>
                            <Route path="login" view=LoginPage/>
                            <Route path="register" view=RegisterPage/>
                            <Route path="dashboard" view=DashboardPage/>
                            <Route path="project" view=ProjectPage/>
                            <Route path="settings" view=SettingsPage/>
                            <Route path="sosmed" view=SosmedPage/>
                            <Route path="anime" view=|| view! { <AnimePage source=1 /> }/>
                            <Route path="anime2" view=|| view! { <AnimePage source=2 /> }/>
                            <Route path="anime/detail/:slug" view=crate::pages::anime::detail::AnimeDetailPage/>
                            <Route path="anime/watch/:slug" view=crate::pages::anime::watch::WatchPage/>
                            <Route path="anime2/detail/:slug" view=crate::pages::anime::detail::AnimeDetailPage/>
                            <Route path="anime2/watch/:slug" view=crate::pages::anime::watch::WatchPage/>
                            <Route path="anime/search" view=crate::pages::anime::search::AnimeSearchPage/>
                            <Route path="anime2/search" view=crate::pages::anime::search::AnimeSearchPage/>
                            <Route path="komik" view=KomikPage/>
                            <Route path="komik/detail" view=crate::pages::komik::detail::KomikDetailPage/>
                            <Route path="komik/read/:slug" view=crate::pages::komik::read::ReadPage/>
                            <Route path="komik/search" view=crate::pages::komik::search::KomikSearchPage/>
                            <Route path="/*any" view=NotFound/>
                        </Routes>
                    </PageTransition>
                </ClientLayout>
            </ErrorBoundary>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_axum::ResponseOptions>();
        resp.set_status(StatusCode::NOT_FOUND);
    }

    view! {
        <div class="min-h-screen flex items-center justify-center p-8 relative overflow-hidden scanlines">
            <div class="absolute inset-0 opacity-20 pointer-events-none">
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[40rem] h-[40rem] bg-destructive/10 rounded-full blur-[120px] animate-pulse" />
            </div>
            
            <div class="relative z-10 text-center space-y-10">
                <div class="space-y-4">
                    <h1 class="text-8xl md:text-11xl font-black italic tracking-tighter text-destructive/80 font-display animate-bounce">"404"</h1>
                    <div class="h-1 w-24 bg-destructive/40 mx-auto rounded-full" />
                </div>
                
                <div class="space-y-2">
                    <h2 class="text-2xl md:text-4xl font-black italic uppercase tracking-tighter">"Anomaly Detected"</h2>
                    <p class="text-muted-foreground/60 max-w-sm mx-auto font-medium italic">"The requested coordinate does not exist in the current data stream."</p>
                </div>

                <A href="/" class="inline-flex items-center gap-4 px-10 py-5 rounded-full bg-foreground text-background font-black text-[10px] uppercase tracking-[0.4em] hover:scale-105 active:scale-95 transition-all shadow-2xl">
                    "Back to Reality"
                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                    </svg>
                </A>
            </div>
        </div>
    }
}
