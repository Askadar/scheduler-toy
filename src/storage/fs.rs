use std::path::PathBuf;

use poise::serenity_prelude::async_trait;
use tokio::fs;

use crate::data::Entry;

use super::Storage;

pub struct FsStorage {
    root: PathBuf,
}

impl FsStorage {
    pub fn default() -> Self {
        Self {
            root: std::env::current_dir()
                .unwrap_or(PathBuf::from("./"))
                .join("schedules"),
        }
    }

    async fn create_path(&self, guild: &str) -> Result<PathBuf, std::io::Error> {
        fs::create_dir_all(&self.root).await?;
        Ok(self.root.join(format!("schedule_{}.json", guild)))
    }
}

#[async_trait]
impl Storage for FsStorage {
    async fn get(&self, guild: &str) -> Option<Vec<Entry>> {
        let path = self.create_path(guild).await;
        if let Err(e) = path {
            println!("Failed to create path because of [{e}]");
            return None;
        }
        let path = path.unwrap();

        let schedule_slice = std::fs::read(&path);
        match schedule_slice {
            Ok(slice) => match serde_json::from_slice(&slice) {
                Ok(schedule) => Some(schedule),
                Err(e) => {
                    let slice = std::str::from_utf8(&slice).unwrap_or(&"Failed to stringify");
                    println!("Failed to parse because of [{e}]. Slice was [{slice}]");
                    None
                }
            },
            Err(e) => {
                println!(
                    "Failed to read schedule file [{:?}] because of [{e}]",
                    &path
                );
                None
            }
        }
    }

    async fn set(
        &self,
        guild: &str,
        data: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path = self.create_path(guild).await;
        if let Err(e) = path {
            println!("Failed to create path because of [{e}]");
            return Err(Box::new(e));
        }
        // TODO find out how to unwrap it proper
        let path = path.unwrap();

        match std::fs::write(&path, data) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!(
                    "Failed to write schedule file [{:?}] because of [{e}]",
                    &path
                );
                Err(Box::new(e))
            }
        }
    }
}
