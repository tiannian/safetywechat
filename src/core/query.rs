use serde::{ Serialize, Deserialize };
use crate::config::WechatBase;
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    pub signature: Option<String>,
    pub msg_signature: Option<String>,
    pub timestamp: Option<i64>,
    pub nonce: Option<String>,
    echostr: Option<String>,
    encrypt_type: Option<String>,
}

impl Query {
    // Consider handle lose key by Result.
    pub fn validate(self, config: &WechatBase) -> Option<String> {
        let nonce = self.nonce.unwrap();
        let timestamp = self.timestamp.unwrap();
        let echostr = self.echostr.unwrap();
        let signature = self.signature.unwrap();

        let mut tmp_array = vec![nonce, timestamp.to_string(), config.token.clone()];
        tmp_array.sort();
        let tmp_str = format!("{}{}{}",tmp_array[0], tmp_array[1], tmp_array[2]);
        let sign = sha1::Sha1::from(tmp_str).hexdigest();
        println!("{:?}", sign);
        if sign == signature {
            Some(echostr)
        } else {
            None
        }
    }
}

