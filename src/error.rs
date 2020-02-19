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
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RequestError(e.to_string())
    }
}

