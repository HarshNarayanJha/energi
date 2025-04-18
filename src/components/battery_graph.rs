use dioxus::prelude::*;
use dioxus_charts::LineChart;

#[component]
pub fn BatteryGraph(values: Vec<f32>, labels: Vec<String>, value_format: fn(f32) -> String) -> Element {
    rsx! {
        div { class: "aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-lg xl:aspect-w-9 xl:aspect-h-8",
            LineChart {
                padding_top: 30,
                padding_left: 65,
                padding_right: 80,
                padding_bottom: 30,
                show_grid: true,
                show_dotted_grid: false,
                line_width: "0.05em",
                dot_size: "0.2em",
                viewbox_height: 250,
                class_grid: "stroke-gray-700 dark:stroke-gray-400",
                class_grid_label: "fill-gray-600 dark:fill-gray-400 text-[0.4em]",
                class_line: "stroke-2",
                class_line_dot: "fill-current",
                class_line_label: "fill-gray-600 dark:fill-gray-400 text-[0.2em]",
                label_interpolation: value_format,
                series: vec![values],
                labels,
            }
        }
    }
}
