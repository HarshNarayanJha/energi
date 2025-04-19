mod upower;

use upower::get_battery_data;
use upower::BatteryData;

#[tauri::command(async)]
async fn battery_data() -> Result<BatteryData, String> {
    return get_battery_data().await.map_err(|err| err.to_string());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![battery_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
