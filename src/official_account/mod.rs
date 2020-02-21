pub mod jssdk;
mod modules;
mod official_account;

pub use jssdk::Jssdk;
pub use modules::Modules;
pub use official_account::OfficialAccount;

/* #[cfg(test)] */
// mod tests {
//     use crate::config;
//     use crate::cache::Cache;
//     use std::collections::HashMap;
//     use serde::{ Serialize, de::DeserializeOwned };
//     use crate::error::Result;
//     use std::sync::Arc;
//     use super::OfficialAccount;
//
//     struct TestCache {
//         inner: Arc<HashMap<Vec<u8>, Vec<u8>>>
//     }
//
//     impl Default for TestCache {
//         fn default() -> Self {
//             TestCache {
//                 inner: Arc::new(HashMap::new())
//             }
//         }
//     }
//
//     impl Clone for TestCache {
//         fn clone(&self) -> Self {
//             TestCache {
//                 inner: self.inner.clone()
//             }
//         }
//     }
//
//     #[async_trait]
//     impl Cache for TestCache {
//         async fn get<K, V>(&self, key: &K) -> Result<Option<V>> where K: AsRef<[u8]> + Sync, V: DeserializeOwned + Sync {
//             let r = self.inner.get(&Vec::from(key.as_ref()));
//             match r {
//                 Some(data) => {
//                     let v: V = serde_json::from_slice(data).unwrap();
//                     Ok(Some(v))
//                 },
//                 None => Ok(None)
//             }
//         }
//         async fn set<K, V>(&mut self, key: &K, value: V) -> Result<()> where K: AsRef<[u8]> + Sync, V: Serialize + Send {
//             let data = serde_json::to_vec(&value).unwrap();
//             let inner = Arc::make_mut(&mut self.inner);
//             inner.insert(Vec::from(key.as_ref()), data);
//             Ok(())
//         }
//
//         async fn ttl<K>(&mut self, _key: &K, _ttl: u32) -> Result<()> where K: AsRef<[u8]> + Sync {
//             Ok(())
//         }
//
//         async fn delete<K>(&mut self, key: &K) -> Result<()> where K: AsRef<[u8]> + Sync {
//             let inner = Arc::make_mut(&mut self.inner);
//             inner.remove(&Vec::from(key.as_ref()));
//             Ok(())
//         }
//
//         async fn clear(&mut self) -> Result<()> {
//             let inner = Arc::make_mut(&mut self.inner);
//             inner.clear();
//             Ok(())
//         }
//
//         async fn has<K> (&self, key: &K) -> Result<bool> where K: AsRef<[u8]> + Sync {
//             Ok(self.inner.contains_key(&Vec::from(key.as_ref())))
//         }
//     }
//
//    // #[tokio::test]
//     async fn test_oa() {
//         let config = config::WechatBase {
//             app_id: "wx5a3dbaf21ec95f39".to_string(),
//             secret: "4c3e8320e3ea6b5b3a1d4878b40664d7".to_string(),
//             token: "".to_string(),
//             aes_key: "".to_string(),
//         };
//
//         let cache = TestCache::default();
//
//         let mut oa = OfficialAccount::new(cache, config);
//
//         let mut accesstoken = oa.accesstoken();
//         let token = accesstoken.get_token().await;
//         println!("{:?}", token);
//
//         let mut jssdk = oa.jssdk();
//         let config = jssdk.build_config(true, String::from("http://baidu.com"), vec![String::from("test")]).await;
//         println!("{:?}", config);
//     }
//
// }
/*  */
