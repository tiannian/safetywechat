use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Location {
    x: f64,
    y: f32,
    scale: u64,
    label: String,
}
