use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    media_id: String,
    thumb_id: String,
}
