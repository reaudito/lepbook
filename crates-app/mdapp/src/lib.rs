mod components;
mod router;

use leptos::*;
use router::RouterApp;

#[component]
pub fn App() -> impl IntoView {
    view! { <RouterApp/> }
}
