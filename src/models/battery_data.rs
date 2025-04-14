#[derive(Clone, PartialEq, Debug)]
pub struct BatteryData {
    pub percentage: f64,
    pub charging: bool,
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
    pub history_percentage: Vec<(f64, f32)>,
    pub history_rate: Vec<(f64, f32)>,
}
