use leptos::*;

#[component]
pub fn CachedImage(
    src: String,
    alt: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] fallback_class: Option<String>,
    #[prop(optional)] loading: Option<String>,
) -> impl IntoView {
    let (error, set_error) = create_signal(false);
    let (loaded, set_loaded) = create_signal(false);

    let fallback_cls = fallback_class.unwrap_or_else(|| "bg-muted animate-pulse".to_string());
    let fallback_cls_err = fallback_cls.clone();
    let cls = class.unwrap_or_default();
    let load_mode = loading.unwrap_or_else(|| "lazy".to_string());

    view! {
        <div class="relative w-full h-full">
            <Show when=move || !loaded.get() && !error.get()>
                 <div class=format!("absolute inset-0 {}", fallback_cls) />
            </Show>
            
            <Show when=move || error.get()>
                 <div class=format!("{} flex items-center justify-center text-muted-foreground", fallback_cls_err)>
                    "Failed to load image"
                </div>
            </Show>

            <img
                src=src
                alt=alt
                class=move || format!("{} {}", cls, if !loaded.get() || error.get() { "opacity-0" } else { "opacity-100" })
                loading=load_mode
                on:load=move |_| set_loaded.set(true)
                on:error=move |_| set_error.set(true)
            />
        </div>
    }
}
