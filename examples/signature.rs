#[macro_use]
extern crate async_trait;

use warp::{ Filter };
use safetywechat::config::WechatBase;
use safetywechat::config::{ MessageFormat, EncryptMode };
use safetywechat::core::Query;
use safetywechat::official_account::OfficialAccount;
use safetywechat::cache::Cache;
use std::collections::HashMap;
use serde::{ Serialize, de::DeserializeOwned };
use safetywechat::Result;
use std::sync::Mutex;
use std::sync::Arc;

struct TestCache {
    inner: Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>
}

impl Default for TestCache {
    fn default() -> Self {
        TestCache {
            inner: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl Clone for TestCache {
    fn clone(&self) -> Self {
        TestCache {
            inner: self.inner.clone()
        }
    }
}

#[async_trait]
impl Cache for TestCache {
    async fn get<K, V>(&self, key: &K) -> Result<Option<V>> where K: AsRef<[u8]> + Sync, V: DeserializeOwned + Sync {
        let inner = self.inner.lock().unwrap();
        let r = inner.get(&Vec::from(key.as_ref()));
        match r {
            Some(data) => {
                let v: V = serde_json::from_slice(data).unwrap();
                Ok(Some(v))
            },
            None => Ok(None)
        }
    }
    async fn set<K, V>(&self, key: &K, value: V) -> Result<()> where K: AsRef<[u8]> + Sync, V: Serialize + Send {
        let data = serde_json::to_vec(&value).unwrap();
        let mut inner = self.inner.lock().unwrap();
        inner.insert(Vec::from(key.as_ref()), data);
        Ok(())
    }

    async fn ttl<K>(&self, _key: &K, _ttl: u32) -> Result<()> where K: AsRef<[u8]> + Sync {
        Ok(())
    }

    async fn delete<K>(&self, key: &K) -> Result<()> where K: AsRef<[u8]> + Sync {
        let mut inner = self.inner.lock().unwrap();
        inner.remove(&Vec::from(key.as_ref()));
        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.clear();
        Ok(())
    }

    async fn has<K> (&self, key: &K) -> Result<bool> where K: AsRef<[u8]> + Sync {
        Ok(self.inner.lock().unwrap().contains_key(&Vec::from(key.as_ref())))
    }
}

#[tokio::main]
async fn main() {
    let config = WechatBase {
        app_id: String::from("wx5a3dbaf21ec95f39"),
        secret: String::from("4c3e8320e3ea6b5b3a1d4878b40664d7"),
        token: String::from("abcdefgh"),
        aes_key: Some(String::from("2Ytb9xgNNs72AvD3W60iY3qFX9w4qgKWurto47l2Kfw")),
        msg_type: MessageFormat::XML,
        encrypt_mode: EncryptMode::Plaintext,
    };

    let cache = TestCache::default();
    let oa = OfficialAccount::new(cache, config);
    let server1 = oa.server();
    let server2 = oa.server();
    let signature = warp::query::query::<Query>()
        .map(move |query: Query| {
            server1.validate(query).unwrap()
        });

    let message = warp::body::content_length_limit(1024 * 32)
        .and(warp::query::query())
        .and(warp::body::bytes())
        .map(move |query: Query, bytes: bytes::Bytes| {
            let da = server2.input(query, bytes);
            println!("{:?}", da);
            String::new()
        });

    let wechat = warp::path!("wechat").and(
        warp::get().and(signature)
        .or(warp::post().and(message))
    );


    warp::serve(wechat)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

