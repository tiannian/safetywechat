use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Image {
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Video {
    pub media_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Music {
    pub title: String,
    pub description: String,
    pub url: String,
    pub hq_url: String,
    pub thumb_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct New {
    pub title: String,
    pub description: String,
    pub pic_url: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Helper {
    pub to: String,
    pub from: String,
    pub timestamp: i64,
    pub t: String,
    pub content: Option<String>,

    pub image: Option<Image>,

    pub video: Option<Video>,

    pub music: Option<Music>,

    pub news: Option<Vec<New>>,

    pub count: Option<u64>,

}

