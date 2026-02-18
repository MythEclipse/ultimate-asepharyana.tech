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
use http::StatusCode;

use crate::components::layout::ClientLayout;
use crate::components::ui::{ErrorFallback, PageTransition};

use crate::pages::anime::AnimePage;
use crate::pages::komik::KomikPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:class="bg-background text-foreground"/>
        
        <Stylesheet id="leptos" href="/pkg/apps-leptos.css"/>

        <Title text="Asepharyana"/>

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
                            <Route path="anime" view=AnimePage/>
                            <Route path="anime/detail/:slug" view=crate::pages::anime::detail::AnimeDetailPage/>
                            <Route path="anime/search" view=crate::pages::anime::search::AnimeSearchPage/>
                            <Route path="komik" view=KomikPage/>
                            <Route path="komik/detail" view=crate::pages::komik::detail::KomikDetailPage/>
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
        <div class="min-h-screen flex items-center justify-center p-8">
            <div class="text-center">
                <h1 class="text-4xl font-bold text-destructive mb-4">"404 - Not Found"</h1>
                <p class="text-muted-foreground">"The page you are looking for does not exist."</p>
            </div>
        </div>
    }
}
