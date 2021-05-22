use chrono::{TimeZone, Utc};

use discord2::requests::*;
use discord2::resources::audit_log::AuditLogEvent;
use discord2::resources::guild::GuildId;
use discord2::snowflake::Snowflake;
use discord2::{Config, Discord, Error, Token};

use std::env::var;
use std::str::FromStr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let token = var("DISCORD_BOT_TOKEN").unwrap();
    let guild_id = var("DISCORD_GUILD_ID").unwrap();

    let config = Config::builder().token(Token::bot(token)).build();

    let discord = Discord::new(&config)?;

    let guild_id = GuildId::from_str(&guild_id).unwrap();

    //
    // Get all recent audit logs.
    //
    let audit_log = GetGuildAuditLog::builder()
        .guild_id(guild_id)
        .build()
        .send(&discord)
        .await?;
    println!("Recent entries: {:#?}", audit_log);

    //
    // Filter audit logs by user.
    //
    let me = GetCurrentUser::builder().build().send(&discord).await?;
    let my_audit_log = GetGuildAuditLog::builder()
        .guild_id(guild_id)
        .user_id(me.id())
        .build()
        .send(&discord)
        .await?;
    println!("My entries: {:#?}", my_audit_log);

    //
    // Filter audit logs by action type.
    //
    let ch_audit_log = GetGuildAuditLog::builder()
        .guild_id(guild_id)
        .action_kind(AuditLogEvent::ChannelCreate)
        .build()
        .send(&discord)
        .await?;
    println!("Channel create entries: {:#?}", ch_audit_log);

    //
    // Filter audit logs by date.
    //
    let date = Utc.ymd(2020, 12, 15).and_hms(0, 0, 0);
    let date_audit_log = GetGuildAuditLog::builder()
        .guild_id(guild_id)
        .before(Snowflake::from_date_time(date).unwrap())
        .build()
        .send(&discord)
        .await?;
    println!("Old entries: {:#?}", date_audit_log);
    Ok(())
}
