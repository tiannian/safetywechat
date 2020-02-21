use crate::config::WechatBase;
use bytes::Bytes;
use crate::core::message::{ Message, EncryptedMessage, ReceivedMessage, Query };
use crate::config::{ EncryptMode, MessageFormat };
use crate::Result;

pub struct Server<'a> {
    config: &'a WechatBase,
}

impl<'a> Server<'a> {
    pub fn new(config: &'a WechatBase) -> Self {
        Server {
            config,
        }
    }

    fn parse_xml_encrypted(&self, query: Query, data: String) -> Result<Message> {
        let encrypt_data: EncryptedMessage = quick_xml::de::from_str(&data)?;
        let message = encrypt_data.decrypt(query, &self.config)?;
        let received_data: ReceivedMessage = quick_xml::de::from_str(&message)?;
        received_data.get_message()
    }

    fn parse_json_encrypted(&self, query: Query, data: String) -> Result<Message> {
        let encrypt_data: EncryptedMessage = serde_json::from_str(&data)?;
        let message = encrypt_data.decrypt(query, &self.config)?;
        let received_data: ReceivedMessage = serde_json::from_str(&message)?;
        received_data.get_message()
    }

    fn parse_json(data: String) -> Result<Message> {
        let received_data: ReceivedMessage = serde_json::from_str(&data)?;
        received_data.get_message()
    }

    fn parse_xml(data: String) -> Result<Message> {
        let received_data: ReceivedMessage = quick_xml::de::from_str(&data)?;
        received_data.get_message()
    }

    pub fn input(&self, query: Query, bytes: Bytes) -> Result<Message> {
        let data = String::from_utf8(Vec::from(bytes.as_ref())).unwrap();
        match (&self.config.msg_type, &self.config.encrypt_mode) {
            (MessageFormat::XML,  EncryptMode::Encrypted)  => self.parse_xml_encrypted(query, data),
            (MessageFormat::Json, EncryptMode::Encrypted)  => self.parse_json_encrypted(query, data),
            (MessageFormat::XML,  EncryptMode::Hybrid)     => Self::parse_xml(data),
            (MessageFormat::Json, EncryptMode::Hybrid)     => Self::parse_json(data),
            (MessageFormat::XML,  EncryptMode::Plaintext)  => Self::parse_xml(data),
            (MessageFormat::Json, EncryptMode::Plaintext)  => Self::parse_json(data),
        }
    }
}



