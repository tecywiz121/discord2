use chrono::{TimeZone, Utc};

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
    let audit_log = discord
        .get_guild_audit_log(guild_id, None, None, None, None)
        .await?;
    println!("Recent entries: {:#?}", audit_log);

    //
    // Filter audit logs by user.
    //
    let me = discord.get_current_user().await?;
    let my_audit_log = discord
        .get_guild_audit_log(guild_id, Some(me.id()), None, None, None)
        .await?;
    println!("My entries: {:#?}", my_audit_log);

    //
    // Filter audit logs by action type.
    //
    let ch_audit_log = discord
        .get_guild_audit_log(
            guild_id,
            None,
            Some(AuditLogEvent::ChannelCreate),
            None,
            None,
        )
        .await?;
    println!("Channel create entries: {:#?}", ch_audit_log);

    //
    // Filter audit logs by date.
    //
    let date = Utc.ymd(2020, 12, 15).and_hms(0, 0, 0);
    let date_audit_log = discord
        .get_guild_audit_log(
            guild_id,
            None,
            None,
            Some(Snowflake::from_date_time(date).unwrap()),
            None,
        )
        .await?;
    println!("Old entries: {:#?}", date_audit_log);
    Ok(())
}
