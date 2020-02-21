use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Text(Text),
    Image(Image),
    Voice(Voice),
    Video(Video),
    ShortVideo(Video),
    Location(Location),
    Link(Link),
    Music(Music),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Image {
    pub url: String,
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voice {
    pub media_id: String,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub media_id: String,
    pub thumb_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub scale: u64,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Music {
    pub title: String,
    pub description: String,
    pub url: String,
    pub hq_url: String,
    pub thumb_id: String
}
