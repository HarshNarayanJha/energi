use upower_dbus::BatteryState;

#[derive(Clone, PartialEq, Debug)]
pub struct BatteryData {
    pub percentage: f64,
    pub state: BatteryState,
    pub health: f64,
    pub temperature: f64,
    pub rate: f64,
    pub time_to_empty: i64,
    pub time_to_full: i64,
    pub serial: String,
    pub vendor: String,
    pub model: String,
    pub voltage: f64,
    pub charge_cycles: i64,
    pub history_percentage: Vec<(u32, f64)>,
    pub history_rate: Vec<(u32, f64)>,
}
