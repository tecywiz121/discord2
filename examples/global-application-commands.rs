use discord2::application::*;
use discord2::{Config, Discord, Error, Token};

use std::env::var;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let token = var("DISCORD_BOT_TOKEN").unwrap();
    let config = Config::builder().token(Token::bot(token)).build();

    let discord = Discord::new(&config)?;

    let me = discord.get_current_user().await?;

    // List existing commands.
    let current = discord
        .get_global_application_commands(me.id().into())
        .await?;

    println!("Current Commands: {:#?}", current);

    // Create a single command: `/hello`
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

    println!("Created Command: {:#?}", created);

    // Update the `/hello` command with a new description.
    let cmd2 = EditApplicationCommand::builder()
        .description("this is an updated command")
        .build();

    let edited = discord
        .edit_global_application_command(me.id().into(), created.id(), &cmd2)
        .await?;

    println!("\nEdited Command: {:#?}", edited);

    // Delete the `/hello` command.
    discord
        .delete_global_application_command(me.id().into(), edited.id())
        .await?;

    println!("\nCommand Deleted.");

    // Bulk create two commands.
    let commands: &[_] = &[
        cmd,
        NewApplicationCommand::builder()
            .name("goodbye")
            .description("this is another command")
            .build(),
    ];

    let created = discord
        .create_all_global_application_commands(me.id().into(), commands)
        .await?;

    println!("\nCreated commands: {:#?}", created);

    Ok(())
}
