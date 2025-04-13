#[derive(Clone, PartialEq, Debug)]
pub struct BatteryData {
    pub percentage: f32,
    pub charging: bool,
    pub health: f32,
    pub temperature: f32,
    pub rate: f32,
    pub time_remaining: f32,
    pub history_percentage: Vec<(f64, f32)>,
    pub history_rate: Vec<(f64, f32)>,
}
