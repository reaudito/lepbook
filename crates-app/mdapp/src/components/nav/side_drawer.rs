use leptos::*;
use thaw::*;

#[component]
pub fn SideDrawer() -> impl IntoView {
    let show = create_rw_signal(false);
    let placement = create_rw_signal(DrawerPlacement::Left);

    let open = Callback::new(move |new_placement: DrawerPlacement| {
        // Note: Since `show` changes are made in real time,
        // please put it in front of `show.set(true)` when changing `placement`.
        placement.set(new_placement);
        show.set(true);
    });

    view! {
        <ButtonGroup>
            <Button on_click=Callback::new(move |_| leptos::Callable::call(&open, DrawerPlacement::Left))>"Left"</Button>
        </ButtonGroup>
        <Drawer title="Title" show placement>
            "Hello"
        </Drawer>
    }
}
