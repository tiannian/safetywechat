use crate::cache::Cache;
use crate::config::WechatBase;
use crate::core::accesstoken::AccessTokenIns;
use crate::Result;
use serde::{ Serialize, Deserialize };
use crate::error::Error;
use chrono::{ Utc };
use hex::ToHex;
use rand::{ thread_rng, RngCore };

#[derive(Serialize, Deserialize, Debug)]
pub struct JsapiTicketBody {
    pub errcode: Option<u32>,
    pub errmsg:  Option<String>,
    pub ticket:  Option<String>,
    pub expires_in: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JssdkConfig {
    debug: bool,
    #[serde(alias = "appId")]
    appid: String,
    timestamp: i64,
    #[serde(alias = "nonceStr")]
    nonce: String,
    signature: String,
    #[serde(alias = "jsApiList")]
    apis: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsapiTicket {
    pub ticket: String,
    pub expires_in: u32,
}

pub struct Jssdk<C: Cache> {
    pub ticket: Option<JsapiTicket>,
    cache: C,
    config: WechatBase,
    access_token: AccessTokenIns<C>,
}

impl<C: Cache> Jssdk<C> {
    pub fn new(cache: C, config: WechatBase, access_token: AccessTokenIns<C>) -> Self {
        Jssdk {
            ticket: None,
            cache,
            config,
            access_token,
        }
    }

    pub async fn update_ticket(&mut self) -> Result<JsapiTicket> {
        let token = self.access_token.get_token().await?;
        let url = format!("https://api.weixin.qq.com/cgi-bin/ticket/getticket?access_token={}&type=jsapi", token.access_token);
        let tk = reqwest::get(&url).await?
            .json::<JsapiTicketBody>().await?;
        let key = format!("jsapiticket-{}", self.config.app_id);
        if let Some(code) = tk.errcode {
            if code == 0 {
                let jtk = JsapiTicket {
                    ticket: tk.ticket.unwrap(),
                    expires_in: tk.expires_in.unwrap(),
                };
                self.cache.set(&key, jtk.clone()).await?;
                self.cache.ttl(&key, jtk.expires_in).await?;
                Ok(jtk)
            } else {
                Err(Error::JsapiTicketError(tk))
            }
        } else {
            Err(Error::JsapiTicketError(tk))
        }
    }

    pub async fn get_ticket(&mut self) -> Result<JsapiTicket> {
        let key = format!("jsapiticket-{}", self.config.app_id);
        match self.cache.get(&key).await? {
            Some(token) => Ok(token),
            None => self.update_ticket().await
        }
    }

    pub async fn build_config(&mut self, debug: bool, url: String, apis: Vec<String>) -> Result<JssdkConfig> {
        // get timestamp.
        let timestamp = Utc::now().timestamp();

        // get ramdon string.
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = nonce_bytes.encode_hex_upper::<String>();

        let ticket = self.get_ticket().await?;

        let data = format!("jsapi_ticket={}&noncestr={}&timestamp={}url={}",
                           ticket.ticket,
                           nonce,
                           timestamp,
                           url);
        println!("{}", data);

        let signature = sha1::Sha1::from(data).hexdigest();

        Ok(JssdkConfig {
            debug,
            appid: self.config.app_id.clone(),
            timestamp,
            nonce,
            signature,
            apis,
        })
    }
}

