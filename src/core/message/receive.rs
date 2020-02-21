use serde::{ Serialize, Deserialize };

use crate::error::Error;
use crate::Result;
use crate::core::message::{ Message, Text, Image, Voice, Video, Location, Link };

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivedMessage {
    #[serde(alias = "FromUserName")]
    from: String,
    #[serde(alias = "ToUserName")]
    to: String,
    #[serde(alias = "CreateTime")]
    timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "MsgType")]
    t: String,

    #[serde(alias = "Content")]
    #[serde(alias = "Label")]
    #[serde(alias = "Description")]
    text: Option<String>,

    #[serde(alias = "PicUrl")]
    #[serde(alias = "Url")]
    url: Option<String>,

    #[serde(alias = "MediaId")]
    media_id: Option<String>,

    #[serde(alias = "Title")]
    #[serde(alias = "Format")]
    format: Option<String>,

    #[serde(alias = "ThumbMediaId")]
    thumb_id: Option<String>,

    #[serde(alias = "Location_X")]
    x: Option<f64>,

    #[serde(alias = "Location_Y")]
    y: Option<f64>,

    #[serde(alias = "Scale")]
    scale: Option<u64>,

    #[serde(default = "default_vx")]
    _v1: Option<String>,

    #[serde(default = "default_vx")]
    _v2: Option<String>,
}

fn default_vx() -> Option<String> {
    Some(String::default())
}

macro_rules! message_body_convert {
    ( $s:expr, $e:ident, $t:ident, $( $x:ident => $y:ident ), * ) => {
        {
            if true
                $(
                    && $s.$y.is_some()
                 )+ {
                    Ok( Message::$e( $t {
                        $(
                            $x: $s.$y.unwrap(),
                            )*
                    } ) )
                } else {
                    let info = String::from("lose field(s): ")
                        + $( &format!("{} => {:?}; ", stringify!($x), $s.$y) + )* "";
                    Err(Error::MessageKeyError(info))
                }
        }
    }
}

impl ReceivedMessage {
    pub fn get_message(self) -> Result<Message> {
        match self.t.as_str() {
            "text" => message_body_convert!(self, Text, Text, text => text),
            "image" => message_body_convert!(self, Image, Image, url => url, media_id => media_id),
            "voice" => message_body_convert!(self, Voice, Voice, format => format, media_id => media_id),
            "video" => message_body_convert!(self, Video, Video, thumb_id => thumb_id, media_id => media_id, title => _v1, description => _v2),
            "shortvideo" => message_body_convert!(self, ShortVideo, Video, thumb_id => thumb_id, media_id => media_id, title => _v1, description => _v2),
            "location" => message_body_convert!(self, Location, Location, x => x, y => y, scale=>scale, label => text),
            "link" => message_body_convert!(self, Link, Link, title => format, description => text, url => url),
            _ => Err(Error::UnknownMessageType(self.t)),
        }
    }
}


#[cfg(test)]
mod tests {
    use quick_xml::de::{from_str, DeError};
    use super::ReceivedMessage;
    #[test]
    fn test_text() -> Result<(), DeError> {
        let text = "
        <xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[fromUser]]></FromUserName>
            <CreateTime>1348831860</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[this is a test]]></Content>
            <MsgId>1234567890123456</MsgId>
        </xml>";
        let r_text: ReceivedMessage = from_str(text)?;
        println!("Text: {:?}", r_text.get_message());

        let image = "
        <xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[fromUser]]></FromUserName>
            <CreateTime>1348831860</CreateTime>
            <MsgType><![CDATA[image]]></MsgType>
            <PicUrl><![CDATA[this is a url]]></PicUrl>
            <MediaId><![CDATA[media_id]]></MediaId>
            <MsgId>1234567890123456</MsgId>
        </xml>";
        let r_image: ReceivedMessage = from_str(image)?;
        println!("Image: {:?}", r_image.get_message());

        let voice = "<xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[fromUser]]></FromUserName>
            <CreateTime>1357290913</CreateTime>
            <MsgType><![CDATA[voice]]></MsgType>
            <MediaId><![CDATA[media_id]]></MediaId>
            <Format><![CDATA[Format]]></Format>
            <MsgId>1234567890123456</MsgId>
        </xml>";
        let r_voice: MessageBody = from_str(voice)?;
        println!("Voice: {:?}", r_voice.get_message());
        Ok(())
    }
}

