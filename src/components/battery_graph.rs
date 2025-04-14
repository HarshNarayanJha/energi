use dioxus::prelude::*;
use dioxus_charts::LineChart;

#[component]
pub fn BatteryGraph() -> Element {
    rsx! {
        div { class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg xl:aspect-w-9 xl:aspect-h-8",
            LineChart {
                padding_top: 30,
                padding_left: 65,
                padding_right: 80,
                padding_bottom: 30,
                show_grid: true,
                show_dotted_grid: true,
                line_width: "0.1em",
                dot_size: "0.2em",
                viewbox_height: 250,
                class_grid: "stroke-gray-700 dark:stroke-gray-400",
                class_grid_label: "fill-gray-600 dark:fill-gray-400 text-[0.5em]",
                class_line: "stroke-2",
                class_line_dot: "fill-current",
                class_line_label: "fill-gray-600 dark:fill-gray-400 text-[0.5em]",
                label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                series: vec![
                    vec![
                        100.0,
                        97.0,
                        98.0,
                        80.0,
                        79.0,
                        78.7,
                        68.9,
                        68.0,
                        67.0,
                        66.0,
                        50.0,
                        49.0,
                        72.0,
                        14.0,
                        9.0,
                    ],
                ],
                labels: vec![
                    "09:00".into(),
                    "09:30".into(),
                    "10:00".into(),
                    "10:30".into(),
                    "11:00".into(),
                    "11:30".into(),
                    "12:00".into(),
                    "12:30".into(),
                    "13:00".into(),
                    "13:30".into(),
                    "14:00".into(),
                    "14:30".into(),
                    "15:00".into(),
                    "15:30".into(),
                    "16:00".into(),
                ],
            }
        }
    }
}
