use discord2::permissions::{Permissions, RoleId};
use discord2::requests::*;
use discord2::resources::channel::{ChannelId, Overwrite};
use discord2::{Config, Discord, Error, Token};

use std::env::var;
use std::str::FromStr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let token = var("DISCORD_BOT_TOKEN").unwrap();
    let channel_id = var("DISCORD_CHANNEL_ID").unwrap();

    let config = Config::builder().token(Token::bot(token)).build();

    let discord = Discord::new(&config)?;

    let channel_id = ChannelId::from_str(&channel_id).unwrap();

    let channel = GetChannel::builder()
        .channel_id(channel_id)
        .build()
        .send(&discord)
        .await?;

    println!("Got Channel: {:#?}", channel);

    let everyone = RoleId::everyone(channel.guild_id().unwrap());
    let edit = ModifyChannel::builder()
        .channel_id(channel_id)
        .name("new-name")
        .permission_overwrites([Overwrite::builder()
            .id(everyone)
            .allow(Permissions::empty())
            .deny(Permissions::all())
            .build()])
        .build()
        .send(&discord)
        .await?;

    println!("Edited Channel: {:#?}", edit);

    Ok(())
}
