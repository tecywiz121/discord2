use discord2::application::{
    ApplicationCommandOption, ApplicationCommandOptionChoice,
    ApplicationCommandOptionKind, NewApplicationCommand,
};
use discord2::{Config, Discord, Error, Token};

use std::env::var;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let token = var("DISCORD_BOT_TOKEN").unwrap();
    let config = Config::builder().token(Token::bot(token)).build();

    let discord = Discord::new(&config)?;

    let me = discord.get_current_user().await?;
    let cmd = NewApplicationCommand::builder()
        .name("hello")
        .description("this is a command")
        .options([ApplicationCommandOption::builder()
            .kind(ApplicationCommandOptionKind::String)
            .name("who")
            .required(true)
            .description("who are you saying hello to")
            .choices([
                ApplicationCommandOptionChoice::builder()
                    .name("The World")
                    .value("world")
                    .build(),
                ApplicationCommandOptionChoice::builder()
                    .name("Yourself")
                    .value("yourself")
                    .build(),
            ])
            .build()])
        .build();

    let created = discord
        .create_global_application_command(me.id().into(), &cmd)
        .await?;

    println!("{:#?}", created);

    Ok(())
}
