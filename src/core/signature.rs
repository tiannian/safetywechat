use serde::{ Serialize, Deserialize };
use crate::config::OfficialAccount;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signature {
    signature: String,
    timestamp: String,
    nonce: String,
    echostr: String,
}

impl Signature {
    pub fn validate(self, config: OfficialAccount) -> Option<String> {
        let mut tmp_array = vec![self.nonce, self.timestamp, config.token];
        tmp_array.sort();
        let tmp_str = format!("{}{}{}",tmp_array[0], tmp_array[1], tmp_array[2]);
        let sign = sha1::Sha1::from(tmp_str).hexdigest();
        println!("{:?}", sign);
        if sign == self.signature {
            Some(self.echostr)
        } else {
            None
        }
    }
}

