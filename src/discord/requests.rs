use crate::enums::IntegerEnum;
use crate::image::UploadImage;
use crate::resources::application::{
    ApplicationCommand, ApplicationCommandId, ApplicationCommandOption,
    ApplicationCommandPermission, ApplicationId, EditApplicationCommand,
    EditGuildApplicationCommandPermissions, GuildApplicationCommandPermissions,
    NewApplicationCommand,
};
use crate::resources::audit_log::{AuditLog, AuditLogEntryId, AuditLogEvent};
use crate::resources::channel::{
    Channel, ChannelId, ChannelKind, EditChannel, Message, MessageId,
    Overwrite, VideoQualityMode,
};
use crate::resources::guild::GuildId;
use crate::resources::user::{User, UserId};

use serde::Serialize;

use super::{Discord, Error};

use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetGlobalApplicationCommands {
    #[builder(setter(into))]
    application_id: ApplicationId,
}

impl GetGlobalApplicationCommands {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!("applications/{}/commands", self.application_id);
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetGlobalApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    command_id: ApplicationCommandId,
}

impl GetGlobalApplicationCommand {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<ApplicationCommand, Error> {
        let path = format!(
            "applications/{}/commands/{}",
            self.application_id, self.command_id
        );
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct BulkOverwriteGlobalApplicationCommands {
    #[builder(setter(into))]
    application_id: ApplicationId,

    #[builder(setter(into))]
    commands: Vec<NewApplicationCommand>,
}

impl BulkOverwriteGlobalApplicationCommands {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!("applications/{}/commands", self.application_id);
        discord.put(path, &self.commands).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CreateGlobalApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,

    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    description: String,

    #[builder(default, setter(strip_option, into))]
    options: Option<Vec<ApplicationCommandOption>>,

    #[builder(default, setter(strip_option, into))]
    default_permission: Option<bool>,
}

impl CreateGlobalApplicationCommand {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<ApplicationCommand, Error> {
        let new_command = NewApplicationCommand {
            name: self.name,
            description: self.description,
            options: self.options,
            default_permission: self.default_permission,
        };

        let path = format!("applications/{}/commands", self.application_id);
        discord.post(path, &new_command).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct EditGlobalApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    command_id: ApplicationCommandId,

    #[builder(default, setter(into, strip_option))]
    name: Option<String>,

    #[builder(default, setter(into, strip_option))]
    description: Option<String>,

    #[builder(default, setter(strip_option, into))]
    options: Option<Vec<ApplicationCommandOption>>,

    #[builder(default, setter(strip_option, into))]
    default_permission: Option<bool>,
}

impl EditGlobalApplicationCommand {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<ApplicationCommand, Error> {
        let edit_command = EditApplicationCommand {
            name: self.name,
            description: self.description,
            options: self.options,
            default_permission: self.default_permission,
        };

        let path = format!(
            "applications/{}/commands/{}",
            self.application_id, self.command_id
        );
        discord.patch(path, &edit_command).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct DeleteGlobalApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    command_id: ApplicationCommandId,
}

impl DeleteGlobalApplicationCommand {
    pub async fn send(self, discord: &Discord) -> Result<(), Error> {
        let path = format!(
            "applications/{}/commands/{}",
            self.application_id, self.command_id
        );
        discord.delete(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetGuildApplicationCommands {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,
}

impl GetGuildApplicationCommands {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands",
            self.application_id, self.guild_id
        );
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetGuildApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    command_id: ApplicationCommandId,
    guild_id: GuildId,
}

impl GetGuildApplicationCommand {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<ApplicationCommand, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}",
            self.application_id, self.guild_id, self.command_id
        );
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct BulkOverwriteGuildApplicationCommands {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,

    #[builder(setter(into))]
    commands: Vec<NewApplicationCommand>,
}

impl BulkOverwriteGuildApplicationCommands {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands",
            self.application_id, self.guild_id
        );
        discord.put(path, &self.commands).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CreateGuildApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,

    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    description: String,

    #[builder(default, setter(strip_option, into))]
    options: Option<Vec<ApplicationCommandOption>>,

    #[builder(default, setter(strip_option, into))]
    default_permission: Option<bool>,
}

impl CreateGuildApplicationCommand {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<ApplicationCommand, Error> {
        let new_command = NewApplicationCommand {
            name: self.name,
            description: self.description,
            options: self.options,
            default_permission: self.default_permission,
        };

        let path = format!(
            "applications/{}/guilds/{}/commands",
            self.application_id, self.guild_id
        );
        discord.post(path, &new_command).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct EditGuildApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,
    command_id: ApplicationCommandId,

    #[builder(default, setter(into, strip_option))]
    name: Option<String>,

    #[builder(default, setter(into, strip_option))]
    description: Option<String>,

    #[builder(default, setter(strip_option, into))]
    options: Option<Vec<ApplicationCommandOption>>,

    #[builder(default, setter(strip_option, into))]
    default_permission: Option<bool>,
}

impl EditGuildApplicationCommand {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<ApplicationCommand, Error> {
        let edit_command = EditApplicationCommand {
            name: self.name,
            description: self.description,
            options: self.options,
            default_permission: self.default_permission,
        };

        let path = format!(
            "applications/{}/guilds/{}/commands/{}",
            self.application_id, self.guild_id, self.command_id
        );
        discord.patch(path, &edit_command).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct DeleteGuildApplicationCommand {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,
    command_id: ApplicationCommandId,
}

impl DeleteGuildApplicationCommand {
    pub async fn send(self, discord: &Discord) -> Result<(), Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}",
            self.application_id, self.guild_id, self.command_id
        );
        discord.delete(path).await
    }
}

// TODO: CreateInteractionResponse
// TODO: GetOriginalInteractionResponse
// TODO: EditOriginalInteractionResponse
// TODO: DeleteOriginalInteractionResponse
// TODO: CreateFollowupMessage
// TODO: EditFollowupMessage
// TODO: DeleteFollowupMessage

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetGuildApplicationCommandPermissions {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,
}

impl GetGuildApplicationCommandPermissions {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<Vec<GuildApplicationCommandPermissions>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/permissions",
            self.application_id, self.guild_id
        );
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetApplicationCommandPermissions {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,
    command_id: ApplicationCommandId,
}

impl GetApplicationCommandPermissions {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<GuildApplicationCommandPermissions, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}/permissions",
            self.application_id, self.guild_id, self.command_id,
        );
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct EditApplicationCommandPermissions {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,
    command_id: ApplicationCommandId,

    #[builder(setter(into))]
    permissions: Vec<ApplicationCommandPermission>,
}

impl EditApplicationCommandPermissions {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<GuildApplicationCommandPermissions, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}/permissions",
            self.application_id, self.guild_id, self.command_id
        );

        #[derive(Debug, Serialize)]
        struct Request<'a> {
            permissions: &'a [ApplicationCommandPermission],
        }

        discord
            .put(
                path,
                &Request {
                    permissions: &self.permissions,
                },
            )
            .await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct BatchEditApplicationCommandPermissions {
    #[builder(setter(into))]
    application_id: ApplicationId,
    guild_id: GuildId,

    #[builder(setter(into))]
    command_permissions: Vec<EditGuildApplicationCommandPermissions>,
}

impl BatchEditApplicationCommandPermissions {
    pub async fn send(
        self,
        discord: &Discord,
    ) -> Result<Vec<GuildApplicationCommandPermissions>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/permissions",
            self.application_id, self.guild_id
        );

        discord.put(path, &self.command_permissions).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetGuildAuditLog {
    guild_id: GuildId,

    #[builder(default, setter(strip_option))]
    user_id: Option<UserId>,

    #[builder(default, setter(strip_option, into))]
    action_kind: Option<IntegerEnum<AuditLogEvent>>,

    #[builder(default, setter(strip_option))]
    before: Option<AuditLogEntryId>,

    #[builder(default, setter(strip_option))]
    limit: Option<u64>,
}

impl GetGuildAuditLog {
    pub async fn send(self, discord: &Discord) -> Result<AuditLog, Error> {
        let mut path = format!("guilds/{}/audit-logs", self.guild_id);

        let user_id = self.user_id.map(|u| format!("user_id={}", u));
        let action_type = self
            .action_kind
            .map(|u| format!("action_type={}", u64::from(u)));
        let before = self.before.map(|u| format!("before={}", u));
        let limit = self.limit.map(|u| format!("limit={}", u));

        let query = user_id
            .into_iter()
            .chain(action_type.into_iter())
            .chain(before.into_iter())
            .chain(limit.into_iter())
            .collect::<Vec<_>>()
            .join("&");

        if !query.is_empty() {
            path.push('?');
            path.push_str(&query);
        }

        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetCurrentUser {
    #[builder(default, setter(skip))]
    _p: (),
}

impl GetCurrentUser {
    pub async fn send(self, discord: &Discord) -> Result<User, Error> {
        let path = "users/@me";
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetChannel {
    channel_id: ChannelId,
}

impl GetChannel {
    pub async fn send(self, discord: &Discord) -> Result<Channel, Error> {
        let path = format!("channels/{}", self.channel_id);
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetChannelMessage {
    channel_id: ChannelId,
    message_id: MessageId,
}

impl GetChannelMessage {
    pub async fn send(self, discord: &Discord) -> Result<Message, Error> {
        let path = format!(
            "channels/{}/messages/{}",
            self.channel_id, self.message_id
        );
        discord.get(path).await
    }
}

#[derive(Debug, Clone, TypedBuilder, Serialize)]
pub struct ModifyChannel {
    channel_id: ChannelId,

    #[builder(default, setter(strip_option, into))]
    name: Option<String>,

    #[builder(default, setter(strip_option))]
    icon: Option<UploadImage>,

    #[builder(default, setter(strip_option, into))]
    kind: Option<IntegerEnum<ChannelKind>>,

    #[builder(default, setter(strip_option))]
    position: Option<u64>,

    #[builder(default, setter(strip_option, into))]
    topic: Option<String>,

    #[builder(default, setter(strip_option))]
    nsfw: Option<bool>,

    #[builder(default, setter(strip_option))]
    rate_limit_per_user: Option<u64>,

    #[builder(default, setter(strip_option))]
    bitrate: Option<u64>,

    #[builder(default, setter(strip_option))]
    user_limit: Option<u64>,

    #[builder(default, setter(strip_option, into))]
    permission_overwrites: Option<Vec<Overwrite>>,

    #[builder(default, setter(strip_option))]
    parent_id: Option<ChannelId>,

    #[builder(default, setter(strip_option, into))]
    rtc_region: Option<String>,

    #[builder(default, setter(strip_option, into))]
    video_quality_mode: Option<IntegerEnum<VideoQualityMode>>,

    #[builder(default, setter(strip_option))]
    archived: Option<bool>,

    #[builder(default, setter(strip_option))]
    auto_archive_duration: Option<u64>,

    #[builder(default, setter(strip_option))]
    locked: Option<bool>,
}

impl ModifyChannel {
    pub async fn send(self, discord: &Discord) -> Result<Channel, Error> {
        let path = format!("channels/{}", self.channel_id);

        let body = EditChannel {
            name: self.name,
            icon: self.icon,
            kind: self.kind,
            position: self.position,
            topic: self.topic,
            nsfw: self.nsfw,
            rate_limit_per_user: self.rate_limit_per_user,
            bitrate: self.bitrate,
            user_limit: self.user_limit,
            permission_overwrites: self.permission_overwrites,
            parent_id: self.parent_id,
            rtc_region: self.rtc_region,
            video_quality_mode: self.video_quality_mode,
            archived: self.archived,
            auto_archive_duration: self.auto_archive_duration,
            locked: self.locked,
        };

        discord.patch(path, &body).await
    }
}
