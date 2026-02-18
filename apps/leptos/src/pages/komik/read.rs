use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::komik::fetch_chapter;
use urlencoding;

#[component]
pub fn ReadPage() -> impl IntoView {
    let params = use_params_map();
    // In web version, slug might be passed differently, but fetch_chapter awaits chapter_url
    // If the router path is /komik/read/:slug, we use slug directly.
    // If slug is URL encoded in param, we might need to decode or pass as is depending on backend expectation.
    // Based on `fetch_chapter` implementation, it encodes again, so we assume we get a raw slug here?
    // Actually, in SolidJS: href={`/komik/chapter/${encodeURIComponent(ch.chapter_id)}`}
    // So the param in URL is encoded. Leptos router might decode it automatically? 
    // Let's assume params.get("slug") returns the decoded string (or the string as is in URL).
    // Note: fetch_chapter does `urlencoding::encode(&slug)`. 
    // If we receive "chapter-1", it becomes "chapter-1". 
    // If we receive a full URL as ID, it might be tricky.
    
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    
    let chapter_data = create_resource(slug, |s| async move {
        if s.is_empty() { return None; }
        // Attempt to unescape if it was double encoded or just pass common slug
        // SolidJS passes `encodeURIComponent(chapter_id)`.
        // If chapter_id is a URL, it is encoded.
        // Leptos router should decode one level.
        fetch_chapter(s).await.ok()
    });

    // Control visibility (simplified from SolidJS version)
    let (show_controls, set_show_controls) = create_signal(true);

    view! {
        <main class="min-h-screen bg-black text-white pb-20">
            <Suspense fallback=move || view! { <div class="h-screen flex items-center justify-center">"Loading chapter..."</div> }>
                {move || chapter_data.get().flatten().map(|data| view! {
                    <Title text=format!("{} | Asepharyana", data.title)/>
                    
                    // Fixed Header
                    <div class=move || format!("fixed top-0 left-0 right-0 z-50 transition-transform duration-300 {}", if show_controls.get() { "translate-y-0" } else { "-translate-y-full" })>
                        <div class="bg-black/80 backdrop-blur-md border-b border-white/10 p-4">
                            <div class="container mx-auto flex items-center justify-between">
                                <a href=if !data.list_chapter.is_empty() { data.list_chapter.clone() } else { "/komik".to_string() } class="p-2 hover:bg-white/10 rounded-lg">
                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                    </svg>
                                </a>
                                <h1 class="font-bold text-sm md:text-base truncate px-4">{data.title.clone()}</h1>
                                <div class="w-8"></div> // spacer
                            </div>
                        </div>
                    </div>

                    // Images
                    <div class="container mx-auto max-w-3xl pt-20 pb-24 px-0 md:px-4"
                        on:click=move |_| set_show_controls.update(|v| *v = !*v)>
                        {data.images.iter().enumerate().map(|(i, img)| view! {
                            <div class="relative w-full mb-2">
                                <img src=img.clone() class="w-full h-auto" loading="lazy" alt=format!("Page {}", i + 1) />
                                <div class="absolute bottom-4 right-4 bg-black/50 text-white text-xs px-2 py-1 rounded-full backdrop-blur-sm">
                                    {i + 1}
                                </div>
                            </div>
                        }).collect_view()}
                    </div>

                    // Bottom Navigation
                    <div class=move || format!("fixed bottom-0 left-0 right-0 z-50 transition-transform duration-300 {}", if show_controls.get() { "translate-y-0" } else { "translate-y-full" })>
                        <div class="bg-black/80 backdrop-blur-md border-t border-white/10 p-4">
                            <div class="container mx-auto flex items-center justify-center gap-4">
                                {if !data.prev_chapter_id.is_empty() {
                                    view! {
                                        <a href=format!("/komik/read/{}", urlencoding::encode(&data.prev_chapter_id)) class="px-6 py-3 bg-white/10 hover:bg-white/20 rounded-xl transition-colors font-medium">
                                            "Previous"
                                        </a>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}

                                {if !data.next_chapter_id.is_empty() {
                                    view! {
                                        <a href=format!("/komik/read/{}", urlencoding::encode(&data.next_chapter_id)) class="px-6 py-3 bg-orange-500 hover:bg-orange-600 rounded-xl transition-colors font-medium text-white">
                                            "Next"
                                        </a>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        </div>
                    </div>

                    // End Chapter Marker
                    <div class="py-12 text-center text-white/50">
                        <p>"End of Chapter"</p>
                        <p class="text-sm mt-2">{data.title}</p>
                    </div>

                }).collect_view()}
            </Suspense>
        </main>
    }
}
