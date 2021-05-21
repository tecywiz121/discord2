use discord2::application::*;
use discord2::guild::GuildId;
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

    let me = discord.get_current_user().await?;

    // List existing commands.
    let current = discord
        .get_guild_application_commands(me.id().into(), guild_id)
        .await?;

    println!("Current Commands: {:#?}", current);

    // Create a single command: `/guild-hello`
    let cmd = NewApplicationCommand::builder()
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
        .build();

    let created = discord
        .create_guild_application_command(me.id().into(), guild_id, &cmd)
        .await?;

    println!("Created Command: {:#?}", created);

    // Update the `/guild-hello` command with a new description.
    let cmd2 = EditApplicationCommand::builder()
        .description("this is an updated guild command")
        .build();

    let edited = discord
        .edit_guild_application_command(
            me.id().into(),
            guild_id,
            created.id(),
            &cmd2,
        )
        .await?;

    println!("\nEdited Command: {:#?}", edited);

    // Delete the `/guild-hello` command.
    discord
        .delete_guild_application_command(me.id().into(), guild_id, edited.id())
        .await?;

    println!("\nCommand Deleted.");

    // Bulk create two commands.
    let commands: &[_] = &[
        cmd,
        NewApplicationCommand::builder()
            .name("guild-goodbye")
            .description("this is another guild command")
            .build(),
    ];

    let created = discord
        .create_all_guild_application_commands(
            me.id().into(),
            guild_id,
            commands,
        )
        .await?;

    println!("\nCreated commands: {:#?}", created);

    // Set the specific permissions for a command.
    let eperms = discord
        .edit_application_command_permissions(
            me.id().into(),
            guild_id,
            created[0].id(),
            &[ApplicationCommandPermission::builder()
                .id(me.id())
                .permission(false)
                .build()],
        )
        .await?;

    println!("\nEdited permissions: {:#?}", eperms);

    // Get the specific permissions for a command.
    let perms = discord
        .get_application_command_permissions(
            me.id().into(),
            guild_id,
            created[0].id(),
        )
        .await?;

    println!("\nCommand permissions: {:#?}", perms);

    Ok(())
}
