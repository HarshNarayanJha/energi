use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::models::BatteryData;

#[component]
pub fn Home() -> Element {
    let mut battery_data = use_signal(|| BatteryData {
        percentage: 67.0,
        charging: true,
        health: 90.0,
        temperature: 25.0,
        rate: 8.56,
        time_remaining: 2.0,
        history_percentage: Vec::new(),
        history_rate: Vec::new(),
    });

    let charge = move |_| {
        tracing::info!("Pressed");
        let mut current = battery_data();
        current.percentage += 1.0;
        current.charging = !current.charging;
        current.time_remaining = 4.5;
        battery_data.set(current);
    };

    rsx! {
        div { id: "root", class: "select-none",
            h1 { "Energi" }
            p { "Current Charge Level: {battery_data:?}%" }
            p { "Battery Status: {battery_data.read().charging}" }
            p { "Estimated Time Remaining: {battery_data.read().time_remaining} hours" }
            // p { "Estimated Time To Full: {battery_data.time_to_full} hours" }
            p { "Battery Temperature: {battery_data.read().temperature}°C" }
            p { "Battery Health: {battery_data.read().health}%" }
            p { "Battery discharging rate: {battery_data.read().rate}W" }

            div { class: "mt-8",
                p { class: "text-center", "Graph for battery charge over time..." }
            }

            div { class: "mt-8",
                p { class: "text-center", "Graph for battery watt usage over time..." }
            }

            button { onclick: charge, "Charge My Battery" }
        }
    }
}
