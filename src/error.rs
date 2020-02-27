use std::convert::From;
use std::string::ToString;
use crate::core::accesstoken::AccessTokenBody;
use crate::official_account::jssdk::JsapiTicketBody;

#[derive(Debug)]
pub enum Error {
    RequestError(String),
    AccessTokenError(AccessTokenBody),
    CacheError(String),
    JsapiTicketError(JsapiTicketBody),
    MessageKeyError(String),
    UnknownMessageType(String),
    SignatureError,
    Base64DecodeError(base64::DecodeError),
    CryptoAESBlockError(block_modes::BlockModeError),
    XMLParseError(quick_xml::DeError),
    JsonParseError(serde_json::Error),
    UnsupportMessageType,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RequestError(e.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Base64DecodeError(e)
    }
}

impl From<block_modes::BlockModeError> for Error {
    fn from(e: block_modes::BlockModeError) -> Self {
        Error::CryptoAESBlockError(e)
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(e: quick_xml::DeError) -> Self {
        Error::XMLParseError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonParseError(e)
    }
}

