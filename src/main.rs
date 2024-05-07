use poise::serenity_prelude::{self as serenity};
use storage::{
    // FsStorage,
    FsStorage, RedisStorage, Storage
};

use crate::data::BotData;
use crate::schedule::{next_stream, save_schedule, show_schedule};

pub mod data;
pub mod schedule;
pub mod storage;

#[tokio::main]
async fn main() {
    dotenv::dotenv().map_err(|e| eprintln!("failed to load dotenv, be sure to have necessary env setup in other ways: {}", e)).ok();

    let token = std::env::var("BOT_TOKEN").expect("missing BOT_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // TODO find nice way to match without wrapping everything in Strings
    let storage_var = std::env::var("STORAGE_DRIVER").unwrap_or("default".into());
    let storage: Box<dyn Storage + Send + Sync> = match storage_var.as_str() {
        "redis" => Box::new(RedisStorage::default()),
        "fs" => Box::new(FsStorage::default()),
        _e => {
            println!("Unrecognized storage driver setting [{_e}] defaulting to fs");
            Box::new(FsStorage::default())
        },
    };


    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![save_schedule(), show_schedule(), next_stream()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(BotData {
                    storage,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
