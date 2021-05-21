// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod error {
    use snafu::{Backtrace, IntoError, Snafu};

    use super::RawAuditLogChange;

    #[derive(Debug, Snafu)]
    #[snafu(visibility = "pub(super)")]
    pub enum FromRawAuditLogChangeError {
        Deserialize {
            source: Box<dyn std::error::Error + 'static>,
            backtrace: Backtrace,
        },

        UnrecognizedKind {
            change: RawAuditLogChange,
        },
    }

    impl From<serde_json::Error> for FromRawAuditLogChangeError {
        fn from(err: serde_json::Error) -> Self {
            Deserialize {}.into_error(Box::new(err))
        }
    }
}

use crate::application::ApplicationId;
use crate::channel::{ChannelId, ChannelKind, MessageId, Overwrite};
use crate::enums::{
    EnumFromIntegerError, IntegerEnum, ParseEnumError, StringEnum,
};
use crate::guild::{
    DefaultMessageNotificationLevel, ExplicitContentFilterLevel,
    IntegrationAccount, IntegrationExpireBehavior, IntegrationId, MfaLevel,
    VerificationLevel,
};
use crate::permissions::RoleId;
use crate::snowflake::{AnyId, Id};
use crate::user::{User, UserId};
use crate::webhook::Webhook;

pub use self::error::FromRawAuditLogChangeError;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogRole {
    id: RoleId,
    name: String,
}

impl AuditLogRole {
    pub fn id(&self) -> RoleId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum EntityKind {
    Role,
    Member,
}

impl AsRef<str> for EntityKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Role => "0",
            Self::Member => "1",
        }
    }
}

impl FromStr for EntityKind {
    type Err = ParseEnumError;

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        let r = match txt {
            "0" => Self::Role,
            "1" => Self::Member,
            other => return Err(ParseEnumError::new(other.to_owned())),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntryInfo {
    delete_member_days: Option<String>,
    members_removed: Option<String>,
    channel_id: Option<ChannelId>,
    message_id: Option<MessageId>,
    count: Option<String>,
    id: Option<AnyId>,
    #[serde(rename = "type")]
    kind: Option<StringEnum<EntityKind>>,
    role_name: Option<String>,
}

impl AuditEntryInfo {
    pub fn delete_member_days(&self) -> Option<&str> {
        self.delete_member_days.as_deref()
    }

    pub fn members_removed(&self) -> Option<&str> {
        self.members_removed.as_deref()
    }

    pub fn channel_id(&self) -> Option<ChannelId> {
        self.channel_id
    }

    pub fn message_id(&self) -> Option<MessageId> {
        self.message_id
    }

    pub fn count(&self) -> Option<&str> {
        self.count.as_deref()
    }

    pub fn id(&self) -> Option<AnyId> {
        self.id
    }

    pub fn try_kind(&self) -> Option<Result<EntityKind, ParseEnumError>> {
        self.kind.as_ref().map(StringEnum::try_unwrap)
    }

    pub fn kind(&self) -> Option<EntityKind> {
        self.kind.as_ref().map(StringEnum::unwrap)
    }

    pub fn role_name(&self) -> Option<&str> {
        self.role_name.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditLogKindValue {
    ChannelKind(IntegerEnum<ChannelKind>),
    String(String),
}

#[derive(Debug, Clone, Copy, Hash)]
#[non_exhaustive]
pub struct AuditLogValues<T> {
    pub new: Option<T>,
    pub old: Option<T>,
}

impl<T> AuditLogValues<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn new(
        old: Option<serde_json::Value>,
        new: Option<serde_json::Value>,
    ) -> Result<Self, serde_json::Error> {
        let new = match new {
            Some(n) => Some(serde_json::from_value(n)?),
            None => None,
        };

        let old = match old {
            Some(n) => Some(serde_json::from_value(n)?),
            None => None,
        };

        Ok(Self { new, old })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawAuditLogChange {
    key: String,
    new_value: Option<serde_json::Value>,
    old_value: Option<serde_json::Value>,
}

impl RawAuditLogChange {
    pub fn key(&self) -> &str {
        &self.key
    }

    // TODO: Expose new_value and old_value sanely.
}

impl TryFrom<RawAuditLogChange> for AuditLogChange {
    type Error = FromRawAuditLogChangeError;

    fn try_from(alh: RawAuditLogChange) -> Result<AuditLogChange, Self::Error> {
        let r = match alh.key.as_str() {
            "name" => AuditLogChange::Name(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "description" => AuditLogChange::Description(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "icon_hash" => AuditLogChange::IconHash(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "splash_hash" => AuditLogChange::SplashHash(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "discovery_splash_hash" => AuditLogChange::DiscoverySplashHash(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "banner_hash" => AuditLogChange::BannerHash(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "owner_id" => AuditLogChange::OwnerId(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "region" => AuditLogChange::Region(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "preferred_locale" => AuditLogChange::PreferredLocale(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "afk_channel_id" => AuditLogChange::AfkChannelId(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "afk_timeout" => AuditLogChange::AfkTimeout(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "rules_channel_id" => AuditLogChange::RulesChannelId(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "public_updates_channel_id" => {
                AuditLogChange::PublicUpdatesChannelId(AuditLogValues::new(
                    alh.old_value,
                    alh.new_value,
                )?)
            }
            "mfa_level" => AuditLogChange::MfaLevel(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "verification_level" => AuditLogChange::VerificationLevel(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "explicit_content_filter" => AuditLogChange::ExplicitContentFilter(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "default_message_notifications" => {
                AuditLogChange::DefaultMessageNotifications(
                    AuditLogValues::new(alh.old_value, alh.new_value)?,
                )
            }
            "vanity_url_code" => AuditLogChange::VanityUrlCode(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "$add" => AuditLogChange::RoleAdd(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "$remove" => AuditLogChange::RoleRemove(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "prune_delete_days" => AuditLogChange::PruneDeleteDays(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "widget_enabled" => AuditLogChange::WidgetEnabled(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "widget_channel_id" => AuditLogChange::WidgetChannelId(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "system_channel_id" => AuditLogChange::SystemChannelId(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "position" => AuditLogChange::Position(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "topic" => AuditLogChange::Topic(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "bitrate" => AuditLogChange::Bitrate(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "permission_overwrites" => AuditLogChange::PermissionOverwrites(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "nsfw" => AuditLogChange::Nsfw(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "application_id" => AuditLogChange::ApplicationId(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "rate_limit_per_user" => AuditLogChange::RateLimitPerUser(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "permissions" => AuditLogChange::Permissions(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "color" => AuditLogChange::Color(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "hoist" => AuditLogChange::Hoist(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "mentionable" => AuditLogChange::Mentionable(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "allow" => AuditLogChange::Allow(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "deny" => AuditLogChange::Deny(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "code" => AuditLogChange::Code(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "channel_id" => AuditLogChange::ChannelId(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "inviter_id" => AuditLogChange::InviterId(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "max_uses" => AuditLogChange::MaxUses(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "uses" => AuditLogChange::Uses(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "max_age" => AuditLogChange::MaxAge(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "temporary" => AuditLogChange::Temporary(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "deaf" => AuditLogChange::Deaf(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "mute" => AuditLogChange::Mute(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "nick" => AuditLogChange::Nick(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "avatar_hash" => AuditLogChange::AvatarHash(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "id" => AuditLogChange::Id(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "type" => AuditLogChange::Kind(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),
            "enable_emoticons" => AuditLogChange::EnableEmoticons(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "expire_behavior" => AuditLogChange::ExpireBehavior(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "expire_grace_period" => AuditLogChange::ExpireGracePeriod(
                AuditLogValues::new(alh.old_value, alh.new_value)?,
            ),
            "user_limit" => AuditLogChange::UserLimit(AuditLogValues::new(
                alh.old_value,
                alh.new_value,
            )?),

            _ => return error::UnrecognizedKind { change: alh }.fail(),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "RawAuditLogChange")]
pub enum AuditLogChange {
    Name(AuditLogValues<String>),
    Description(AuditLogValues<String>),
    IconHash(AuditLogValues<String>),
    SplashHash(AuditLogValues<String>),
    DiscoverySplashHash(AuditLogValues<String>),
    BannerHash(AuditLogValues<String>),
    OwnerId(AuditLogValues<UserId>),
    Region(AuditLogValues<String>),
    PreferredLocale(AuditLogValues<String>),
    AfkChannelId(AuditLogValues<ChannelId>),
    AfkTimeout(AuditLogValues<u64>),
    RulesChannelId(AuditLogValues<ChannelId>),
    PublicUpdatesChannelId(AuditLogValues<ChannelId>),
    MfaLevel(AuditLogValues<IntegerEnum<MfaLevel>>),
    VerificationLevel(AuditLogValues<IntegerEnum<VerificationLevel>>),
    ExplicitContentFilter(
        AuditLogValues<IntegerEnum<ExplicitContentFilterLevel>>,
    ),
    DefaultMessageNotifications(
        AuditLogValues<IntegerEnum<DefaultMessageNotificationLevel>>,
    ),
    VanityUrlCode(AuditLogValues<String>),
    RoleAdd(AuditLogValues<AuditLogRole>),
    RoleRemove(AuditLogValues<AuditLogRole>),
    PruneDeleteDays(AuditLogValues<u64>),
    WidgetEnabled(AuditLogValues<bool>),
    WidgetChannelId(AuditLogValues<ChannelId>),
    SystemChannelId(AuditLogValues<ChannelId>),
    Position(AuditLogValues<u64>),
    Topic(AuditLogValues<String>),
    Bitrate(AuditLogValues<u64>),
    PermissionOverwrites(AuditLogValues<Vec<Overwrite>>),
    Nsfw(AuditLogValues<bool>),
    ApplicationId(AuditLogValues<ApplicationId>),
    RateLimitPerUser(AuditLogValues<u64>),
    Permissions(AuditLogValues<String>), // TODO: Type-ify
    Color(AuditLogValues<u32>),
    Hoist(AuditLogValues<bool>),
    Mentionable(AuditLogValues<bool>),
    Allow(AuditLogValues<String>), // TODO: Expand allow?
    Deny(AuditLogValues<String>),  // TODO: Expand deny?
    Code(AuditLogValues<String>),
    ChannelId(AuditLogValues<ChannelId>),
    InviterId(AuditLogValues<UserId>),
    MaxUses(AuditLogValues<u64>),
    Uses(AuditLogValues<u64>),
    MaxAge(AuditLogValues<u64>),
    Temporary(AuditLogValues<bool>),
    Deaf(AuditLogValues<bool>),
    Mute(AuditLogValues<bool>),
    Nick(AuditLogValues<String>),
    AvatarHash(AuditLogValues<String>),
    Id(AuditLogValues<AnyId>),
    Kind(AuditLogValues<AuditLogKindValue>),
    EnableEmoticons(AuditLogValues<bool>),
    ExpireBehavior(AuditLogValues<IntegerEnum<IntegrationExpireBehavior>>),
    ExpireGracePeriod(AuditLogValues<u64>),
    UserLimit(AuditLogValues<u64>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AuditLogEvent {
    GuildUpdate,

    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,

    ChannelOverwriteCreate,
    ChannelOverwriteUpdate,
    ChannelOverwriteDelete,

    MemberKick,
    MemberPrune,
    MemberBanAdd,
    MemberBanRemove,
    MemberUpdate,
    MemberRoleUpdate,
    MemberMove,
    MemberDisconnect,
    BotAdd,

    RoleCreate,
    RoleUpdate,
    RoleDelete,

    InviteCreate,
    InviteUpdate,
    InviteDelete,

    WebhookCreate,
    WebhookUpdate,
    WebhookDelete,

    EmojiCreate,
    EmojiUpdate,
    EmojiDelete,

    MessageDelete,
    MessageBulkDelete,
    MessagePin,
    MessageUnpin,

    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
}

impl TryFrom<u64> for AuditLogEvent {
    type Error = EnumFromIntegerError;

    fn try_from(evt: u64) -> Result<Self, Self::Error> {
        let r = match evt {
            1 => AuditLogEvent::GuildUpdate,

            10 => AuditLogEvent::ChannelCreate,
            11 => AuditLogEvent::ChannelUpdate,
            12 => AuditLogEvent::ChannelDelete,
            13 => AuditLogEvent::ChannelOverwriteCreate,
            14 => AuditLogEvent::ChannelOverwriteUpdate,
            15 => AuditLogEvent::ChannelOverwriteDelete,

            20 => AuditLogEvent::MemberKick,
            21 => AuditLogEvent::MemberPrune,
            22 => AuditLogEvent::MemberBanAdd,
            23 => AuditLogEvent::MemberBanRemove,
            24 => AuditLogEvent::MemberUpdate,
            25 => AuditLogEvent::MemberRoleUpdate,
            26 => AuditLogEvent::MemberMove,
            27 => AuditLogEvent::MemberDisconnect,
            28 => AuditLogEvent::BotAdd,

            30 => AuditLogEvent::RoleCreate,
            31 => AuditLogEvent::RoleUpdate,
            32 => AuditLogEvent::RoleDelete,

            40 => AuditLogEvent::InviteCreate,
            41 => AuditLogEvent::InviteUpdate,
            42 => AuditLogEvent::InviteDelete,

            50 => AuditLogEvent::WebhookCreate,
            51 => AuditLogEvent::WebhookUpdate,
            52 => AuditLogEvent::WebhookDelete,

            60 => AuditLogEvent::EmojiCreate,
            61 => AuditLogEvent::EmojiUpdate,
            62 => AuditLogEvent::EmojiDelete,

            72 => AuditLogEvent::MessageDelete,
            73 => AuditLogEvent::MessageBulkDelete,
            74 => AuditLogEvent::MessagePin,
            75 => AuditLogEvent::MessageUnpin,

            80 => AuditLogEvent::IntegrationCreate,
            81 => AuditLogEvent::IntegrationUpdate,
            82 => AuditLogEvent::IntegrationDelete,

            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<AuditLogEvent> for u64 {
    fn from(evt: AuditLogEvent) -> Self {
        match evt {
            AuditLogEvent::GuildUpdate => 1,

            AuditLogEvent::ChannelCreate => 10,
            AuditLogEvent::ChannelUpdate => 11,
            AuditLogEvent::ChannelDelete => 12,
            AuditLogEvent::ChannelOverwriteCreate => 13,
            AuditLogEvent::ChannelOverwriteUpdate => 14,
            AuditLogEvent::ChannelOverwriteDelete => 15,

            AuditLogEvent::MemberKick => 20,
            AuditLogEvent::MemberPrune => 21,
            AuditLogEvent::MemberBanAdd => 22,
            AuditLogEvent::MemberBanRemove => 23,
            AuditLogEvent::MemberUpdate => 24,
            AuditLogEvent::MemberRoleUpdate => 25,
            AuditLogEvent::MemberMove => 26,
            AuditLogEvent::MemberDisconnect => 27,
            AuditLogEvent::BotAdd => 28,

            AuditLogEvent::RoleCreate => 30,
            AuditLogEvent::RoleUpdate => 31,
            AuditLogEvent::RoleDelete => 32,

            AuditLogEvent::InviteCreate => 40,
            AuditLogEvent::InviteUpdate => 41,
            AuditLogEvent::InviteDelete => 42,

            AuditLogEvent::WebhookCreate => 50,
            AuditLogEvent::WebhookUpdate => 51,
            AuditLogEvent::WebhookDelete => 52,

            AuditLogEvent::EmojiCreate => 60,
            AuditLogEvent::EmojiUpdate => 61,
            AuditLogEvent::EmojiDelete => 62,

            AuditLogEvent::MessageDelete => 72,
            AuditLogEvent::MessageBulkDelete => 73,
            AuditLogEvent::MessagePin => 74,
            AuditLogEvent::MessageUnpin => 75,

            AuditLogEvent::IntegrationCreate => 80,
            AuditLogEvent::IntegrationUpdate => 81,
            AuditLogEvent::IntegrationDelete => 82,
        }
    }
}

pub type AuditLogEntryId = Id<AuditLogEntry>;

#[derive(Debug, Clone, Deserialize)]
pub struct AuditLogEntry {
    id: AuditLogEntryId,
    target_id: Option<AnyId>,
    user_id: Option<UserId>,
    changes: Option<Vec<AuditLogChange>>, // TODO: Expose RawAuditLogChange.
    #[serde(rename = "action_type")]
    action_kind: IntegerEnum<AuditLogEvent>,
    options: Option<AuditEntryInfo>,
    reason: Option<String>,
}

impl AuditLogEntry {
    pub fn id(&self) -> AuditLogEntryId {
        self.id
    }

    pub fn target_id(&self) -> Option<AnyId> {
        self.target_id
    }

    pub fn user_id(&self) -> Option<UserId> {
        self.user_id
    }

    pub fn changes(&self) -> Option<&[AuditLogChange]> {
        self.changes.as_deref()
    }

    pub fn try_action_kind(
        &self,
    ) -> Result<AuditLogEvent, EnumFromIntegerError> {
        self.action_kind.try_unwrap()
    }

    pub fn action_kind(&self) -> AuditLogEvent {
        self.action_kind.unwrap()
    }

    pub fn options(&self) -> Option<&AuditEntryInfo> {
        self.options.as_ref()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogIntegration {
    id: IntegrationId,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    account: IntegrationAccount,
}

impl AuditLogIntegration {
    pub fn id(&self) -> IntegrationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn account(&self) -> &IntegrationAccount {
        &self.account
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuditLog {
    webhooks: Vec<Webhook>,
    users: Vec<User>,
    audit_log_entries: Vec<AuditLogEntry>,
    integrations: Vec<AuditLogIntegration>,
}

impl AuditLog {
    pub fn webhooks(&self) -> &[Webhook] {
        &self.webhooks
    }

    pub fn users(&self) -> &[User] {
        &self.users
    }

    pub fn audit_log_entries(&self) -> &[AuditLogEntry] {
        &self.audit_log_entries
    }

    pub fn integrations(&self) -> &[AuditLogIntegration] {
        &self.integrations
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_audit_log_role() {
        let json = json!({
            "name": "I am a role",
            "id": "584120723283509258"
        });

        let role: AuditLogRole = serde_json::from_value(json).unwrap();

        assert_eq!(role.name(), "I am a role");
        assert_eq!(role.id(), 584120723283509258.into());
    }

    #[test]
    fn deserialize_audit_log_integration() {
        let json = json!({
            "id": "33590653072239123",
            "name": "A Name",
            "type": "twitch",
            "account": {
                "name": "twitchusername",
                "id": "1234567"
            }
        });

        let int: AuditLogIntegration = serde_json::from_value(json).unwrap();

        assert_eq!(int.id(), 33590653072239123.into());
        assert_eq!(int.name(), "A Name");
        assert_eq!(int.kind(), "twitch");
        assert_eq!(int.account().name(), "twitchusername");
        assert_eq!(int.account().id(), 1234567.into());
    }

    #[test]
    fn deserialize_audit_log() {
        let json = json!({
            "audit_log_entries": [
            {
                "action_type": 31,
                "changes": [
                {
                    "key": "permissions",
                    "new_value": "6546771521",
                    "old_value": "4399287873"
                }
                ],
                "id": "845138997059863333",
                "target_id": "843299980508444444",
                "user_id": "144232857852837888"
            },
            {
                "action_type": 74,
                "id": "843340438576666666",
                "options": {
                    "channel_id": "843299980508444444",
                    "message_id": "843340436517158932"
                },
                "target_id": "843299027126666666",
                "user_id": "843299027126666666"
            },
            {
                "action_type": 11,
                "changes": [
                {
                    "key": "topic",
                    "new_value": "jecehzjzzyzm6ovuwqthx78i8",
                    "old_value": "zntdmn9wsfhoxresszxmueun7"
                },
                {
                    "key": "name",
                    "new_value": "y5ce0w0v7tjog2dpi8ewmdthi",
                    "old_value": "yn7fexsrik59uu87qimoglptb"
                }
                ],
                "id": "843340115815311111",
                "target_id": "843299980508444444",
                "user_id": "843299027126666666"
            },
            {
                "action_type": 12,
                "changes": [
                {
                    "key": "name",
                    "old_value": "knybvzdqcj5gbiblwb6niltnw"
                },
                {
                    "key": "type",
                    "old_value": 0
                },
                {
                    "key": "permission_overwrites",
                    "old_value": []
                },
                {
                    "key": "nsfw",
                    "old_value": false
                },
                {
                    "key": "rate_limit_per_user",
                    "old_value": 0
                }
                ],
                "id": "843340114334583333",
                "target_id": "843340112879955555",
                "user_id": "843299027126666666"
            },
            {
                "action_type": 10,
                "changes": [
                {
                    "key": "name",
                    "new_value": "knybvzdqcj5gbiblwb6niltnw"
                },
                {
                    "key": "type",
                    "new_value": 0
                },
                {
                    "key": "permission_overwrites",
                    "new_value": []
                },
                {
                    "key": "nsfw",
                    "new_value": false
                },
                {
                    "key": "rate_limit_per_user",
                    "new_value": 0
                }
                ],
                "id": "843340113316413333",
                "target_id": "843340112877777777",
                "user_id": "843299027126666666"
            },
            {
                "action_type": 42,
                "changes": [
                {
                    "key": "code",
                    "old_value": "aAAaAAA"
                },
                {
                    "key": "channel_id",
                    "old_value": "843299980508444444"
                },
                {
                    "key": "inviter_id",
                    "old_value": "843299027126666666"
                },
                {
                    "key": "uses",
                    "old_value": 0
                },
                {
                    "key": "max_uses",
                    "old_value": 3
                },
                {
                    "key": "max_age",
                    "old_value": 500
                },
                {
                    "key": "temporary",
                    "old_value": false
                }
                ],
                "id": "843340112103700000",
                "target_id": null,
                "user_id": "843299027126666666"
            },
            {
                "action_type": 40,
                "changes": [
                {
                    "key": "code",
                    "new_value": "aAAaAAA"
                },
                {
                    "key": "channel_id",
                    "new_value": "843299980508444444"
                },
                {
                    "key": "inviter_id",
                    "new_value": "843299027126666666"
                },
                {
                    "key": "uses",
                    "new_value": 0
                },
                {
                    "key": "max_uses",
                    "new_value": 3
                },
                {
                    "key": "max_age",
                    "new_value": 500
                },
                {
                    "key": "temporary",
                    "new_value": false
                }
                ],
                "id": "843340110657777777",
                "target_id": null,
                "user_id": "843299027126666666"
            },
            ],
            "integrations": [],
            "users": [
            {
                "avatar": "162f914fb3f39a5cb344d20f40e744a8",
                "discriminator": "1234",
                "id": "144232812345678888",
                "public_flags": 0,
                "username": "some-user"
            },
            {
                "avatar": null,
                "bot": true,
                "discriminator": "4321",
                "id": "843299027126666666",
                "public_flags": 0,
                "username": "discord-next-testing"
            }
            ],
            "webhooks": []
        });

        let log: AuditLog = serde_json::from_value(json).unwrap();

        let entries = log.audit_log_entries();
        assert_eq!(entries.len(), 7);
        assert_eq!(entries[0].action_kind(), AuditLogEvent::RoleUpdate);
        assert_eq!(entries[0].id(), 845138997059863333.into());
        assert_eq!(entries[0].target_id(), Some(843299980508444444.into()));
        assert_eq!(entries[0].user_id(), Some(144232857852837888.into()));

        // TODO: More thorough asserts on change new_value/old_value.

        let changes = entries[0].changes().unwrap();
        assert_eq!(changes.len(), 1);
        assert_matches!(changes[0], AuditLogChange::Permissions(_));

        assert_eq!(entries[1].action_kind(), AuditLogEvent::MessagePin);
        assert_eq!(entries[1].id(), 843340438576666666.into());
        assert_eq!(entries[1].target_id(), Some(843299027126666666.into()));
        assert_eq!(entries[1].user_id(), Some(843299027126666666.into()));
        assert!(entries[1].changes().is_none());

        let options = entries[1].options().unwrap();
        assert_eq!(options.channel_id(), Some(843299980508444444.into()));
        assert_eq!(options.message_id(), Some(843340436517158932.into()));

        assert_eq!(entries[2].action_kind(), AuditLogEvent::ChannelUpdate);
        assert_eq!(entries[2].id(), 843340115815311111.into());
        assert_eq!(entries[2].target_id(), Some(843299980508444444.into()));
        assert_eq!(entries[2].user_id(), Some(843299027126666666.into()));

        let changes = entries[2].changes().unwrap();
        assert_eq!(changes.len(), 2);
        assert_matches!(changes[0], AuditLogChange::Topic(_));
        assert_matches!(changes[1], AuditLogChange::Name(_));

        assert_eq!(entries[3].action_kind(), AuditLogEvent::ChannelDelete);
        assert_eq!(entries[3].id(), 843340114334583333.into());
        assert_eq!(entries[3].target_id(), Some(843340112879955555.into()));
        assert_eq!(entries[3].user_id(), Some(843299027126666666.into()));

        let changes = entries[3].changes().unwrap();
        assert_eq!(changes.len(), 5);
        assert_matches!(changes[0], AuditLogChange::Name(_));
        assert_matches!(changes[1], AuditLogChange::Kind(_));
        assert_matches!(changes[2], AuditLogChange::PermissionOverwrites(_));
        assert_matches!(changes[3], AuditLogChange::Nsfw(_));
        assert_matches!(changes[4], AuditLogChange::RateLimitPerUser(_));

        assert_eq!(entries[4].action_kind(), AuditLogEvent::ChannelCreate);
        assert_eq!(entries[4].id(), 843340113316413333.into());
        assert_eq!(entries[4].target_id(), Some(843340112877777777.into()));
        assert_eq!(entries[4].user_id(), Some(843299027126666666.into()));

        let changes = entries[4].changes().unwrap();
        assert_eq!(changes.len(), 5);
        assert_matches!(changes[0], AuditLogChange::Name(_));
        assert_matches!(changes[1], AuditLogChange::Kind(_));
        assert_matches!(changes[2], AuditLogChange::PermissionOverwrites(_));
        assert_matches!(changes[3], AuditLogChange::Nsfw(_));
        assert_matches!(changes[4], AuditLogChange::RateLimitPerUser(_));

        assert_eq!(entries[5].action_kind(), AuditLogEvent::InviteDelete);
        assert_eq!(entries[5].id(), 843340112103700000.into());
        assert_eq!(entries[5].target_id(), None);
        assert_eq!(entries[5].user_id(), Some(843299027126666666.into()));

        let changes = entries[5].changes().unwrap();
        assert_eq!(changes.len(), 7);
        assert_matches!(changes[0], AuditLogChange::Code(_));
        assert_matches!(changes[1], AuditLogChange::ChannelId(_));
        assert_matches!(changes[2], AuditLogChange::InviterId(_));
        assert_matches!(changes[3], AuditLogChange::Uses(_));
        assert_matches!(changes[4], AuditLogChange::MaxUses(_));
        assert_matches!(changes[5], AuditLogChange::MaxAge(_));
        assert_matches!(changes[6], AuditLogChange::Temporary(_));

        assert_eq!(entries[6].action_kind(), AuditLogEvent::InviteCreate);
        assert_eq!(entries[6].id(), 843340110657777777.into());
        assert_eq!(entries[6].target_id(), None);
        assert_eq!(entries[6].user_id(), Some(843299027126666666.into()));

        let changes = entries[6].changes().unwrap();
        assert_eq!(changes.len(), 7);
        assert_matches!(changes[0], AuditLogChange::Code(_));
        assert_matches!(changes[1], AuditLogChange::ChannelId(_));
        assert_matches!(changes[2], AuditLogChange::InviterId(_));
        assert_matches!(changes[3], AuditLogChange::Uses(_));
        assert_matches!(changes[4], AuditLogChange::MaxUses(_));
        assert_matches!(changes[5], AuditLogChange::MaxAge(_));
        assert_matches!(changes[6], AuditLogChange::Temporary(_));

        assert!(log.integrations().is_empty());
        assert!(log.webhooks().is_empty());

        let users = log.users();
        assert_eq!(users.len(), 2);
    }
}
