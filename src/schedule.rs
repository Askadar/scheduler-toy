use crate::data::{BotData, Entry};
use chrono::{Datelike, NaiveDateTime};
use poise::serenity_prelude::{self as serenity, GetMessages};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, BotData, Error>;

fn parse_schedule<'a, C>(lines: C) -> Option<Vec<Entry>>
where
    C: IntoIterator<Item = &'a str>,
{
    let entries = lines
        .into_iter()
        .filter_map(move |s| {
            let parts = s.split(" - ").collect::<Vec<_>>();
            match parts.len() {
                2 => Some(Entry::new(&parts)),
                _ => None,
            }
        })
        .filter_map(|entry| {
            let teststr = format!("{} {} 00:00", chrono::Utc::now().year(), entry.datestr);

            let date = NaiveDateTime::parse_from_str(&teststr, "%Y %d/%m %l%p %M:%S");
            let date_time = match date {
                Ok(date) => {
                    let date = date
                        .and_local_timezone(
                            chrono::FixedOffset::east_opt(10 * 3600)
                                .expect("Failed to create constant timezone"),
                        )
                        .map(|d| d.to_utc())
                        .single();

                    date
                }
                Err(e) => {
                    eprintln!("Failed to parse date [{}] reason [{e}]", entry.datestr);
                    None
                }
            };

            match date_time {
                Some(_) => Some(Entry {
                    datestr: entry.datestr,
                    label: entry.label,
                    date: date_time,
                }),
                None => None,
            }
        })
        .collect::<Vec<_>>();

    if entries.len() == 0 {
        return None;
    }
    Some(entries)
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
    let storage = &ctx.data().storage;
    let guild = ctx
        .guild()
        .ok_or("Save schedule can only be called from whithin a discord server")?
        .id
        .to_string();
    let sauce = channel
        .or(ctx.guild_channel().await)
        .ok_or("Failed to find channel to get schedule from")?;

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
        let schedule = parse_schedule(entries);
        match schedule {
            Some(mut schedule) => {
                schedule.sort_by_key(|e| e.date);
                let data = serde_json::to_string(&schedule)?;

                storage.set(&guild, &data).await?;

                ctx.reply(format_schedule(&schedule)).await?;
            }
            None => {
                ctx.reply("No schedule found in provided channel's last messages")
                    .await?;
            }
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn show_schedule(ctx: Context<'_>) -> Result<(), Error> {
    let storage = &ctx.data().storage;
    let guild = ctx.guild().unwrap().id.to_string();
    let schedule = storage.get(&guild).await;

    match schedule {
        Some(schedule) => {
            ctx.reply(format_schedule(&schedule)).await?;
        }
        None => {
            ctx.reply("No schedule saved").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn next_stream(ctx: Context<'_>) -> Result<(), Error> {
    let storage = &ctx.data().storage;
    let guild = ctx.guild().unwrap().id.to_string();
    let schedule = storage.get(&guild).await;

    match schedule {
        Some(schedule) => {
            let now = chrono::Utc::now();
            for e in schedule {
                let scheduled = e.date.unwrap();

                if scheduled.timestamp() > now.timestamp() {
                    ctx.reply(format_entry(&e)).await?;
                    break;
                }
            }
        }
        None => {
            ctx.reply("No schedule saved").await?;
        }
    }

    Ok(())
}
