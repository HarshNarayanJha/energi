use dioxus::{desktop::window, prelude::*};

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

        div {
            id: "footer",
            class: "w-full bg-gray-100 dark:bg-gray-800 p-4 text-center border-t border-gray-200 dark:border-gray-700",
            div { class: "text-sm text-gray-600 dark:text-gray-400",
                "Copyright \u{00A9} "
                " Harsh Narayan Jha. See Energi on "
                a {
                    href: "https://github.com/HarshNarayanJha/energi",
                    class: "text-blue-500 hover:text-blue-700",
                    "GitHub"
                }
                "."
            }
        }
    }
}
