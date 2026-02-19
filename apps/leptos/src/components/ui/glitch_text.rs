use leptos::*;

#[component]
pub fn GlitchText(
    #[prop(into)] text: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    view! {
        <span class=format!("glitch-heavy {}", class) data-text=text.clone()>
            {text.clone()}
            <div class="glitch-layer whitespace-nowrap" data-text=text></div>
        </span>
    }
}
