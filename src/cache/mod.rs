use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::AsRef;
use crate::Result;

#[async_trait]
pub trait Cache : Clone {
    async fn get<K, V>(&self, key: &K) -> Result<Option<V>> where K: AsRef<[u8]> + Sync, V: DeserializeOwned + Sync;

    async fn set<K, V>(&self, key: &K, value: V) -> Result<()> where K: AsRef<[u8]> + Sync, V: Serialize + Send;

    async fn ttl<K>(&self, key: &K, ttl: u32) -> Result<()> where K: AsRef<[u8]> + Sync;

    async fn delete<K>(&self, key: &K) -> Result<()> where K: AsRef<[u8]> + Sync;

    async fn clear(&self) -> Result<()>;

    async fn has<K> (&self, key: &K) -> Result<bool> where K: AsRef<[u8]> + Sync;
}

