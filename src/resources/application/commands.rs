// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use crate::enums::{EnumFromIntegerError, IntegerEnum};
use crate::permissions::RoleId;
use crate::resources::channel::{AllowedMentions, Embed};
use crate::resources::guild::GuildId;
use crate::resources::user::UserId;
use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use super::ApplicationId;

use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChoiceValue {
    Integer(u64),
    String(String),
}

impl ChoiceValue {
    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(u) => Some(u),
            _ => None,
        }
    }

    pub fn into_u64(self) -> Option<u64> {
        match self {
            Self::Integer(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Integer(u) => Some(*u),
            _ => None,
        }
    }
}

impl From<&str> for ChoiceValue {
    fn from(u: &str) -> Self {
        Self::String(u.to_owned())
    }
}

impl From<String> for ChoiceValue {
    fn from(u: String) -> Self {
        Self::String(u)
    }
}

impl From<u64> for ChoiceValue {
    fn from(u: u64) -> Self {
        Self::Integer(u)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ApplicationCommandOptionChoice {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    value: ChoiceValue,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ApplicationCommandOptionKind {
    SubCommand,
    SubCommandGroup,
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
}

impl From<ApplicationCommandOptionKind> for u64 {
    fn from(u: ApplicationCommandOptionKind) -> Self {
        match u {
            ApplicationCommandOptionKind::SubCommand => 1,
            ApplicationCommandOptionKind::SubCommandGroup => 2,
            ApplicationCommandOptionKind::String => 3,
            ApplicationCommandOptionKind::Integer => 4,
            ApplicationCommandOptionKind::Boolean => 5,
            ApplicationCommandOptionKind::User => 6,
            ApplicationCommandOptionKind::Channel => 7,
            ApplicationCommandOptionKind::Role => 8,
            ApplicationCommandOptionKind::Mentionable => 9,
        }
    }
}

impl TryFrom<u64> for ApplicationCommandOptionKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::SubCommand,
            2 => Self::SubCommandGroup,
            3 => Self::String,
            4 => Self::Integer,
            5 => Self::Boolean,
            6 => Self::User,
            7 => Self::Channel,
            8 => Self::Role,
            9 => Self::Mentionable,

            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ApplicationCommandOption {
    #[builder(setter(into))]
    #[serde(rename = "type")]
    kind: IntegerEnum<ApplicationCommandOptionKind>,

    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    description: String,

    #[builder(default, setter(strip_option))]
    required: Option<bool>,

    #[builder(default, setter(into, strip_option))]
    choices: Option<Vec<ApplicationCommandOptionChoice>>,

    #[builder(default, setter(into, strip_option))]
    options: Option<Vec<ApplicationCommandOption>>,
}

pub type ApplicationCommandId = Id<ApplicationCommand>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommand {
    id: ApplicationCommandId,
    application_id: ApplicationId,
    name: String,
    description: String,
    options: Option<Vec<ApplicationCommandOption>>,
    default_permission: Option<bool>,
}

impl ApplicationCommand {
    pub fn id(&self) -> ApplicationCommandId {
        self.id
    }

    pub fn application_id(&self) -> ApplicationId {
        self.application_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn options(&self) -> Option<&[ApplicationCommandOption]> {
        self.options.as_deref()
    }

    pub fn default_permission(&self) -> Option<bool> {
        self.default_permission
    }
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct NewApplicationCommand {
    #[builder(setter(into))]
    pub(crate) name: String,

    #[builder(setter(into))]
    pub(crate) description: String,

    #[builder(default, setter(strip_option, into))]
    pub(crate) options: Option<Vec<ApplicationCommandOption>>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) default_permission: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct EditApplicationCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    #[builder(setter(into))]
    kind: IntegerEnum<InteractionCallbackKind>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<InteractionApplicationCommandCallbackData>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InteractionCallbackKind {
    Pong,
    ChannelMessageWithSource,
    DeferredChannelMessageWithSource,
}

impl From<InteractionCallbackKind> for u64 {
    fn from(kind: InteractionCallbackKind) -> u64 {
        match kind {
            InteractionCallbackKind::Pong => 1,
            InteractionCallbackKind::ChannelMessageWithSource => 4,
            InteractionCallbackKind::DeferredChannelMessageWithSource => 5,
        }
    }
}

impl TryFrom<u64> for InteractionCallbackKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => InteractionCallbackKind::Pong,
            4 => InteractionCallbackKind::ChannelMessageWithSource,
            5 => InteractionCallbackKind::DeferredChannelMessageWithSource,

            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct InteractionApplicationCommandCallbackData {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<AllowedMentions>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<IntegerEnum<InteractionCallbackFlags>>,
}

bitflags! {
    pub struct InteractionCallbackFlags: u64 {
        const EPHEMERAL = 1<<6;
    }
}

impl TryFrom<u64> for InteractionCallbackFlags {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Self::from_bits(u).ok_or_else(|| Self::Error::new(u))
    }
}

impl From<InteractionCallbackFlags> for u64 {
    fn from(uf: InteractionCallbackFlags) -> u64 {
        uf.bits()
    }
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct EditGuildApplicationCommandPermissions {
    #[builder(setter(into))]
    id: ApplicationCommandId,

    #[builder(setter(into))]
    permissions: Vec<ApplicationCommandPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildApplicationCommandPermissions {
    id: ApplicationCommandId,
    application_id: ApplicationId,
    guild_id: GuildId,
    permissions: Vec<ApplicationCommandPermission>,
}

impl GuildApplicationCommandPermissions {
    pub fn id(&self) -> ApplicationCommandId {
        self.id
    }

    pub fn application_id(&self) -> ApplicationId {
        self.application_id
    }

    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    pub fn permissions(&self) -> &[ApplicationCommandPermission] {
        &self.permissions
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct CmdPermIdHelper {
    id: Id<()>,
    #[serde(rename = "type")]
    kind: u64,
}

impl From<CommandPermissionId> for CmdPermIdHelper {
    fn from(cpi: CommandPermissionId) -> Self {
        match cpi {
            CommandPermissionId::Role(rid) => Self {
                id: u64::from(rid).into(),
                kind: 1,
            },
            CommandPermissionId::User(uid) => Self {
                id: u64::from(uid).into(),
                kind: 2,
            },
        }
    }
}

impl From<CmdPermIdHelper> for CommandPermissionId {
    fn from(cpi: CmdPermIdHelper) -> Self {
        match cpi {
            CmdPermIdHelper { id, kind: 1 } => Self::Role(u64::from(id).into()),
            CmdPermIdHelper { id, kind: 2 } => Self::User(u64::from(id).into()),
            _ => panic!("unsupported command permission id"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "CmdPermIdHelper", into = "CmdPermIdHelper")]
pub enum CommandPermissionId {
    Role(RoleId),
    User(UserId),
}

impl CommandPermissionId {
    pub fn is_user(self) -> bool {
        matches!(self, Self::User(_))
    }

    pub fn is_role(self) -> bool {
        matches!(self, Self::Role(_))
    }
}

impl From<UserId> for CommandPermissionId {
    fn from(uid: UserId) -> Self {
        CommandPermissionId::User(uid)
    }
}

impl From<RoleId> for CommandPermissionId {
    fn from(rid: RoleId) -> Self {
        CommandPermissionId::Role(rid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ApplicationCommandPermission {
    #[builder(setter(into))]
    #[serde(flatten)]
    id: CommandPermissionId,
    permission: bool,
}

impl ApplicationCommandPermission {
    pub fn id(&self) -> CommandPermissionId {
        self.id
    }

    pub fn permission(&self) -> bool {
        self.permission
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_application_command_permission_user() {
        let json = json!({
            "id": 172150183260323840u64,
            "type": 2,
            "permission": true,
        });

        let acp: ApplicationCommandPermission =
            serde_json::from_value(json).unwrap();

        assert_eq!(
            acp.id(),
            CommandPermissionId::User(172150183260323840.into())
        );
        assert_eq!(acp.permission(), true);
    }

    #[test]
    fn deserialize_application_command_permission_role() {
        let json = json!({
            "id": "172150183260323840",
            "type": 1,
            "permission": true,
        });

        let acp: ApplicationCommandPermission =
            serde_json::from_value(json).unwrap();

        assert_eq!(
            acp.id(),
            CommandPermissionId::Role(172150183260323840.into())
        );
        assert_eq!(acp.permission(), true);
    }

    #[test]
    fn deserialize_guild_application_command_permissions() {
        let json = json!({
            "application_id": "658822586720976555",
            "guild_id": "41771983429143937",
            "id": "61771983423143937",
            "permissions": [
                {
                    "id": "658822586720976555",
                    "permission": false,
                    "type": 2
                }
            ]
        });

        let perms: GuildApplicationCommandPermissions =
            serde_json::from_value(json).unwrap();

        assert_eq!(perms.application_id(), 658822586720976555.into());
        assert_eq!(perms.guild_id(), 41771983429143937.into());
        assert_eq!(perms.id(), 61771983423143937.into());

        let items = perms.permissions();
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].id(),
            CommandPermissionId::User(658822586720976555.into())
        );
        assert_eq!(items[0].permission(), false);
    }
}
