use leptos::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::ui::navigation_progress::NavigationProgress;
use crate::providers::{provide_theme, provide_auth};

#[component]
pub fn ClientLayout(children: Children) -> impl IntoView {
    // Provide contexts at the layout level
    provide_theme();
    provide_auth();

    view! {
        <div class="min-h-screen flex flex-col">
            <NavigationProgress/>
            <Navbar/>
            <main class="flex-1">
                {children()}
            </main>
            // Footer removed to match SolidJS
        </div>
        // Toaster would go here
    }
}
