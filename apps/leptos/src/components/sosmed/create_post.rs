use leptos::*;
use crate::api::social::{create_post, CreatePostRequest};
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn CreatePost(
    #[prop(into)]
    on_post_created: Callback<()>,
) -> impl IntoView {
    let (content, set_content) = create_signal(String::new());
    let (image_url, set_image_url) = create_signal(String::new());
    let (is_submitting, set_is_submitting) = create_signal(false);
    let (is_success, set_is_success) = create_signal(false);
    let (is_focused, set_is_focused) = create_signal(false);

    let handle_submit = move |_| {
        let content_val = content.get();
        if content_val.is_empty() { return; }

        if let Ok(token) = LocalStorage::get::<String>("access_token") {
            set_is_submitting.set(true);
            let img = image_url.get();
            let img_opt = if img.is_empty() { None } else { Some(img) };
            
            let on_created = on_post_created;
            spawn_local(async move {
                let req = CreatePostRequest {
                    content: content_val,
                    image_url: img_opt,
                };
                match create_post(token, req).await {
                    Ok(_) => {
                        set_content.set(String::new());
                        set_image_url.set(String::new());
                        set_is_success.set(true);
                        set_timeout(move || set_is_success.set(false), std::time::Duration::from_millis(2000));
                        on_created.call(());
                    }
                    Err(e) => logging::error!("Create post error: {}", e.message),
                }
                set_is_submitting.set(false);
            });
        }
    };

    view! {
        <div 
            class=move || format!(
                "glass-card p-2 rounded-[2rem] transition-all duration-700 shadow-2xl relative overflow-hidden group {}",
                if is_focused.get() { "border-indigo-500/30 ring-4 ring-indigo-500/10 scale-[1.01]" } else { "border-white/10 hover:border-white/20" }
            )
        >
            <div class=move || format!(
                "absolute inset-0 bg-indigo-500/5 blur-3xl transition-opacity duration-700 pointer-events-none {}",
                if is_focused.get() { "opacity-100" } else { "opacity-0 group-hover:opacity-50" }
            ) />
            
            <div class="relative p-6 space-y-4">
                <div class="flex items-center gap-4 mb-2">
                    <div class=move || format!(
                        "w-10 h-10 rounded-2xl flex items-center justify-center text-xl shadow-2xl transition-all duration-500 {}",
                        if is_focused.get() { "bg-indigo-500 text-white scale-110 rotate-3" } else { "bg-indigo-500/20" }
                    )>"✍️"</div>
                    <span class=move || format!(
                        "text-[10px] font-black uppercase tracking-[0.4em] transition-colors duration-300 {}",
                        if is_focused.get() { "text-indigo-400" } else { "text-muted-foreground/60" }
                    )>"Create Post"</span>
                </div>

                <textarea
                    prop:value=content
                    on:input=move |e| set_content.set(event_target_value(&e))
                    on:focus=move |_| set_is_focused.set(true)
                    on:blur=move |_| set_is_focused.set(false)
                    placeholder="Share your thoughts with the community..."
                    class="w-full h-32 bg-white/2 border border-white/5 rounded-2xl p-6 focus:outline-none focus:border-indigo-500/30 transition-all resize-none text-foreground placeholder:text-muted-foreground/30 font-medium tracking-tight"
                ></textarea>
                
                <div class="flex flex-col md:flex-row gap-4 items-center">
                    <div class="relative flex-1 w-full">
                        <div class="absolute left-4 top-1/2 -translate-y-1/2 text-muted-foreground/40">
                             <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                        </div>
                        <input
                            type="text"
                            prop:value=image_url
                            on:input=move |e| set_image_url.set(event_target_value(&e))
                            on:focus=move |_| set_is_focused.set(true)
                            on:blur=move |_| set_is_focused.set(false)
                            placeholder="Image URL (optional)"
                            class="w-full bg-white/2 border border-white/5 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:border-indigo-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/20"
                        />
                    </div>

                    <button
                        on:click=handle_submit
                        disabled=move || is_submitting.get() || content.get().is_empty()
                        class=move || format!(
                            "w-full md:w-auto px-10 py-3.5 rounded-2xl font-black uppercase text-xs tracking-widest hover:scale-105 active:scale-95 transition-all shadow-2xl disabled:opacity-30 disabled:scale-100 flex items-center justify-center gap-3 group/btn overflow-hidden relative duration-500 {}",
                            if is_success.get() { "bg-green-500 text-white" } else { "bg-foreground text-background" }
                        )
                    >
                        <Show when=move || is_submitting.get() fallback=move || {
                            view! {
                                <Show when=move || is_success.get() fallback=move || view! {
                                    <span class="relative z-10 transition-transform group-hover/btn:translate-x-1">"Post"</span>
                                    <svg class="w-4 h-4 relative z-10 group-hover/btn:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                    </svg>
                                }>
                                    <span class="relative z-10">"Sent!"</span>
                                    <svg class="w-4 h-4 relative z-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                                    </svg>
                                </Show>
                            }
                        }>
                            <div class="w-4 h-4 border-2 border-background/30 border-t-background rounded-full animate-spin"></div>
                        </Show>
                        <div class="absolute inset-0 bg-gradient-to-r from-indigo-500 to-purple-500 opacity-0 group-hover/btn:opacity-10 transition-opacity" />
                    </button>
                </div>
            </div>
        </div>
    }
}
