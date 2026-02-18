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
                        on_created.call(());
                    }
                    Err(e) => logging::error!("Create post error: {}", e.message),
                }
                set_is_submitting.set(false);
            });
        } else {
            logging::warn!("Token not found, please login");
        }
    };

    view! {
        <div class="glass-card p-6 rounded-2xl mb-8 transition-all hover:border-primary/50 shadow-lg">
            <h2 class="text-xl font-bold mb-4 gradient-text">"Create Post"</h2>
            <div class="space-y-4">
                <textarea
                    prop:value=content
                    on:input=move |e| set_content.set(event_target_value(&e))
                    placeholder="What's on your mind?"
                    class="w-full h-32 bg-background/50 border border-border rounded-xl p-4 focus:outline-none focus:ring-2 focus:ring-primary/50 transition-all resize-none"
                ></textarea>
                
                <input
                    type="text"
                    prop:value=image_url
                    on:input=move |e| set_image_url.set(event_target_value(&e))
                    placeholder="Image URL (optional)"
                    class="w-full bg-background/50 border border-border rounded-xl p-3 px-4 focus:outline-none focus:ring-2 focus:ring-primary/50 transition-all text-sm"
                />

                <div class="flex justify-end">
                    <button
                        on:click=handle_submit
                        disabled=move || is_submitting.get() || content.get().is_empty()
                        class="bg-primary text-primary-foreground px-6 py-2.5 rounded-xl font-semibold hover:opacity-90 transition-all active:scale-95 disabled:opacity-50 disabled:active:scale-100 flex items-center gap-2"
                    >
                        <Show when=move || is_submitting.get()>
                            <div class="w-4 h-4 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
                        </Show>
                        "Post"
                    </button>
                </div>
            </div>
        </div>
    }
}
