use poise::serenity_prelude::{self as serenity};

use crate::data::BotData;
use crate::schedule::{save_schedule, show_schedule, next_stream};

pub mod data;
pub mod schedule;

#[tokio::main]
async fn main() {
    // TODO maybe remove forced dotenv
    dotenv::dotenv().expect("shouldn't have failed nerd");

    let token = std::env::var("BOT_TOKEN").expect("missing BOT_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                save_schedule(),
                show_schedule(),
                next_stream(),
                ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(BotData {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
