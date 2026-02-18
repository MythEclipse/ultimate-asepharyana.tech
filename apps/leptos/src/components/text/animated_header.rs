use leptos::*;

#[derive(Clone, Debug)]
pub struct WordItem {
    pub text: &'static str,
    pub class: &'static str,
}

#[component]
pub fn AnimatedHeader(
    words: Vec<WordItem>,
    #[prop(optional)] class: String,
    #[prop(optional)] cursor_class: String,
) -> impl IntoView {
    // Flatten words into characters with global index for staggered delay
    let chars: Vec<(char, String, usize)> = words
        .iter()
        .flat_map(|w| {
            w.text.chars().map(|c| (c, w.class.to_string())).collect::<Vec<_>>()
        })
        .enumerate()
        .map(|(i, (c, cls))| (c, cls, i))
        .collect();

    view! {
        <span class=format!("inline-block {}", class)>
            <span class="sr-only">
                {words.iter().map(|w| w.text).collect::<Vec<_>>().join(" ")}
            </span>
            <div class="inline" aria-hidden="true">
                {chars.into_iter().map(|(char, cls, index)| {
                    let delay = format!("transition-delay: {}ms", index * 100);
                    // Using animate-fade-in from tailwind config which essentially does opacity/transform check
                    // Ideally we'd use intersection observer here but for simplicity in port we assume visibility
                    // or use a simple CSS animation class
                    view! {
                         <span
                            class=format!("inline-block transition-opacity duration-300 ease-in-out opacity-0 animate-slide-up fill-mode-forwards {}", cls)
                            style=delay
                         >
                            {char.to_string()}
                         </span>
                    }
                }).collect_view()}
            </div>
            <span
                class=format!(
                    "inline-block rounded-sm w-[3px] h-7 bg-blue-500 animate-pulse ml-1 {}",
                    cursor_class
                )
                aria-hidden="true"
            />
        </span>
    }
}
