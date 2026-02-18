use leptos::*;
use crate::types::Post;
use crate::providers::use_auth;
use gloo_storage::{LocalStorage, Storage};
use super::comment_section::CommentSection;

#[component]
pub fn PostCard(
    post: Post,
    #[prop(into)]
    on_post_updated: Callback<Post>,
    #[prop(into)]
    on_delete: Callback<String>,
) -> impl IntoView {
    let _ = on_post_updated;
    let _ = on_delete;
    let auth = use_auth();
    let user = auth.user;
    
    let (post, _set_post) = create_signal(post);
    let (show_comments, set_show_comments) = create_signal(false);
    let (is_liking, set_is_liking) = create_signal(false);

    let is_liked = move || {
        let current_user_id = user.get().map(|u| u.id);
        if let Some(uid) = current_user_id {
            post.get().likes.iter().any(|l| l.user_id == uid)
        } else {
            false
        }
    };
    
    let is_owner = move || {
        let current_user_id = user.get().map(|u| u.id);
        if let Some(uid) = current_user_id {
            post.get().user_id == uid
        } else {
            false
        }
    };

    let handle_like = move |_| {
        if is_liking.get() { return; }
        
        if let Ok(token) = LocalStorage::get::<String>("access_token") {
            set_is_liking.set(true);
            let post_id = post.get().id;
            
            spawn_local(async move {
                match crate::api::social::like_post(token, post_id).await {
                    Ok(msg) => {
                        logging::log!("Success: {}", msg);
                    }
                    Err(e) => logging::error!("Like error: {}", e.message),
                }
                set_is_liking.set(false);
            });
        } else {
            logging::warn!("Login required to like");
        }
    };

    let handle_delete = move |_| {
        if let Ok(token) = LocalStorage::get::<String>("access_token") {
            let post_id = post.get().id;
            let on_delete_cb = on_delete;
            
            spawn_local(async move {
                match crate::api::social::delete_post(token, post_id.clone()).await {
                    Ok(_) => {
                        on_delete_cb.call(post_id);
                    }
                    Err(e) => logging::error!("Delete error: {}", e.message),
                }
            });
        }
    };

    // Derived values for view
    let post_val = move || post.get();
    let image_url = move || post.get().image_url;
    
    let user_name = move || post_val().user.as_ref().map(|u| u.name.clone()).unwrap_or("Unknown".to_string());
    let user_image = move || post_val().user.as_ref().and_then(|u| u.image.clone()).unwrap_or_else(|| format!("https://ui-avatars.com/api/?name={}", user_name()));

    view! {
        <div class="glass-card p-6 rounded-2xl mb-6 last:mb-0 transition-opacity duration-500 animate-fade-in">
             // Header
            <div class="flex justify-between items-start mb-4">
                <div class="flex gap-3 items-center">
                    <img
                        src=user_image
                        alt=user_name
                        class="w-10 h-10 rounded-full object-cover border border-border"
                    />
                    <div>
                        <h3 class="font-semibold">{user_name}</h3>
                        <p class="text-xs text-muted-foreground">
                            {move || post.get().created_at} // Format distance to now TODO
                        </p>
                    </div>
                </div>

                <Show when=is_owner>
                    <button
                        on:click=handle_delete
                        class="text-muted-foreground hover:text-destructive transition-colors p-2"
                        title="Delete Post"
                    >
                         <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                        </svg>
                    </button>
                </Show>
            </div>

            // Content
            <p class="text-foreground/90 mb-4 whitespace-pre-wrap">{move || post.get().content}</p>

            <Show when=move || image_url().is_some()>
                <div class="mb-4 rounded-xl overflow-hidden border border-border/50">
                    <img
                        src=move || image_url().unwrap()
                        alt="Post content"
                        class="w-full h-auto max-h-[500px] object-cover"
                    />
                </div>
            </Show>

            // Actions
            <div class="flex items-center gap-6 pt-4 border-t border-border/50">
                <button
                    on:click=handle_like
                    disabled=move || is_liking.get()
                    class=move || format!("flex items-center gap-2 text-sm font-medium transition-colors {}", 
                        if is_liked() { "text-red-500" } else { "text-muted-foreground hover:text-red-500" }
                    )
                >
                    <svg
                        class=move || format!("w-5 h-5 {}", if is_liked() { "fill-current" } else { "fill-none" })
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                    <span>{move || post.get().likes.len()} " Likes"</span>
                </button>

                <button
                    on:click=move |_| set_show_comments.update(|s| *s = !*s)
                    class="flex items-center gap-2 text-sm font-medium text-muted-foreground hover:text-primary transition-colors"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                    <span>{move || post.get().comments.len()} " Comments"</span>
                </button>
            </div>

            <Show when=move || show_comments.get()>
                <CommentSection
                    post_id=Signal::derive(move || post.get().id)
                    comments=Signal::derive(move || post.get().comments)
                />
            </Show>
        </div>
    }
}
