use leptos::*;
use std::time::Duration;
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
    let (show_heart_anim, set_show_heart_anim) = create_signal(false);
    let (image_loaded, set_image_loaded) = create_signal(false);

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
            set_show_heart_anim.set(true);
            // Reset animation state after a short delay
            set_timeout(move || set_show_heart_anim.set(false), Duration::from_millis(800));

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

    let post_val = move || post.get();
    let user_name = move || post_val().user.as_ref().map(|u| u.name.clone()).unwrap_or("Unknown".to_string());
    let user_image = move || post_val().user.as_ref().and_then(|u| u.image.clone()).unwrap_or_else(|| format!("https://ui-avatars.com/api/?name={}", user_name()));

    view! {
        <article class="glass-card p-8 rounded-[2.5rem] border border-white/5 transition-all duration-700 hover:border-white/20 group/card relative overflow-hidden shadow-2xl">
            <div class="absolute inset-0 bg-indigo-500/5 blur-3xl opacity-0 group-hover/card:opacity-100 transition-opacity pointer-events-none" />
            
            // Header
            <div class="flex justify-between items-center mb-8">
                <div class="flex gap-4 items-center">
                    <div class="relative">
                        <img
                            src=user_image
                            alt=user_name.clone()
                            class="w-12 h-12 rounded-2xl object-cover border-2 border-white/10 shadow-xl"
                        />
                        <div class="absolute -bottom-1 -right-1 w-4 h-4 rounded-full bg-green-500 border-2 border-muted shadow-lg" />
                    </div>
                    <div>
                        <h3 class="font-black italic tracking-tighter uppercase text-sm group-hover/card:text-indigo-400 transition-colors">{user_name}</h3>
                        <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">
                            {move || post.get().created_at}
                        </p>
                    </div>
                </div>

                <div class="flex items-center gap-2">
                    <Show when=is_owner>
                        <button
                            on:click=handle_delete
                            class="w-10 h-10 rounded-xl bg-white/2 border border-white/5 flex items-center justify-center text-muted-foreground/40 hover:text-red-500 hover:bg-red-500/10 hover:border-red-500/20 transition-all active:scale-95"
                        >
                             <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                        </button>
                    </Show>
                </div>
            </div>

            // Content
            <div class="space-y-6">
                <p class="text-foreground font-medium text-lg leading-relaxed tracking-tight whitespace-pre-wrap">
                    {move || post.get().content}
                </p>

                <Show when=move || post.get().image_url.is_some()>
                    <div class="relative rounded-[2rem] overflow-hidden border border-white/10 shadow-2xl group/img bg-white/5">
                        <div class=move || format!("absolute inset-0 bg-white/5 animate-shimmer {}", if image_loaded.get() { "hidden" } else { "block" }) />
                        <img
                            src=move || post.get().image_url.unwrap_or_default()
                            alt="Visual Attachment"
                            on:load=move |_| set_image_loaded.set(true)
                            class=move || format!(
                                "w-full h-auto max-h-[600px] object-cover transition-all duration-1000 group-hover/img:scale-105 {}",
                                if image_loaded.get() { "opacity-100" } else { "opacity-0" }
                            )
                        />
                        <div class="absolute inset-0 bg-gradient-to-t from-black/40 via-transparent to-transparent opacity-0 group-hover/img:opacity-100 transition-opacity" />
                    </div>
                </Show>
            </div>

            // Actions
            <div class="flex items-center gap-4 pt-8 mt-8 border-t border-white/5 relative">
                // Heart Explosion layer
                <Show when=move || show_heart_anim.get()>
                     <div class="absolute left-12 -top-12 pointer-events-none z-50">
                        <span class="absolute animate-bounce-in text-4xl">"❤️"</span>
                        <span class="absolute animate-ping text-4xl text-red-500 opacity-75">"❤️"</span>
                     </div>
                </Show>

                <button
                    on:click=handle_like
                    disabled=move || is_liking.get()
                    class=move || format!(
                        "flex-1 flex items-center justify-center gap-3 py-3 rounded-2xl glass transition-all active:scale-95 group/like relative overflow-hidden {}",
                        if is_liked() { "bg-red-500/10 text-red-500 border-red-500/40" } else { "border-white/5 text-muted-foreground/60 hover:bg-white/5 hover:text-foreground" }
                    )
                >
                    <div class="absolute inset-0 bg-red-500/20 scale-0 group-hover/like:scale-150 transition-transform duration-500 rounded-full blur-xl opacity-0 group-hover/like:opacity-100" />
                    <svg
                        class=move || format!("w-5 h-5 transition-all duration-300 relative z-10 {}",
                            if is_liked() { "scale-110 fill-current" } else { "fill-none group-hover/like:scale-110" },
                        )
                        stroke="currentColor"
                        stroke-width="2.5"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                    <span class="text-[10px] font-black uppercase tracking-widest relative z-10">{move || post.get().likes.len()}</span>
                </button>

                <button
                    on:click=move |_| set_show_comments.update(|s| *s = !*s)
                    class="flex-1 flex items-center justify-center gap-3 py-3 rounded-2xl glass border border-white/5 text-muted-foreground/60 hover:bg-white/5 hover:text-indigo-400 hover:border-indigo-500/40 transition-all active:scale-95"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                    <span class="text-[10px] font-black uppercase tracking-widest">{move || post.get().comments.len()}</span>
                </button>
            </div>

            <Show when=move || show_comments.get()>
                <div class="mt-8 pt-8 border-t border-white/5 animate-slide-up">
                    <CommentSection
                        post_id=Signal::derive(move || post.get().id)
                        comments=Signal::derive(move || post.get().comments)
                    />
                </div>
            </Show>
        </article>
    }
}
