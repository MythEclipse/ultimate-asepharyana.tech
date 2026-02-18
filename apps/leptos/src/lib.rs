use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod api;
pub mod components;
pub mod pages;
pub mod providers;
pub mod types;
pub mod app;

use crate::app::App;

#[component]
pub fn AppRoot() -> impl IntoView {
    provide_meta_context();
    view! {
        <App />
    }
}
