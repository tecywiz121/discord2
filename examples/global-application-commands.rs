use discord2::requests::*;
use discord2::resources::application::*;
use discord2::{Config, Discord, Error, Token};

use std::env::var;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let token = var("DISCORD_BOT_TOKEN").unwrap();
    let config = Config::builder().token(Token::bot(token)).build();

    let discord = Discord::new(&config)?;

    let me = GetCurrentUser::builder().build().send(&discord).await?;

    // List existing commands.
    let current = GetGlobalApplicationCommands::builder()
        .application_id(me.id())
        .build()
        .send(&discord)
        .await?;

    println!("Current Commands: {:#?}", current);

    // Create a single command: `/hello`
    let created = CreateGlobalApplicationCommand::builder()
        .application_id(me.id())
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
        .build()
        .send(&discord)
        .await?;

    println!("Created Command: {:#?}", created);

    // Update the `/hello` command with a new description.
    let edited = EditGlobalApplicationCommand::builder()
        .application_id(me.id())
        .command_id(created.id())
        .description("this is an updated command")
        .build()
        .send(&discord)
        .await?;

    println!("\nEdited Command: {:#?}", edited);

    // Delete the `/hello` command.
    DeleteGlobalApplicationCommand::builder()
        .application_id(me.id())
        .command_id(edited.id())
        .build()
        .send(&discord)
        .await?;

    println!("\nCommand Deleted.");

    // Bulk create two commands.
    let commands: &[_] = &[
        NewApplicationCommand::builder()
            .name("hello")
            .description("this is short command")
            .build(),
        NewApplicationCommand::builder()
            .name("goodbye")
            .description("this is another command")
            .build(),
    ];

    let created = BulkOverwriteGlobalApplicationCommands::builder()
        .commands(commands)
        .application_id(me.id())
        .build()
        .send(&discord)
        .await?;

    println!("\nCreated commands: {:#?}", created);

    Ok(())
}
