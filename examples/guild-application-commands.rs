use discord2::requests::*;
use discord2::resources::application::*;
use discord2::resources::guild::GuildId;
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

    let me = GetCurrentUser::builder().build().send(&discord).await?;

    // List existing commands.
    let current = GetGuildApplicationCommands::builder()
        .application_id(me.id())
        .guild_id(guild_id)
        .build()
        .send(&discord)
        .await?;

    println!("Current Commands: {:#?}", current);

    // Create a single command: `/guild-hello`
    let created = CreateGuildApplicationCommand::builder()
        .guild_id(guild_id)
        .application_id(me.id())
        .name("guild-hello")
        .description("this is a guild command")
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

    // Update the `/guild-hello` command with a new description.
    let edited = EditGuildApplicationCommand::builder()
        .application_id(me.id())
        .guild_id(guild_id)
        .command_id(created.id())
        .description("this is an updated guild command")
        .build()
        .send(&discord)
        .await?;

    println!("\nEdited Command: {:#?}", edited);

    // Delete the `/guild-hello` command.
    DeleteGuildApplicationCommand::builder()
        .application_id(me.id())
        .guild_id(guild_id)
        .command_id(edited.id())
        .build()
        .send(&discord)
        .await?;

    println!("\nCommand Deleted.");

    // Bulk create two commands.
    let commands: &[_] = &[
        NewApplicationCommand::builder()
            .name("guild-hello")
            .description("this is shorter guild command")
            .build(),
        NewApplicationCommand::builder()
            .name("guild-goodbye")
            .description("this is another guild command")
            .build(),
    ];

    let created = BulkOverwriteGuildApplicationCommands::builder()
        .application_id(me.id())
        .guild_id(guild_id)
        .commands(commands)
        .build()
        .send(&discord)
        .await?;

    println!("\nCreated commands: {:#?}", created);

    // Set the specific permissions for a command.
    let eperms = EditApplicationCommandPermissions::builder()
        .application_id(me.id())
        .guild_id(guild_id)
        .command_id(created[0].id())
        .permissions([ApplicationCommandPermission::builder()
            .id(me.id())
            .permission(true)
            .build()])
        .build()
        .send(&discord)
        .await?;

    println!("\nEdited permissions: {:#?}", eperms);

    // Get the specific permissions for a command.
    let perms = GetApplicationCommandPermissions::builder()
        .application_id(me.id())
        .guild_id(guild_id)
        .command_id(created[0].id())
        .build()
        .send(&discord)
        .await?;

    println!("\nCommand permissions: {:#?}", perms);

    Ok(())
}
