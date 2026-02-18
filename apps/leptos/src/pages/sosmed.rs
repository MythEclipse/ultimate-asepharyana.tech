use leptos::*;
use crate::components::sosmed::post_card::PostCard;
use crate::components::sosmed::create_post::CreatePost;
use crate::api::social::get_posts;

#[component]
pub fn SosmedPage() -> impl IntoView {
    let posts_resource = create_resource(|| (), |_| async move { get_posts().await });

    let handle_refresh = move |_: crate::types::Post| {
        posts_resource.refetch();
    };

    let handle_delete = move |post_id: String| {
        logging::log!("Refetching after delete of {}", post_id);
        posts_resource.refetch();
    };

    let handle_post_created = move |_: ()| {
        posts_resource.refetch();
    };

    view! {
        <div class="container mx-auto max-w-2xl px-4 py-8">
            <h1 class="text-3xl font-bold gradient-text mb-8">"Social Feed"</h1>
            
            <CreatePost on_post_created=handle_post_created />

            <Transition fallback=move || view! { <div class="text-center py-10">"Loading feed..."</div> }>
                {move || posts_resource.get().map(|res| match res {
                    Ok(posts) => if posts.is_empty() {
                        view! { <div class="text-center py-10 text-muted-foreground">"No posts yet. Be the first to share something!"</div> }.into_view()
                    } else {
                        view! {
                            <div class="space-y-6">
                                {posts.into_iter().map(|post| view! {
                                    <PostCard
                                        post=post
                                        on_post_updated=handle_refresh
                                        on_delete=handle_delete
                                    />
                                }).collect_view()}
                            </div>
                        }.into_view()
                    },
                    Err(_) => view! { <div class="text-center py-10 text-destructive">"Error loading feed. Please try again later."</div> }.into_view()
                })}
            </Transition>
        </div>
    }
}
