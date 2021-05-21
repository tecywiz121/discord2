// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod error;

use crate::resources::application::{
    ApplicationCommand, ApplicationCommandId, ApplicationCommandPermission,
    ApplicationId, EditApplicationCommand,
    EditGuildApplicationCommandPermissions, GuildApplicationCommandPermissions,
    NewApplicationCommand,
};
use crate::resources::audit_log::{AuditLog, AuditLogEntryId, AuditLogEvent};
use crate::resources::channel::{Channel, ChannelId, Message, MessageId};
use crate::resources::guild::GuildId;
use crate::resources::user::{User, UserId};
use crate::str::obscure;

use educe::Educe;

use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{ClientBuilder, Response, Url};

pub use self::error::Error;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use snafu::ResultExt;

use std::str::FromStr;

use typed_builder::TypedBuilder;

#[derive(Educe)]
#[educe(Debug)]
enum InnerToken {
    #[educe(Debug(named_field = false))]
    Bot {
        #[educe(Debug(method = "obscure"))]
        bot_token: String,
    },
    #[educe(Debug(named_field = false))]
    Bearer {
        #[educe(Debug(method = "obscure"))]
        bearer_token: String,
    },
}

#[derive(Debug)]
pub struct Token(InnerToken);

impl Token {
    pub fn bot(bot_token: String) -> Self {
        Self(InnerToken::Bot { bot_token })
    }

    pub fn bearer(bearer_token: String) -> Self {
        Self(InnerToken::Bearer { bearer_token })
    }

    fn to_header_value(&self) -> Result<HeaderValue, Error> {
        let (kind, token) = match &self.0 {
            InnerToken::Bot { bot_token } => ("Bot", bot_token),
            InnerToken::Bearer { bearer_token } => ("Bearer", bearer_token),
        };

        let text = format!("{} {}", kind, token);

        let mut value = HeaderValue::from_str(&text)?;
        value.set_sensitive(true);
        Ok(value)
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(doc)]
pub struct Config {
    token: Token,

    #[builder(default_code = "Config::DEFAULT_NAME.to_owned()")]
    name: String,

    #[builder(default_code = "Config::DEFAULT_URL.to_owned()")]
    url: String,

    #[builder(default_code = "Config::DEFAULT_VERSION.to_owned()")]
    version: String,

    #[builder(default_code = "Config::DEFAULT_API_ROOT.to_owned()")]
    api_root: String,
}

impl Config {
    const DEFAULT_NAME: &'static str = "RustDiscord2Bot";
    const DEFAULT_URL: &'static str = env!("CARGO_PKG_REPOSITORY");
    const DEFAULT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const DEFAULT_API_ROOT: &'static str = "https://discord.com/api/v9/";
}

#[derive(Debug, Deserialize)]
struct DiscordError {
    code: Option<u64>,
    message: Option<String>,
}

#[derive(Debug)]
pub struct Discord {
    api_root: Url,
    client: reqwest::Client,
}

impl Discord {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let api_root = Url::from_str(&config.api_root)
            .map_err(|e| Box::new(e) as Box<_>)
            .context(error::InvalidConfig)?;

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, config.token.to_header_value()?);

        let user_agent_txt =
            format!("{} ({}, {})", config.name, config.url, config.version,);
        let user_agent = HeaderValue::from_str(&user_agent_txt)?;

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(Self { api_root, client })
    }

    fn url<S>(&self, path: S) -> Url
    where
        S: AsRef<str>,
    {
        self.api_root.join(path.as_ref()).unwrap()
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        if response.status().is_success() {
            //let json: serde_json::Value = response.json().await?;
            //eprintln!("json: {}", json);
            //Ok(serde_json::from_value(json).unwrap())
            Ok(response.json().await?)
        } else {
            let err: DiscordError = response.json().await?;

            error::Discord {
                code: err.code,
                message: err.message,
            }
            .fail()
        }
    }

    async fn delete<S>(&self, path: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let url = self.url(path);
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let err: DiscordError = response.json().await?;

            error::Discord {
                code: err.code,
                message: err.message,
            }
            .fail()
        }
    }

    async fn patch<S, B, T>(&self, path: S, body: &B) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.url(path);
        let response = self.client.patch(url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn put<S, B, T>(&self, path: S, body: &B) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.url(path);
        let response = self.client.put(url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn post<S, B, T>(&self, path: S, body: &B) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.url(path);
        let response = self.client.post(url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn get<S, T>(&self, path: S) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let url = self.url(path);
        let response = self.client.get(url).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_global_application_commands(
        &self,
        application_id: ApplicationId,
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!("applications/{}/commands", application_id);
        self.get(path).await
    }

    pub async fn get_global_application_command(
        &self,
        application_id: ApplicationId,
        command_id: ApplicationCommandId,
    ) -> Result<ApplicationCommand, Error> {
        let path =
            format!("applications/{}/commands/{}", application_id, command_id);
        self.get(path).await
    }

    pub async fn create_all_global_application_commands(
        &self,
        application_id: ApplicationId,
        new_commands: &[NewApplicationCommand],
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!("applications/{}/commands", application_id);
        self.put(path, &new_commands).await
    }

    pub async fn create_global_application_command(
        &self,
        application_id: ApplicationId,
        new_command: &NewApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        let path = format!("applications/{}/commands", application_id);
        self.post(path, new_command).await
    }

    pub async fn edit_global_application_command(
        &self,
        application_id: ApplicationId,
        command_id: ApplicationCommandId,
        edit_command: &EditApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        let path =
            format!("applications/{}/commands/{}", application_id, command_id);
        self.patch(path, edit_command).await
    }

    pub async fn delete_global_application_command(
        &self,
        application_id: ApplicationId,
        command_id: ApplicationCommandId,
    ) -> Result<(), Error> {
        let path =
            format!("applications/{}/commands/{}", application_id, command_id);
        self.delete(path).await
    }

    pub async fn get_guild_application_commands(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands",
            application_id, guild_id
        );
        self.get(path).await
    }

    pub async fn create_guild_application_command(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        new_command: &NewApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands",
            application_id, guild_id
        );
        self.post(path, new_command).await
    }

    pub async fn get_guild_application_command(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: ApplicationCommandId,
    ) -> Result<ApplicationCommand, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}",
            application_id, guild_id, command_id
        );
        self.get(path).await
    }

    pub async fn edit_guild_application_command(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: ApplicationCommandId,
        edit_command: &EditApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}",
            application_id, guild_id, command_id
        );
        self.patch(path, edit_command).await
    }

    pub async fn delete_guild_application_command(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: ApplicationCommandId,
    ) -> Result<(), Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}",
            application_id, guild_id, command_id
        );
        self.delete(path).await
    }

    pub async fn create_all_guild_application_commands(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        new_commands: &[NewApplicationCommand],
    ) -> Result<Vec<ApplicationCommand>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands",
            application_id, guild_id
        );
        self.put(path, &new_commands).await
    }

    // TODO: create_interaction_response
    // TODO: get_original_interaction_response
    // TODO: edit_original_interaction_response
    // TODO: delete_original_interaction_response
    // TODO: create_followup_message
    // TODO: edit_followup_message
    // TODO: delete_followup_message

    pub async fn get_guild_application_command_permissions(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
    ) -> Result<Vec<GuildApplicationCommandPermissions>, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/permissions",
            application_id, guild_id
        );
        self.get(path).await
    }

    pub async fn get_application_command_permissions(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: ApplicationCommandId,
    ) -> Result<GuildApplicationCommandPermissions, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/{}/permissions",
            application_id, guild_id, command_id,
        );
        self.get(path).await
    }

    pub async fn edit_application_command_permissions(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: ApplicationCommandId,
        permissions: &[ApplicationCommandPermission],
    ) -> Result<GuildApplicationCommandPermissions, Error> {
        #[derive(Debug, Serialize)]
        struct Request<'a> {
            permissions: &'a [ApplicationCommandPermission],
        }

        let path = format!(
            "applications/{}/guilds/{}/commands/{}/permissions",
            application_id, guild_id, command_id,
        );

        self.put(path, &Request { permissions }).await
    }

    pub async fn edit_all_application_command_permissions(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        permissions: &[EditGuildApplicationCommandPermissions],
    ) -> Result<GuildApplicationCommandPermissions, Error> {
        let path = format!(
            "applications/{}/guilds/{}/commands/permissions",
            application_id, guild_id
        );

        self.put(path, &permissions).await
    }

    pub async fn get_guild_audit_log(
        &self,
        guild_id: GuildId,
        user_id: Option<UserId>,
        action_type: Option<AuditLogEvent>,
        before: Option<AuditLogEntryId>,
        limit: Option<u64>,
    ) -> Result<AuditLog, Error> {
        let mut path = format!("guilds/{}/audit-logs", guild_id);

        let user_id = user_id.map(|u| format!("user_id={}", u));
        let action_type =
            action_type.map(|u| format!("action_type={}", u64::from(u)));
        let before = before.map(|u| format!("before={}", u));
        let limit = limit.map(|u| format!("limit={}", u));

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

        self.get(path).await
    }

    pub async fn get_current_user(&self) -> Result<User, Error> {
        let path = "users/@me";
        self.get(path).await
    }

    pub async fn get_channel(
        &self,
        channel_id: ChannelId,
    ) -> Result<Channel, Error> {
        let url = self.url(format!("channels/{}", channel_id));
        let response = self.client.get(url).send().await?;

        self.handle_response(response).await
    }

    pub async fn get_channel_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<Message, Error> {
        let url = self
            .url(format!("channels/{}/messages/{}", channel_id, message_id));
        let response = self.client.get(url).send().await?;

        self.handle_response(response).await
    }
}
