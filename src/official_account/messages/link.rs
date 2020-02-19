use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Link {
    title: String,
    description: String,
    url: String,
}

