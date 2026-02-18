use leptos::*;


#[component]
pub fn Footer() -> impl IntoView {
    let current_year = chrono::Utc::now().format("%Y").to_string();

    view! {
        <footer class="border-t border-white/10 mt-auto py-6 bg-background/50">
            <div class="container mx-auto px-4">
                <div class="flex flex-col sm:flex-row items-center justify-between gap-4 text-center sm:text-left">
                    <a href="/" class="flex items-center gap-2 group">
                        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-purple-600 flex items-center justify-center shadow-md group-hover:scale-105 transition-transform">
                            <span class="text-white font-bold text-sm">"A"</span>
                        </div>
                        <span class="font-semibold text-foreground">"Asepharyana"</span>
                    </a>
                    <p class="text-xs text-muted-foreground">
                        "© " {current_year} " Asepharyana. Made with ❤️ in Indonesia"
                    </p>
                </div>
            </div>
        </footer>
    }
}
