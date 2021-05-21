// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod embed;
mod message;

use bitflags::bitflags;

use chrono::{DateTime, FixedOffset};

use crate::enums::{EnumFromIntegerError, IntegerEnum};
use crate::permissions::RoleId;
use crate::resources::application::ApplicationId;
use crate::resources::guild::GuildId;
use crate::resources::user::{User, UserId};
use crate::snowflake::Id;

pub use self::embed::*;
pub use self::message::*;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMetadata {
    archived: bool,
    archiver_id: Option<UserId>,
    auto_archive_duration: u64,
    archive_timestamp: DateTime<FixedOffset>,
    locked: Option<bool>,
}

impl ThreadMetadata {
    pub fn archived(&self) -> bool {
        self.archived
    }

    pub fn archiver_id(&self) -> Option<UserId> {
        self.archiver_id
    }

    pub fn auto_archive_duration(&self) -> u64 {
        self.auto_archive_duration
    }

    pub fn archive_timestamp(&self) -> DateTime<FixedOffset> {
        self.archive_timestamp
    }

    pub fn locked(&self) -> Option<bool> {
        self.locked
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMember {
    id: ChannelId,
    user_id: UserId,
    join_timestamp: DateTime<FixedOffset>,
    flags: IntegerEnum<ThreadMemberFlags>,
}

impl ThreadMember {
    pub fn id(&self) -> ChannelId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn join_timestamp(&self) -> DateTime<FixedOffset> {
        self.join_timestamp
    }

    pub fn try_flags(&self) -> Result<ThreadMemberFlags, EnumFromIntegerError> {
        self.flags.try_unwrap()
    }

    pub fn flags(&self) -> ThreadMemberFlags {
        self.flags.unwrap()
    }
}

bitflags! {
    pub struct ThreadMemberFlags: u64 {
        const NONE = 0;
    }
}

impl TryFrom<u64> for ThreadMemberFlags {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Self::from_bits(u).ok_or_else(|| Self::Error::new(u))
    }
}

impl From<ThreadMemberFlags> for u64 {
    fn from(tmf: ThreadMemberFlags) -> u64 {
        tmf.bits()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum VideoQualityMode {
    Auto,
    Full,
}

impl TryFrom<u64> for VideoQualityMode {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::Auto,
            2 => Self::Full,
            raw => return Err(EnumFromIntegerError::new(raw)),
        };

        Ok(r)
    }
}

impl From<VideoQualityMode> for u64 {
    fn from(v: VideoQualityMode) -> Self {
        match v {
            VideoQualityMode::Auto => 1,
            VideoQualityMode::Full => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct OverwriteIdHelper {
    id: Id<()>,
    #[serde(rename = "type")]
    kind: u64,
}

impl From<OverwriteId> for OverwriteIdHelper {
    fn from(oid: OverwriteId) -> Self {
        match oid {
            OverwriteId::Role(rid) => Self {
                id: u64::from(rid).into(),
                kind: 0,
            },
            OverwriteId::Member(uid) => Self {
                id: u64::from(uid).into(),
                kind: 1,
            },
        }
    }
}

impl From<OverwriteIdHelper> for OverwriteId {
    fn from(oih: OverwriteIdHelper) -> Self {
        match oih {
            OverwriteIdHelper { id, kind: 0 } => {
                Self::Role(u64::from(id).into())
            }
            OverwriteIdHelper { id, kind: 1 } => {
                Self::Member(u64::from(id).into())
            }
            _ => panic!("unsupported overwrite id"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(into = "OverwriteIdHelper", from = "OverwriteIdHelper")]
pub enum OverwriteId {
    Role(RoleId),
    Member(UserId),
}

impl OverwriteId {
    pub fn is_member(self) -> bool {
        matches!(self, Self::Member(_))
    }

    pub fn is_role(self) -> bool {
        matches!(self, Self::Role(_))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overwrite {
    #[serde(flatten)]
    id: OverwriteId,
    allow: String,
    deny: String,
}

impl Overwrite {
    pub fn id(&self) -> OverwriteId {
        self.id
    }

    pub fn allow(&self) -> &str {
        &self.allow
    }

    pub fn deny(&self) -> &str {
        &self.deny
    }

    // TODO: Expand allow/deny
}

pub type ChannelId = Id<Channel>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ChannelKind {
    GuildText,
    Dm,
    GuildVoice,
    GroupDm,
    GuildCategory,
    GuildNews,
    GuildStore,
    GuildNewsThread,
    GuildPublicThread,
    GuildPrivateThread,
    GuildStageVoice,
}

impl From<ChannelKind> for u64 {
    fn from(u: ChannelKind) -> Self {
        match u {
            ChannelKind::GuildText => 0,
            ChannelKind::Dm => 1,
            ChannelKind::GuildVoice => 2,
            ChannelKind::GroupDm => 3,
            ChannelKind::GuildCategory => 4,
            ChannelKind::GuildNews => 5,
            ChannelKind::GuildStore => 6,
            ChannelKind::GuildNewsThread => 10,
            ChannelKind::GuildPublicThread => 11,
            ChannelKind::GuildPrivateThread => 12,
            ChannelKind::GuildStageVoice => 13,
        }
    }
}

impl TryFrom<u64> for ChannelKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            0 => Self::GuildText,
            1 => Self::Dm,
            2 => Self::GuildVoice,
            3 => Self::GroupDm,
            4 => Self::GuildCategory,
            5 => Self::GuildNews,
            6 => Self::GuildStore,
            10 => Self::GuildNewsThread,
            11 => Self::GuildPublicThread,
            12 => Self::GuildPrivateThread,
            13 => Self::GuildStageVoice,
            raw => return Err(EnumFromIntegerError::new(raw)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    id: ChannelId,
    #[serde(rename = "type")]
    kind: Option<IntegerEnum<ChannelKind>>,
    guild_id: Option<GuildId>,
    position: Option<u64>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<MessageId>,
    bitrate: Option<u64>,
    user_limit: Option<u64>,
    rate_limit_per_user: Option<u64>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<UserId>,
    application_id: Option<ApplicationId>,
    parent_id: Option<ChannelId>,
    last_pin_timestamp: Option<DateTime<FixedOffset>>,
    rtc_region: Option<String>,
    video_quality_mode: Option<IntegerEnum<VideoQualityMode>>,
    message_count: Option<u64>,
    member_count: Option<u64>,
    thread_metadata: Option<ThreadMetadata>,
    member: Option<ThreadMember>,
}

impl Channel {
    pub fn id(&self) -> ChannelId {
        self.id
    }

    pub fn try_kind(
        &self,
    ) -> Option<Result<ChannelKind, EnumFromIntegerError>> {
        self.kind.map(IntegerEnum::try_unwrap)
    }

    pub fn kind(&self) -> Option<ChannelKind> {
        self.kind.map(IntegerEnum::unwrap)
    }

    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    pub fn position(&self) -> Option<u64> {
        self.position
    }

    pub fn permission_overwrites(&self) -> Option<&[Overwrite]> {
        self.permission_overwrites.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_ref)
    }

    pub fn topic(&self) -> Option<&str> {
        self.topic.as_ref().map(String::as_ref)
    }

    pub fn nsfw(&self) -> Option<bool> {
        self.nsfw
    }

    pub fn last_message_id(&self) -> Option<MessageId> {
        self.last_message_id
    }

    pub fn bitrate(&self) -> Option<u64> {
        self.bitrate
    }

    pub fn user_limit(&self) -> Option<u64> {
        self.user_limit
    }

    pub fn rate_limit_per_user(&self) -> Option<u64> {
        self.rate_limit_per_user
    }

    pub fn recipients(&self) -> Option<&[User]> {
        self.recipients.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(String::as_ref)
    }

    pub fn owner_id(&self) -> Option<UserId> {
        self.owner_id
    }

    pub fn last_pin_timestamp(&self) -> Option<DateTime<FixedOffset>> {
        self.last_pin_timestamp
    }

    pub fn rtc_region(&self) -> Option<&str> {
        self.rtc_region.as_ref().map(String::as_ref)
    }

    pub fn try_video_quality_mode(
        &self,
    ) -> Option<Result<VideoQualityMode, EnumFromIntegerError>> {
        self.video_quality_mode.map(IntegerEnum::try_unwrap)
    }

    pub fn video_quality_mode(&self) -> Option<VideoQualityMode> {
        self.video_quality_mode.map(IntegerEnum::unwrap)
    }

    pub fn message_count(&self) -> Option<u64> {
        self.message_count
    }

    pub fn member_count(&self) -> Option<u64> {
        self.member_count
    }

    pub fn thread_metadata(&self) -> Option<&ThreadMetadata> {
        self.thread_metadata.as_ref()
    }

    pub fn member(&self) -> Option<&ThreadMember> {
        self.member.as_ref()
    }

    pub fn parent_id(&self) -> Option<ChannelId> {
        self.parent_id
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use serde_json::json;

    use super::*;

    #[test]
    fn channel_deserialize_store() {
        let json = json!({
            "id": "41771983423143937",
            "guild_id": "41771983429143937",
            "name": "buy dota-2",
            "type": 6,
            "position": 0,
            "permission_overwrites": [],
            "nsfw": false,
            "parent_id": null
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 41771983423143937.into());
        assert_eq!(channel.guild_id(), Some(41771983429143937.into()));
        assert_eq!(channel.name(), Some("buy dota-2"));
        assert_eq!(channel.kind(), Some(ChannelKind::GuildStore));
        assert_eq!(channel.position(), Some(0));
        assert!(channel.permission_overwrites().unwrap().is_empty());
        assert_eq!(channel.nsfw(), Some(false));
        assert_eq!(channel.parent_id(), None);
    }

    #[test]
    fn channel_deserialize_category() {
        let json = json!({
            "permission_overwrites": [],
            "name": "Test",
            "parent_id": null,
            "nsfw": false,
            "position": 0,
            "guild_id": "290926798629997250",
            "type": 4,
            "id": "399942396007890945"
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 399942396007890945.into());
        assert_eq!(channel.guild_id(), Some(290926798629997250.into()));
        assert_eq!(channel.name(), Some("Test"));
        assert_eq!(channel.kind(), Some(ChannelKind::GuildCategory));
        assert_eq!(channel.position(), Some(0));
        assert!(channel.permission_overwrites().unwrap().is_empty());
        assert_eq!(channel.nsfw(), Some(false));
        assert_eq!(channel.parent_id(), None);
    }

    #[test]
    fn channel_deserialize_group_dm() {
        let json = json!({
            "name": "Some test channel",
            "icon": null,
            "recipients": [
            {
                "username": "test",
                "discriminator": "9999",
                "id": "82198898841029460",
                "avatar": "33ecab261d4681afa4d85a04691c4a01"
            },
            {
                "username": "test2",
                "discriminator": "9999",
                "id": "82198810841029460",
                "avatar": "33ecab261d4681afa4d85a10691c4a01"
            }
            ],
            "last_message_id": "3343820033257021450",
            "type": 3,
            "id": "319674150115710528",
            "owner_id": "82198810841029460"
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 319674150115710528.into());
        assert_eq!(channel.guild_id(), None);
        assert_eq!(channel.name(), Some("Some test channel"));
        assert_eq!(channel.icon(), None);
        assert_eq!(channel.kind(), Some(ChannelKind::GroupDm));
        assert_eq!(channel.parent_id(), None);
        assert_eq!(channel.last_message_id(), Some(3343820033257021450.into()));
        assert_eq!(channel.owner_id(), Some(82198810841029460.into()));

        let recipients = channel.recipients().unwrap();
        assert_eq!(recipients.len(), 2);

        assert_eq!(recipients[0].username(), "test");
        assert_eq!(recipients[0].discriminator(), "9999");
        assert_eq!(recipients[0].id(), 82198898841029460.into());
        assert_eq!(
            recipients[0].avatar(),
            Some("33ecab261d4681afa4d85a04691c4a01")
        );

        assert_eq!(recipients[1].username(), "test2");
        assert_eq!(recipients[1].discriminator(), "9999");
        assert_eq!(recipients[1].id(), 82198810841029460.into());
        assert_eq!(
            recipients[1].avatar(),
            Some("33ecab261d4681afa4d85a10691c4a01")
        );
    }

    #[test]
    fn channel_deserialize_dm() {
        let json = json!({
            "last_message_id": "3343820033257021450",
            "type": 1,
            "id": "319674150115610528",
            "recipients": [
            {
                "username": "test",
                "discriminator": "9999",
                "id": "82198898841029460",
                "avatar": "33ecab261d4681afa4d85a04691c4a01"
            }
            ]
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 319674150115610528.into());
        assert_eq!(channel.guild_id(), None);
        assert_eq!(channel.name(), None);
        assert_eq!(channel.icon(), None);

        assert_eq!(channel.kind(), Some(ChannelKind::Dm));
        assert_eq!(channel.parent_id(), None);
        assert_eq!(channel.last_message_id(), Some(3343820033257021450.into()));
        assert_eq!(channel.owner_id(), None);

        let recipients = channel.recipients().unwrap();
        assert_eq!(recipients.len(), 1);

        assert_eq!(recipients[0].username(), "test");
        assert_eq!(recipients[0].discriminator(), "9999");
        assert_eq!(recipients[0].id(), 82198898841029460.into());
        assert_eq!(
            recipients[0].avatar(),
            Some("33ecab261d4681afa4d85a04691c4a01")
        );
    }

    #[test]
    fn channel_deserialize_voice() {
        let json = json!({
            "id": "155101607195836416",
            "guild_id": "41771983423143937",
            "name": "ROCKET CHEESE",
            "type": 2,
            "nsfw": false,
            "position": 5,
            "permission_overwrites": [],
            "bitrate": 64000,
            "user_limit": 0,
            "parent_id": null,
            "rtc_region": null
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 155101607195836416.into());
        assert_eq!(channel.guild_id(), Some(41771983423143937.into()));
        assert_eq!(channel.name(), Some("ROCKET CHEESE"));
        assert_eq!(channel.icon(), None);

        assert!(channel.recipients().is_none());

        assert_eq!(channel.kind(), Some(ChannelKind::GuildVoice));
        assert_eq!(channel.parent_id(), None);
        assert_eq!(channel.last_message_id(), None);
        assert_eq!(channel.owner_id(), None);
        assert_eq!(channel.nsfw(), Some(false));
        assert_eq!(channel.bitrate(), Some(64000));
        assert_eq!(channel.user_limit(), Some(0));
        assert_eq!(channel.rtc_region(), None);
        assert_eq!(channel.position(), Some(5));
    }

    #[test]
    fn channel_deserialize_news() {
        let json = json!({
            "id": "41771983423143937",
            "guild_id": "41771983423143937",
            "name": "important-news",
            "type": 5,
            "position": 6,
            "permission_overwrites": [],
            "nsfw": true,
            "topic": "Rumors about Half Life 3",
            "last_message_id": "155117677105512449",
            "parent_id": "399942396007890945"
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 41771983423143937.into());
        assert_eq!(channel.guild_id(), Some(41771983423143937.into()));
        assert_eq!(channel.name(), Some("important-news"));
        assert_eq!(channel.icon(), None);

        assert!(channel.recipients().is_none());

        assert_eq!(channel.kind(), Some(ChannelKind::GuildNews));
        assert_eq!(channel.parent_id(), Some(399942396007890945.into()));
        assert_eq!(channel.last_message_id(), Some(155117677105512449.into()));
        assert_eq!(channel.owner_id(), None);
        assert_eq!(channel.nsfw(), Some(true));
        assert_eq!(channel.bitrate(), None);
        assert_eq!(channel.user_limit(), None);
        assert_eq!(channel.rtc_region(), None);
        assert_eq!(channel.position(), Some(6));
        assert_eq!(channel.topic(), Some("Rumors about Half Life 3"));
    }

    #[test]
    fn channel_deserialize_text() {
        let json = json!({
            "id": "41771983423143937",
            "guild_id": "41771983423143937",
            "name": "general",
            "type": 0,
            "position": 6,
            "permission_overwrites": [],
            "rate_limit_per_user": 2,
            "nsfw": true,
            "topic": "24/7 chat about how to gank Mike #2",
            "last_message_id": "155117677105512449",
            "parent_id": "399942396007890945"
        });

        let channel: Channel = serde_json::from_value(json).unwrap();

        assert_eq!(channel.id(), 41771983423143937.into());
        assert_eq!(channel.guild_id(), Some(41771983423143937.into()));
        assert_eq!(channel.name(), Some("general"));
        assert_eq!(channel.icon(), None);

        assert!(channel.recipients().is_none());

        assert_eq!(channel.kind(), Some(ChannelKind::GuildText));
        assert_eq!(channel.parent_id(), Some(399942396007890945.into()));
        assert_eq!(channel.last_message_id(), Some(155117677105512449.into()));
        assert_eq!(channel.owner_id(), None);
        assert_eq!(channel.nsfw(), Some(true));
        assert_eq!(channel.bitrate(), None);
        assert_eq!(channel.user_limit(), None);
        assert_eq!(channel.rtc_region(), None);
        assert_eq!(channel.position(), Some(6));
        assert_eq!(
            channel.topic(),
            Some("24/7 chat about how to gank Mike #2")
        );
        assert_eq!(channel.rate_limit_per_user(), Some(2));
    }

    #[test]
    fn message_deserialize() {
        let json = json!({
            "reactions": [
            {
                "count": 1,
                "me": false,
                "emoji": {
                    "id": null,
                    "name": "🔥"
                }
            }
            ],
            "attachments": [],
            "tts": false,
            "embeds": [],
            "timestamp": "2017-07-11T17:27:07.299000+00:00",
            "mention_everyone": false,
            "id": "334385199974967042",
            "pinned": false,
            "edited_timestamp": null,
            "author": {
                "username": "Mason",
                "discriminator": "9999",
                "id": "53908099506183680",
                "avatar": "a_bab14f271d565501444b2ca3be944b25"
            },
            "mention_roles": [],
            "content": "Supa Hot",
            "channel_id": "290926798999357250",
            "mentions": [],
            "type": 0
        });

        let msg: Message = serde_json::from_value(json).unwrap();
        let expected = Utc.ymd(2017, 7, 11).and_hms_milli(17, 27, 7, 299);

        // TODO: Check reactions
        // TODO: Check attachments
        assert_eq!(msg.tts(), false);
        // TODO: Check embeds
        assert_eq!(msg.timestamp(), expected);
        assert_eq!(msg.mention_everyone(), false);
        assert_eq!(msg.id(), 334385199974967042.into());
        assert_eq!(msg.pinned(), false);
        assert_eq!(msg.edited_timestamp(), None);
        assert_eq!(msg.mention_roles(), &[]);
        assert_eq!(msg.content(), "Supa Hot");
        assert_eq!(msg.channel_id(), 290926798999357250.into());
        assert!(msg.mentions().is_empty());
        assert_eq!(msg.kind(), MessageKind::Default);

        let author = msg.author().unwrap();
        assert_eq!(author.username(), "Mason");
        assert_eq!(author.discriminator(), "9999");
        assert_eq!(author.id(), 53908099506183680.into());
        assert_eq!(author.avatar(), Some("a_bab14f271d565501444b2ca3be944b25"));
    }

    #[test]
    fn message_deserialize_crossposted() {
        let json = json!({
            "reactions": [
            {
                "count": 1,
                "me": false,
                "emoji": {
                    "id": null,
                    "name": "🔥"
                }
            }
            ],
            "attachments": [],
            "tts": false,
            "embeds": [],
            "timestamp": "2017-07-11T17:27:07.299000+00:00",
            "mention_everyone": false,
            "id": "334385199974967042",
            "pinned": false,
            "edited_timestamp": null,
            "author": {
                "username": "Mason",
                "discriminator": "9999",
                "id": "53908099506183680",
                "avatar": "a_bab14f271d565501444b2ca3be944b25"
            },
            "mention_roles": [],
            "mention_channels": [
            {
                "id": "278325129692446722",
                "guild_id": "278325129692446720",
                "name": "big-news",
                "type": 5
            }
            ],
            "content": "Big news! In this <#278325129692446722> channel!",
            "channel_id": "290926798999357250",
            "mentions": [],
            "type": 0,
            "flags": 2,
            "message_reference": {
                "channel_id": "278325129692446722",
                "guild_id": "278325129692446720",
                "message_id": "306588351130107906"
            }
        });

        let msg: Message = serde_json::from_value(json).unwrap();
        let expected = Utc.ymd(2017, 7, 11).and_hms_milli(17, 27, 7, 299);

        // TODO: Check reactions
        // TODO: Check attachments
        assert_eq!(msg.tts(), false);
        // TODO: Check embeds
        assert_eq!(msg.timestamp(), expected);
        assert_eq!(msg.mention_everyone(), false);
        assert_eq!(msg.id(), 334385199974967042.into());
        assert_eq!(msg.pinned(), false);
        assert_eq!(msg.edited_timestamp(), None);
        assert_eq!(msg.mention_roles(), &[]);
        assert_eq!(
            msg.content(),
            "Big news! In this <#278325129692446722> channel!"
        );
        assert_eq!(msg.channel_id(), 290926798999357250.into());
        assert!(msg.mentions().is_empty());
        assert_eq!(msg.kind(), MessageKind::Default);
        assert_eq!(msg.flags(), Some(MessageFlags::IS_CROSSPOST));
        // TODO: Check message reference

        let mention_channels = msg.mention_channels().unwrap();
        assert_eq!(mention_channels.len(), 1);

        assert_eq!(mention_channels[0].id(), 278325129692446722.into());
        assert_eq!(mention_channels[0].guild_id(), 278325129692446720.into());
        assert_eq!(mention_channels[0].name(), "big-news");
        assert_eq!(mention_channels[0].kind(), ChannelKind::GuildNews);

        let author = msg.author().unwrap();
        assert_eq!(author.username(), "Mason");
        assert_eq!(author.discriminator(), "9999");
        assert_eq!(author.id(), 53908099506183680.into());
        assert_eq!(author.avatar(), Some("a_bab14f271d565501444b2ca3be944b25"));
    }
}
