use dioxus::prelude::*;

#[component]
pub fn ReadingTile(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center p-6 rounded-lg shadow-md bg-white dark:bg-gray-800 dark:text-white transition-colors",
            h2 { class: "text-xl font-bold tracking-tight mb-2 text-gray-900 dark:text-white",
                "{label}"
            }
            p { class: "text-gray-600 dark:text-gray-300", "{value}" }
        }
    }
}
