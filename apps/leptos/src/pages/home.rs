use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|n| *n += 1);

    view! {
        <div class="p-8 text-center flex flex-col items-center gap-4 min-h-[calc(100vh-4rem)] justify-center animate-fade-in">
            <h1 class="text-4xl font-bold gradient-text mb-4">"Welcome to Leptos"</h1>
            <p class="text-muted-foreground mb-8">"Ported from SolidJS with ❤️"</p>
            <div class="glass p-8 rounded-xl">
                 <button class="px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition-opacity hover-lift" on:click=on_click>
                    "Click Me: " {count}
                 </button>
            </div>
        </div>
    }
}
