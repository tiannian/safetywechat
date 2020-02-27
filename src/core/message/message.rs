use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Empty,

    Text {
        text: String,
    },

    Image {
        url: String,
        media_id: String,
    },

    Voice {
        media_id: String,
        format: String,
    },

    Video {
        media_id: String,
        thumb_id: String,
        title: String,
        description: String,
    },

    ShortVideo {
        media_id: String,
        thumb_id: String,
        title: String,
        description: String,
    },

    Location {
        x: f64,
        y: f64,
        scale: u64,
        label: String,
    },

    Link {
        title: String,
        description: String,
        url: String,
    },

    Music {
        title: String,
        description: String,
        url: String,
        hq_url: String,
        thumb_id: String
    },

    News {
        count: u64,
        news: Vec<New>,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct New {
    pub title: String,
    pub description: String,
    pub pic_url: String,
    pub url: String,
}

