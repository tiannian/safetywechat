use crate::config::WechatBase;
use bytes::Bytes;
use crate::core::message::{ Message, EncryptedMessage, ReceivedMessage };
use crate::config::{ EncryptMode, MessageFormat };
use crate::Result;
use crate::core::Query;
use crate::config::PlatformType;

#[derive(Clone)]
pub struct Server {
    config: WechatBase,
    message: Message,
}

/* struct SendMessageHelper { */
    // to: String,
    // from: String,
    // timestamp: i64,
    // t: String,
    // content: Option<String>,
    // count: Option<u64>,
/* } */

impl Server {
    pub fn new(config: WechatBase) -> Self {
        Server {
            config,
            message: Message::Empty,
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

    fn parse_json(&self, _query: Query, data: String) -> Result<Message> {
        let received_data: ReceivedMessage = serde_json::from_str(&data)?;
        received_data.get_message()
    }

    fn parse_xml(&self, _query: Query, data: String) -> Result<Message> {
        let received_data: ReceivedMessage = quick_xml::de::from_str(&data)?;
        received_data.get_message()
    }

    pub fn input(&mut self, query: Query, bytes: Bytes) -> Result<&Message> {
        macro_rules! input_match {
            ($self:expr, $func:ident, $query:expr, $data:expr) => {
                {
                    $self.message = $self.$func($query, $data)?;
                }
            };
        }
        let data = String::from_utf8(Vec::from(bytes.as_ref())).unwrap();
        match (&self.config.msg_type, &self.config.encrypt_mode) {
            (MessageFormat::XML, EncryptMode::Encrypted)   => input_match!(self, parse_xml_encrypted,  query, data),
            (MessageFormat::Json, EncryptMode::Encrypted)  => input_match!(self, parse_json_encrypted, query, data),
            (MessageFormat::XML,  EncryptMode::Hybrid)     => input_match!(self, parse_xml_encrypted,  query, data),
            (MessageFormat::Json, EncryptMode::Hybrid)     => input_match!(self, parse_json_encrypted, query, data),
            (MessageFormat::XML,  EncryptMode::Plaintext)  => input_match!(self, parse_xml,  query, data),
            (MessageFormat::Json, EncryptMode::Plaintext)  => input_match!(self, parse_json, query, data),
        }
        Ok(&self.message)
    }

    pub fn validate(&self, query: Query) -> Result<String> {
        match query.validate(&self.config) {
            Some(non) => Ok(non),
            None => Ok(String::new())
        }
    }

    fn output_official_account(&self, message: Message) -> Result<String> {
        match message {
            Message::Empty => Ok(String::new()),
            _ => Ok(String::new())
        }
    }

    pub fn output(self, message: Message) -> Result<String> {
        match self.config.t {
            PlatformType::OfficialAccount => self.output_official_account(message),
            _ => Ok(String::new())
        }
    }
}



