use dioxus::prelude::*;
use dioxus_logger::tracing;
use upower_dbus::BatteryState;

use super::BatteryGraph;
use super::BatteryGraphRecharts;
use super::ReadingTile;
use crate::models::get_battery_data;
use crate::models::BatteryData;

#[component]
pub fn Home() -> Element {
    let mut battery_data = use_signal(|| BatteryData {
        percentage: 100.0,
        state: BatteryState::Discharging,
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

    fn format_battery_state(state: BatteryState) -> String {
        match state {
            BatteryState::Unknown => "Unknown".to_string(),
            BatteryState::Charging => "Charging".to_string(),
            BatteryState::Discharging => "Discharging".to_string(),
            BatteryState::Empty => "Empty".to_string(),
            BatteryState::FullyCharged => "Full Charged".to_string(),
            BatteryState::PendingCharge => "Pending Charge".to_string(),
            BatteryState::PendingDischarge => "Pending Discharge".to_string(),
        }
    }

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
                    value: "{format_battery_state(battery_data.read().state)}",
                }
                match battery_data.read().state {
                    BatteryState::Discharging => {
                        rsx! {
                            ReadingTile {
                                label: "Estimated Time Remaining:",
                                value: "{battery_data.read().time_to_empty} seconds",
                            }
                        }
                    }
                    BatteryState::Charging => {
                        rsx! {
                            ReadingTile {
                                label: "Estimated Time To Full:",
                                value: "{battery_data.read().time_to_full} seconds",
                            }
                        }
                    }
                    _ => rsx! {},
                }
                // ReadingTile {
                //     label: "Battery Temperature:",
                //     value: "{battery_data.read().temperature}°C",
                // }
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

            div { class: "mt-8 rounded-lg p-6",
                h2 { class: "text-2xl font-bold ps-4 mb-8 dark:text-gray-100",
                    "Battery Percentage Graph"
                }

                BatteryGraph {
                    labels: battery_data
                        .read()
                        .history_percentage
                        .iter()
                        .map(|(time, _value)| {
                            format!(
                                "{}",
                                chrono::DateTime::from_timestamp((*time).into(), 0)
                                    .unwrap()
                                    .format("%b %e %H:%M:%S"),
                            )
                        })
                        .collect(),
                    values: battery_data
                        .read()
                        .history_percentage
                        .iter()
                        .map(|(_time, value)| *value as f32)
                        .collect(),
                    value_format: |v| format!("{:.2}%", v),
                }
            }

            div { class: "mt-8 rounded-lg p-6",
                h2 { class: "text-2xl font-bold ps-4 mb-8 dark:text-gray-100",
                    "Battery Energy Consumption Graph"
                }

                BatteryGraph {
                    labels: battery_data
                        .read()
                        .history_rate
                        .iter()
                        .map(|(time, _value)| {
                            format!(
                                "{}",
                                chrono::DateTime::from_timestamp((*time).into(), 0)
                                    .unwrap()
                                    .format("%b %e %H:%M:%S"),
                            )
                        })
                        .collect(),
                    values: battery_data
                        .read()
                        .history_rate
                        .iter()
                        .map(|(_time, value)| *value as f32)
                        .collect(),
                    value_format: |v| format!("{:.2}W", v),
                }
            }

            div { class: "mt-8 flex gap-4 justify-center",
                button {
                    class: "px-4 py-2 bg-gray-300 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 rounded-lg transition-colors dark:text-gray-100",
                    onclick: refresh,
                    "Refresh"
                }
            }

            BatteryGraphRecharts {
                timestamps: battery_data
                    .read()
                    .history_percentage
                    .iter()
                    .map(|(time, _value)| { (*time as u64) * 1000u64 })
                    .collect(),
                values: battery_data
                    .read()
                    .history_percentage
                    .iter()
                    .map(|(_time, value)| *value as f32)
                    .collect(),
                value_format: "%",
            }
        }
    }
}
