use std::string::String;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatBase{
    pub app_id:  String,
    pub secret:  String,
    pub token:   String,
    pub aes_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    offical_account: Vec<WechatBase>,
}
