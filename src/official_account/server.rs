use crate::core::message::ReceivedMessage;
use crate::config::WechatBase;
use bytes::Bytes;
use serde::{ Serialize, Deserialize };

pub struct Server {
    pub config: WechatBase,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    timestamp: i64,
    nonce: String,
    encrypt_type: Option<String>,
    msg_signature: Option<String>
}

impl Server {
    pub fn new(config: WechatBase) -> Self {
        Server {
            config,
        }
    }

/*     pub fn parse_input(bytes: Bytes) -> ReceivedMessage { */
//
    /* } */

}

