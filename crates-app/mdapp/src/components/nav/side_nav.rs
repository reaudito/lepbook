use leptos::*;
#[component]
pub fn SideNav() -> impl IntoView {
    view! {
        <>
        <input type="checkbox" id="sidebar-toggle-anchor" />

        <nav id="sidebar" class="sidebar" aria-label="Table of contents">
            <div class="sidebar-scrollbox">
            <p>Hello</p>
            </div>
            <div id="sidebar-resize-handle" class="sidebar-resize-handle"></div>
        </nav>
       </>
    }
}
