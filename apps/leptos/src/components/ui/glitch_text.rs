use leptos::*;

#[component]
pub fn GlitchText(
    #[prop(into)] text: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let text_clone1 = text.clone();
    let text_clone2 = text.clone();
    let text_clone3 = text.clone();
    
    view! {
        <span class=format!("glitch-heavy group relative inline-block {}", class) data-text=text_clone1>
            {text_clone2}
            <span class="glitch-layer" data-text=text_clone3></span>
            <span class="glitch-layer-2" data-text=text.clone()></span>
        </span>
    }
}
