use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::AsRef;
use crate::Result;

#[async_trait]
pub trait Cache : Clone {
    async fn get<K, V>(&self, key: &K) -> Result<Option<V>> where K: AsRef<[u8]> + Sync, V: DeserializeOwned + Sync;

    async fn set<K, V>(&mut self, key: &K, value: V) -> Result<()> where K: AsRef<[u8]> + Sync, V: Serialize + Send;

    async fn ttl<K>(&mut self, key: &K, ttl: u32) -> Result<()> where K: AsRef<[u8]> + Sync;

    async fn delete<K>(&mut self, key: &K) -> Result<()> where K: AsRef<[u8]> + Sync;

    async fn clear(&mut self) -> Result<()>;

    async fn has<K> (&self, key: &K) -> Result<bool> where K: AsRef<[u8]> + Sync;
}

