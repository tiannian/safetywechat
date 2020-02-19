use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Voice {
    media_id: String,
    format: String,
}
