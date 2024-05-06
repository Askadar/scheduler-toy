pub mod fs;
pub use fs::FsStorage;
use poise::serenity_prelude::async_trait;
// pub use redis::RedisStorage;

use crate::data::Entry;

#[async_trait]
pub trait Storage {
    async fn get(&self, guild: &str) -> Option<Vec<Entry>>;
    async fn set(&self, guild: &str, data: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
