use dioxus::{
    desktop::{tao::window::Theme, window},
    prelude::*,
};

use components::Home;
mod components;
mod models;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(|| {
        let window = window();
        window.set_title("Energi");
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { id: "root", class: "w-full min-h-svh p-8", Home {} }
    }
}
