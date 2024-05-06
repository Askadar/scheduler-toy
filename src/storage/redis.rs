use std::sync::Mutex;

use poise::serenity_prelude::async_trait;

use crate::data::Entry;

use super::Storage;

pub struct RedisStorage {
    redis: Mutex<simple_redis::client::Client>,
}

impl RedisStorage {
    pub fn default() -> Self {
        Self {
            redis: Mutex::new(simple_redis::create("redis://0.0.0.0:6379").unwrap()),
        }
    }

    fn schedule_path(guild: &str) -> String {
        format!("schedules:{guild}")
    }
}

#[async_trait]
impl Storage for RedisStorage {
    async fn get(&self, guild: &str) -> Option<Vec<Entry>> {
        let r = self.redis.lock();
        if let Err(e) = r {
            println!("Failed to accedd redit [{e}]");
            return None;
        }
        let mut r = r.unwrap();

        let schedule: Result<String, simple_redis::types::RedisError> =
            r.get(&Self::schedule_path(&guild));

        match schedule {
            Ok(slice) => match serde_json::from_str(&slice) {
                Ok(schedule) => Some(schedule),
                Err(e) => {
                    println!("Failed to parse because of [{e}]. Slice was [{slice}]");
                    None
                }
            },
            Err(e) => {
                println!("Failed to get schedule because of [{e}]");
                None
            }
        }
    }

    async fn set(
        &self,
        guild: &str,
        data: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let r = self.redis.lock();
        if let Err(e) = r {
            println!("Failed to accedd redit [{e}]");
            return Err(e.to_string().into());
        }
        let mut r = r.unwrap();

        match r.set(&Self::schedule_path(&guild), data) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Failed to set schedule because of [{e}]");
                Err(Box::new(e))
            }
        }
    }
}
