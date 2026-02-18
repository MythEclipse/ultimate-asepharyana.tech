use leptos::*;
use crate::types::Comment;

#[component]
pub fn CommentSection(
    #[prop(into)]
    post_id: MaybeSignal<String>,
    #[prop(into)]
    comments: MaybeSignal<Vec<Comment>>,
    // on_comment_added: Callback<Comment>, // callbacks are tricky in Leptos 0.6 sometimes, using basic closure or omit for now
) -> impl IntoView {
    let _ = post_id;
    let comments_len = comments.clone();
    let comments_list = comments.clone();
    view! {
        <div class="mt-4 pt-4 border-t border-border/50">
            <h4 class="text-sm font-semibold mb-2">"Comments (" {move || comments_len.get().len()} ")"</h4>
            <ul class="space-y-2">
                {move || comments_list.get().into_iter().map(|comment| view! {
                    <li class="text-sm">
                        <span class="font-bold mr-2">{comment.user.map(|u| u.name).unwrap_or("Unknown".to_string())}</span>
                        <span class="text-foreground/80">{comment.content}</span>
                    </li>
                }).collect_view()}
            </ul>
            // Add comment form here
        </div>
    }
}
