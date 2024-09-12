use crate::components::nav::side_drawer::SideDrawer;
// use crate::components::nav::side_nav::SideNav;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <>
            <SideDrawer />
            <p>"Hello World"</p>
            <p>"Rustic"</p>
        </>
    }
}
