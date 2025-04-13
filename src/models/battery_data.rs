#[derive(Clone, PartialEq, Debug)]
pub struct BatteryData {
    pub percentage: f64,
    pub charging: bool,
    pub health: f64,
    pub temperature: f64,
    pub rate: f64,
    pub time_remaining: i64,
    pub history_percentage: Vec<(f64, f32)>,
    pub history_rate: Vec<(f64, f32)>,
}
