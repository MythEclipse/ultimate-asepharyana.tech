use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::providers::{use_auth, use_theme, Theme};

#[component]
pub fn SettingsPage() -> impl IntoView {
    let auth = use_auth();
    let theme = use_theme();
    let navigate = use_navigate();

    // Redirect if not logged in
    create_effect(move |_| {
        if auth.user.get().is_none() {
             navigate("/login", Default::default());
        }
    });

    view! {
        <Title text="Settings | Asepharyana"/>
        <Show when=move || auth.user.get().is_some()>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8">
                <div class="max-w-2xl mx-auto">
                    <h1 class="text-3xl font-bold mb-8">"Settings"</h1>

                    // Profile Section
                    <section class="mb-8 p-6 rounded-xl bg-card border border-border">
                        <h2 class="text-xl font-semibold mb-4">"Profile"</h2>
                        <div class="space-y-4">
                            <div>
                                <label class="text-sm text-muted-foreground">"Name"</label>
                                <p class="font-medium">{move || auth.user.get().map(|u| u.name).unwrap_or_default()}</p>
                            </div>
                             <div>
                                <label class="text-sm text-muted-foreground">"Email"</label>
                                <p class="font-medium">{move || auth.user.get().map(|u| u.email).unwrap_or_default()}</p>
                            </div>
                        </div>
                    </section>
                    
                    // Appearance Section
                    <section class="mb-8 p-6 rounded-xl bg-card border border-border">
                        <h2 class="text-xl font-semibold mb-4">"Appearance"</h2>
                         <div class="flex gap-3">
                            <button
                                on:click=move |_| theme.set_theme.set(Theme::Light)
                                class=move || format!("flex-1 p-4 rounded-lg border-2 transition-all {}",
                                    if theme.theme.get() == Theme::Light { "border-primary bg-primary/10" } else { "border-border hover:border-primary/50" }
                                )
                            >
                                <span class="text-2xl mb-2 block">"☀️"</span>
                                <span class="font-medium">"Light"</span>
                            </button>
                             <button
                                on:click=move |_| theme.set_theme.set(Theme::Dark)
                                class=move || format!("flex-1 p-4 rounded-lg border-2 transition-all {}",
                                    if theme.theme.get() == Theme::Dark { "border-primary bg-primary/10" } else { "border-border hover:border-primary/50" }
                                )
                            >
                                <span class="text-2xl mb-2 block">"🌙"</span>
                                <span class="font-medium">"Dark"</span>
                            </button>
                             <button
                                on:click=move |_| theme.set_theme.set(Theme::System)
                                class=move || format!("flex-1 p-4 rounded-lg border-2 transition-all {}",
                                    if theme.theme.get() == Theme::System { "border-primary bg-primary/10" } else { "border-border hover:border-primary/50" }
                                )
                            >
                                <span class="text-2xl mb-2 block">"💻"</span>
                                <span class="font-medium">"System"</span>
                            </button>
                        </div>
                    </section>

                    // Danger Zone
                    <section class="p-6 rounded-xl bg-destructive/10 border border-destructive/20">
                         <h2 class="text-xl font-semibold mb-4 text-destructive">"Danger Zone"</h2>
                         <button
                            on:click=move |_| auth.logout.dispatch(())
                            class="px-6 py-3 rounded-lg bg-destructive text-destructive-foreground hover:bg-destructive/90 transition-colors"
                        >
                            "Logout"
                        </button>
                    </section>
                </div>
            </main>
        </Show>
    }
}
