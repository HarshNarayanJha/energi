use dioxus::prelude::*;
use dioxus_logger::tracing;

use super::ReadingTile;
use crate::models::get_battery_data;
use crate::models::BatteryData;

#[component]
pub fn Home() -> Element {
    let mut battery_data = use_signal(|| BatteryData {
        percentage: 100.0,
        charging: true,
        health: 100.0,
        temperature: 37.0,
        rate: 5.00,
        time_to_empty: 0,
        time_to_full: 0,
        serial: String::new(),
        vendor: String::new(),
        model: String::new(),
        voltage: 0.0,
        charge_cycles: 0,
        history_percentage: Vec::new(),
        history_rate: Vec::new(),
    });

    use_effect(move || {
        spawn(async move {
            let data = get_battery_data().await;
            if let Ok(data) = data {
                battery_data.set(data);
            }
        });
    });

    let refresh = move |_| {
        tracing::info!("Refreshed");
        spawn(async move {
            let data = get_battery_data().await;
            if let Ok(data) = data {
                battery_data.set(data);
            } else {
                tracing::error!("Failed to refresh battery data");
            }
        });
    };

    rsx! {
        div { id: "home", class: "select-none min-h-screen p-8",
            h1 { class: "text-4xl font-bold tracking-tight mb-2", "Energi" }
            p { class: "mb-8", "Quick stats about your battery" }

            div { id: "stats", class: "flex flex-row flex-wrap gap-4",
                ReadingTile {
                    label: "Charge Level:",
                    value: "{battery_data.read().percentage}%",
                }
                ReadingTile {
                    label: "Battery Status:",
                    value: "{battery_data.read().charging}",
                }
                ReadingTile {
                    label: "Estimated Time Remaining:",
                    value: "{battery_data.read().time_to_empty} minutes",
                }
                ReadingTile {
                    label: "Estimated Time To Full:",
                    value: "{battery_data.read().time_to_full} minutes",
                }
                ReadingTile {
                    label: "Battery Temperature:",
                    value: "{battery_data.read().temperature}°C",
                }
                ReadingTile {
                    label: "Battery Health:",
                    value: "{battery_data.read().health}%",
                }
                ReadingTile {
                    label: "Battery Discharging Rate:",
                    value: "{battery_data.read().rate}W",
                }
                ReadingTile {
                    label: "Voltage:",
                    value: "{battery_data.read().voltage}V",
                }
                // ReadingTile {
                //     label: "Charge Cycle:",
                //     value: "{battery_data.read().charge_cycles}",
                // }
                ReadingTile { label: "Model:", value: "{battery_data.read().model}" }
                ReadingTile {
                    label: "Battery Vendor:",
                    value: "{battery_data.read().vendor}",
                }
                ReadingTile {
                    label: "Serial Number:",
                    value: "{battery_data.read().serial}",
                }
            }


            // p { "Current Charge Level: {battery_data.read().percentage}%" }
            // p { "Battery Status: {battery_data.read().charging}" }
            // p { "Estimated Time Remaining: {battery_data.read().time_remaining} hours" }
            // p { "Estimated Time To Full: {battery_data.time_to_full} hours" }
            // p { "Battery Temperature: {battery_data.read().temperature}°C" }
            // p { "Battery Health: {battery_data.read().health}%" }
            // p { "Battery discharging rate: {battery_data.read().rate}W" }

            div { class: "mt-8 bg-gray-800 rounded-lg p-6",
                p { class: "text-center text-gray-300", "Graph for battery charge over time..." }
            }

            div { class: "mt-8 bg-gray-800 rounded-lg p-6",
                p { class: "text-center text-gray-300", "Graph for battery watt usage over time..." }
            }

            div { class: "mt-8 flex gap-4 justify-center",
                button {
                    class: "px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors",
                    onclick: refresh,
                    "Refresh"
                }
            }
        }
    }
}
