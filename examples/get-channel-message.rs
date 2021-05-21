use discord2::resources::channel::{ChannelId, MessageId};
use discord2::{Config, Discord, Error, Token};

use std::env::var;
use std::str::FromStr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let token = var("DISCORD_BOT_TOKEN").unwrap();
    let channel_id = var("DISCORD_CHANNEL_ID").unwrap();
    let message_id = var("DISCORD_MESSAGE_ID").unwrap();

    let config = Config::builder().token(Token::bot(token)).build();

    let discord = Discord::new(&config)?;

    let channel_id = ChannelId::from_str(&channel_id).unwrap();
    let message_id = MessageId::from_str(&message_id).unwrap();
    let message = discord.get_channel_message(channel_id, message_id).await?;

    println!("{:#?}", message);

    Ok(())
}
