use leptos::*;
use crate::api::proxy::audit_image_cache;
use crate::providers::use_ws;
use crate::api::types::WsMessage;

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
    let (active_src, set_active_src) = create_signal(src.clone());
    let (audited, set_audited) = create_signal(false);

    let ws = use_ws();
    
    // Listen for global image repair events
    let src_for_refresh = src.clone();
    create_effect(move |_| {
        if let Some(msg) = ws.last_message.get() {
            if let WsMessage::ImageRepaired { original_url, cdn_url } = msg {
                if original_url == src_for_refresh {
                    log::info!("Real-time refresh triggered for: {}", src_for_refresh);
                    set_active_src.set(cdn_url);
                    set_error.set(false);
                    set_loaded.set(false); // Retrigger animation
                }
            }
        }
    });

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
                <div class=format!("absolute inset-0 {} flex flex-col items-center justify-center gap-2 text-center", fallback_cls_err)>
                    <span class="text-2xl opacity-40">{"🖼️"}</span>
                    <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">{"Unavailable"}</p>
                </div>
            </Show>

            <img
                src=active_src
                alt=alt
                class=move || format!(
                    "{} transition-opacity duration-500 ease-out transform-gpu {} {}",
                    cls,
                    if !loaded.get() || error.get() { "opacity-0 scale-105 blur-sm" } else { "opacity-100 scale-100 blur-0" },
                    if error.get() { "grayscale" } else { "" }
                )
                loading=load_mode
                on:load=move |_| set_loaded.set(true)
                on:error=move |_| {
                    set_error.set(true);
                    
                    // Only audit once per component instance to avoid loops
                    if !audited.get() {
                        set_audited.set(true);
                        let url = src.clone();
                        
                        spawn_local(async move {
                            log::info!("Image failed to load, auditing: {}", url);
                            match audit_image_cache(url.clone()).await {
                                Ok(res) => {
                                    log::info!("Audit successful: {}", res.message);
                                    if res.re_uploaded {
                                        // Reset error and try reloading
                                        set_error.set(false);
                                        // Force reload by appending a cache buster if needed, 
                                        // but usually the CDN URL might change or same URL is now valid.
                                        if let Some(new_url) = res.cdn_url {
                                            set_active_src.set(new_url);
                                        }
                                    }
                                }
                                Err(e) => log::error!("Audit failed: {}", e),
                            }
                        });
                    }
                }
            />
        </div>
    }
}
