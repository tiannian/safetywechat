use std::string::String;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageFormat {
    XML,
    Json,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EncryptMode {
    Plaintext,
    Hybrid,
    Encrypted
}

impl Default for MessageFormat {
    fn default() -> Self {
        MessageFormat::XML
    }
}

impl Default for EncryptMode {
    fn default() -> Self {
        EncryptMode::Plaintext
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlatformType {
    OfficialAccount,
    OpenPlatfrom,
    MiniProgram,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatBase{
    pub app_id:  String,
    pub secret:  String,
    pub token:   String,
    pub aes_key: Option<String>,
    pub t: PlatformType,
    #[serde(default)]
    pub msg_type: MessageFormat,
    #[serde(default)]
    pub encrypt_mode: EncryptMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    configs: Vec<WechatBase>,
}
