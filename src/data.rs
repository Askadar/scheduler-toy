use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::storage::Storage;

// User data, which is stored and accessible in all command invocations
pub struct BotData {
    pub storage: Box<dyn Storage + Send + Sync>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    // TODO maybe private
    pub datestr: String,

    pub label: String,
    pub date: Option<DateTime<Utc>>,
}

impl Entry {
    pub fn new(parts: &[&str]) -> Self {
        let datestr = parts[0].split(" ").skip(1).collect::<Vec<_>>().join(" ");
        let label = parts[1].to_string();
        Self {
            datestr,
            label,
            date: None,
        }
    }
}
