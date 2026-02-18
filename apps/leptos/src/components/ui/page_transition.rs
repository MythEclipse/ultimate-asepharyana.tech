use leptos::*;
use leptos_router::use_location;

#[component]
pub fn PageTransition(children: Children) -> impl IntoView {
    let location = use_location();
    let (_current_path, set_current_path) = create_signal(location.pathname.clone());

    // update when path changes so children remount/animate
    create_effect(move |_| {
        set_current_path.set(location.pathname.clone());
    });

    view! {
        <div class="w-full">
            <div class="w-full animate-fade-in transition-all duration-300 ease-out">{children()}</div>
        </div>
    }
}
