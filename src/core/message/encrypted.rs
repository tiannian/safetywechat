use serde::{ Serialize, Deserialize };
use crate::core::message::ReceivedMessage;
use crate::Result;
use crate::error::Error;
use crate::config::WechatBase;
use std::string::ToString;
use block_modes::{ BlockMode, Cbc };
use block_modes::block_padding::Pkcs7;
use aes_soft::Aes256;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    timestamp: i64,
    nonce: String,
    encrypt_type: Option<String>,
    msg_signature: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedMessage {
    #[serde(rename = "ToUserName")]
    to: Option<String>,

    #[serde(rename = "Encrypt")]
    data: String,

    #[serde(rename = "MsgSignature")]
    signature: Option<String>,

    #[serde(rename = "TimeStamp")]
    timestamp: Option<i64>,

    #[serde(rename = "Nonce")]
    nonce: Option<String>,
}

impl EncryptedMessage {
    fn validate_signature(&self, signature: String, data: String, token: String, timestamp: String, nonce: String) -> Result<()> {
        let mut tmp = vec![data, token, timestamp, nonce];
        tmp.sort();
        let data_str = format!("{}{}{}{}", tmp[0], tmp[1], tmp[2], tmp[3]);
        let computed_signature = sha1::Sha1::from(data_str).hexdigest();
        if computed_signature == signature {
            // 计算成功
            Ok(())
        } else {
            Err(Error::SignatureError)
        }
    }

    fn decrypt_data(&self, data: &String, key: String, appid_len: usize) -> Result<ReceivedMessage> {
        let mut base64_data = base64::decode(data)?;
        let bin_key = base64::decode(&format!("{}=", key)).unwrap();
        let cipher = Aes256Cbc::new_var(&bin_key, &bin_key[..16]).unwrap();
        let decrypt_data = cipher.decrypt(&mut base64_data)?;

        // TODO: May return error message.
        let xml_data = String::from_utf8(Vec::from(&decrypt_data[20..decrypt_data.len() - appid_len])).unwrap();
        let message = quick_xml::de::from_str(&xml_data)?;
        Ok(message)
    }

    /// Decrypt data. If aes_key error, it will panic.
    pub fn decrypt(&self, query: Query, config: WechatBase) -> Result<ReceivedMessage> {
        if query.msg_signature.is_some() {
            let signature = query.msg_signature.unwrap();
            self.validate_signature(signature, self.data.clone(), config.token, query.timestamp.to_string(), query.nonce)?;
            self.decrypt_data(&self.data, config.aes_key.unwrap(), config.app_id.len())
        } else {
            Err(Error::MessageKeyError("lose `msg_signature`".to_string()))
        }
    }
}

