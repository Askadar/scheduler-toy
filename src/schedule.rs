use crate::data::{BotData, Entry};
use chrono::{Datelike, NaiveDateTime};
use poise::serenity_prelude::{self as serenity, GetMessages};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, BotData, Error>;

fn parse_schedule<'a, C>(lines: C) -> Vec<Entry>
where
    C: IntoIterator<Item = &'a str>,
{
    let entries = lines
        .into_iter()
        .map(move |s| {
            let parts = s.split(" - ").collect::<Vec<_>>();
            match parts.len() {
                2 => Entry::new(&parts),
                len => panic!("Expected 2 parts, got {}", len),
            }
        })
        .map(|entry| {
            let teststr = format!("{} {} 00:00", chrono::Utc::now().year(), entry.datestr);

            let date = NaiveDateTime::parse_from_str(&teststr, "%Y %d/%m %l%p %M:%S");
            let date_time = date
                .unwrap()
                .and_local_timezone(chrono::FixedOffset::east_opt(10 * 3600).unwrap())
                .unwrap()
                .to_utc();

            Entry {
                datestr: entry.datestr,
                label: entry.label,
                date: Some(date_time),
            }
        })
        .collect::<Vec<_>>();

    entries
}

fn format_schedule(entries: &[Entry]) -> String {
    let message = entries
        .iter()
        .map(|s| format_entry(s))
        .collect::<Vec<_>>()
        .join("\n");

    format!("Found these entries:\n{}", message)
}

fn format_entry(entry: &Entry) -> String {
    format!("<t:{}:F>: {}", entry.date.unwrap().timestamp(), entry.label)
}

#[poise::command(slash_command)]
pub async fn save_schedule(
    ctx: Context<'_>,
    #[description = "Channel to fetch schedule from"] channel: Option<serenity::GuildChannel>,
) -> Result<(), Error> {
    let sauce = channel.or(ctx.guild_channel().await).unwrap();

    let messages = sauce
        .messages(&ctx.http(), GetMessages::new().limit(1))
        .await?;

    let message = messages.get(0).ok_or("No schedule message found")?;
    let entries = message
        .content
        .split("\n")
        .skip_while(|s| !s.contains("**schedule**"))
        .skip(1)
        .take_while(|s| s.trim() != "");

    if entries.clone().count() > 0 {
        let mut schedule = parse_schedule(entries);
        schedule.sort_by_key(|e| e.date);
        std::fs::write("schedule.json", serde_json::to_string(&schedule).unwrap()).unwrap();

        ctx.say(format_schedule(&schedule)).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn show_schedule(ctx: Context<'_>) -> Result<(), Error> {
    let schedule = std::fs::read("schedule.json");

    match schedule {
        Ok(schedule) => {
            let schedule: Vec<Entry> = serde_json::from_slice(&schedule).unwrap();
            ctx.say(format_schedule(&schedule)).await?;
        }
        Err(err) => {
            println!("Failed to read schedule file {:?}", err);
            ctx.say("No schedule saved").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn next_stream(ctx: Context<'_>) -> Result<(), Error> {
    let schedule = std::fs::read("schedule.json");

    match schedule {
        Ok(schedule) => {
            let schedule: Vec<Entry> = serde_json::from_slice(&schedule).unwrap();
            let now = chrono::Utc::now();
            for e in schedule {
                let scheduled = e.date.unwrap();

                if scheduled.timestamp() > now.timestamp() {
                    ctx.say(format_entry(&e)).await?;
                    break;
                }
            }
        }
        Err(err) => {
            println!("Failed to read schedule file {:?}", err);
            ctx.say("No schedule saved").await?;
        }
    }

    Ok(())
}
