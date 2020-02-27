use serde::{ Serialize, Deserialize };
use crate::core::message::Message;
use crate::Result;
use crate::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum SenderMessage {
    Image {
        #[serde(rename = "MediaId")]
        media_id: String,
    },

    Voice {
        #[serde(rename = "MediaId")]
        media_id: String,
    },

    Video {
        #[serde(rename = "MediaId")]
        media_id: String,
        #[serde(rename = "Title")]
        title: String,
        #[serde(rename = "Description")]
        description: String,
    },

    Music {
        #[serde(rename = "Title")]
        title: String,
        #[serde(rename = "Description")]
        description: String,
        #[serde(rename = "MusicUrl")]
        url: String,
        #[serde(rename = "HQMusicUrl")]
        hq_url: String,
        #[serde(rename = "ThumbMediaId")]
        thumb_id: String,
    },

    News {
        #[serde(rename = "item")]
        news: Vec<New>,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct New {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "PicUrl")]
    pub pic_url: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlInner {
    pub xml: OfficialAccountMessageHelper,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfficialAccountMessageHelper {
    #[serde(rename = "ToUserName")]
    pub to: String,

    #[serde(rename = "FromUserName")]
    pub from: String,

    #[serde(rename = "CreateTime")]
    pub timestamp: i64,

    #[serde(rename = "MsgType")]
    pub t: String,

    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<SenderMessage>,

    #[serde(rename = "ArticleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

impl OfficialAccountMessageHelper {
    pub fn new(to: String, from: String, timestamp: i64, message: Message) -> Result<Self> {
        let mut result = OfficialAccountMessageHelper {
            to, from, timestamp, t: String::new(), content: None, count: None, message: None,
        };

        match message {
            Message::Text { text } => { result.t = String::from("text"); result.content = Some(text) },
            Message::Image { url: _, media_id } => {
                result.t = String::from("image");
                result.message = Some(SenderMessage::Image{ media_id });
            },
            Message::Voice { media_id, format: _ } => {
                result.t = String::from("voice");
                result.message = Some(SenderMessage::Voice{ media_id });
            },
            Message::Video { media_id, title, description, thumb_id: _ } => {
                result.t = String::from("video");
                result.message = Some(SenderMessage::Video {
                    media_id, title, description
                })
            },
            Message::Music { title, description, url, hq_url, thumb_id } => {
                result.t = String::from("music");
                result.message = Some(SenderMessage::Music {
                    title, description, url, hq_url, thumb_id,
                })
            },
            Message::News { count, news } => {
                result.t = String::from("news");
                result.count = Some(count);
                let mut v = Vec::new();
                for n in news {
                    v.push(New {
                        title: n.title,
                        description: n.description,
                        pic_url: n.pic_url,
                        url: n.url,
                    })
                }
                let msg = SenderMessage::News {
                    news: v
                };
                result.message = Some(msg);
            }
            _ => { return Err(Error::UnsupportMessageType); },
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::OfficialAccountMessageHelper;
    use super::Message;

    #[test]
    fn test_helper_of_official_platform() ->serde_json::Result<()> {
        let message = Message::Image{ url: String::from(""), media_id: String::from("0x1527835128735621873127841284") };
        let r = OfficialAccountMessageHelper::new(String::from("to"), String::from("from"), 123, message).unwrap();
        let s = serde_json::to_string_pretty(&r)?;
        let xml = quick_xml::se::to_string(&r).unwrap();
        println!("{:?}", r);
        println!("{}", s);
        println!("{}", xml);
        Ok(())
    }
}


