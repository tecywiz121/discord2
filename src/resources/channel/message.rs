// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use chrono::{DateTime, FixedOffset};

use crate::enums::{
    EnumFromIntegerError, IntegerEnum, ParseEnumError, StringEnum,
};
use crate::permissions::RoleId;
use crate::resources::application::{Application, ApplicationId};
use crate::resources::emoji::Emoji;
use crate::resources::guild::{GuildId, GuildMember};
use crate::resources::user::{User, UserId};
use crate::resources::webhook::WebhookId;
use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::str::FromStr;

use super::embed::*;
use super::{Channel, ChannelId, ChannelKind};

use typed_builder::TypedBuilder;

pub type AttachmentId = Id<Attachment>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    id: AttachmentId,
    filename: String,
    content_type: Option<String>,
    size: u64,
    url: String,
    proxy_url: String,
    height: Option<u64>,
    width: Option<u64>,
}

impl Attachment {
    pub fn id(&self) -> AttachmentId {
        self.id
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn proxy_url(&self) -> &str {
        &self.proxy_url
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    #[serde(flatten)]
    user: User,
    member: Option<GuildMember>,
}

impl Mention {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn member(&self) -> Option<&GuildMember> {
        self.member.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nonce {
    Integer(u64),
    String(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MessageKind {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconChange,
    ChannelPinnedMessage,
    GuildMemberJoin,
    UserPremiumGuildSubscription,
    UserPremiumGuildSubscriptionTier1,
    UserPremiumGuildSubscriptionTier2,
    UserPremiumGuildSubscriptionTier3,
    ChannelFollowAdd,
    GuildDiscoveryDisqualified,
    GuildDiscoveryRequalified,
    GuildDiscoveryGracePeriodInitialWarning,
    GuildDiscoveryGracePeriodFinalWarning,
    ThreadCreated,
    Reply,
    ApplicationCommand,
    ThreadStarterMessage,
    GuildInviteReminder,
}

impl From<MessageKind> for u64 {
    fn from(u: MessageKind) -> Self {
        match u {
            MessageKind::Default => 0,
            MessageKind::RecipientAdd => 1,
            MessageKind::RecipientRemove => 2,
            MessageKind::Call => 3,
            MessageKind::ChannelNameChange => 4,
            MessageKind::ChannelIconChange => 5,
            MessageKind::ChannelPinnedMessage => 6,
            MessageKind::GuildMemberJoin => 7,
            MessageKind::UserPremiumGuildSubscription => 8,
            MessageKind::UserPremiumGuildSubscriptionTier1 => 9,
            MessageKind::UserPremiumGuildSubscriptionTier2 => 10,
            MessageKind::UserPremiumGuildSubscriptionTier3 => 11,
            MessageKind::ChannelFollowAdd => 12,
            MessageKind::GuildDiscoveryDisqualified => 14,
            MessageKind::GuildDiscoveryRequalified => 15,
            MessageKind::GuildDiscoveryGracePeriodInitialWarning => 16,
            MessageKind::GuildDiscoveryGracePeriodFinalWarning => 17,
            MessageKind::ThreadCreated => 18,
            MessageKind::Reply => 19,
            MessageKind::ApplicationCommand => 20,
            MessageKind::ThreadStarterMessage => 21,
            MessageKind::GuildInviteReminder => 22,
        }
    }
}

impl TryFrom<u64> for MessageKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            0 => Self::Default,
            1 => Self::RecipientAdd,
            2 => Self::RecipientRemove,
            3 => Self::Call,
            4 => Self::ChannelNameChange,
            5 => Self::ChannelIconChange,
            6 => Self::ChannelPinnedMessage,
            7 => Self::GuildMemberJoin,
            8 => Self::UserPremiumGuildSubscription,
            9 => Self::UserPremiumGuildSubscriptionTier1,
            10 => Self::UserPremiumGuildSubscriptionTier2,
            11 => Self::UserPremiumGuildSubscriptionTier3,
            12 => Self::ChannelFollowAdd,
            14 => Self::GuildDiscoveryDisqualified,
            15 => Self::GuildDiscoveryRequalified,
            16 => Self::GuildDiscoveryGracePeriodInitialWarning,
            17 => Self::GuildDiscoveryGracePeriodFinalWarning,
            18 => Self::ThreadCreated,
            19 => Self::Reply,
            20 => Self::ApplicationCommand,
            21 => Self::ThreadStarterMessage,
            22 => Self::GuildInviteReminder,

            raw => return Err(EnumFromIntegerError::new(raw)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMention {
    id: ChannelId,
    guild_id: GuildId,
    #[serde(rename = "type")]
    kind: IntegerEnum<ChannelKind>,
    name: String,
}

impl ChannelMention {
    pub fn id(&self) -> ChannelId {
        self.id
    }

    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    pub fn try_kind(&self) -> Result<ChannelKind, EnumFromIntegerError> {
        self.kind.try_unwrap()
    }

    pub fn kind(&self) -> ChannelKind {
        self.kind.unwrap()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

bitflags! {
    pub struct MessageFlags: u64 {
        const CROSSPOSTED = 1<<0;
        const IS_CROSSPOST = 1<<1;
        const SUPRESS_EMBEDS = 1<<2;
        const SOURCE_MESSAGE_DELETED = 1<<3;
        const URGENT = 1<<4;
        const HAS_THREAD = 1<<5;
        const EPHEMERAL = 1<<6;
        const LOADING = 1<<7;
    }
}

impl TryFrom<u64> for MessageFlags {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Self::from_bits(u).ok_or_else(|| Self::Error::new(u))
    }
}

impl From<MessageFlags> for u64 {
    fn from(uf: MessageFlags) -> u64 {
        uf.bits()
    }
}

pub type MessageId = Id<Message>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    id: MessageId,
    channel_id: ChannelId,
    guild_id: Option<GuildId>,
    author: Option<User>,
    member: Option<GuildMember>,
    content: String,
    timestamp: DateTime<FixedOffset>,
    edited_timestamp: Option<DateTime<FixedOffset>>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<Mention>,
    mention_roles: Vec<RoleId>,
    mention_channels: Option<Vec<ChannelMention>>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<Nonce>,
    pinned: bool,
    webhook_id: Option<WebhookId>,
    #[serde(rename = "type")]
    kind: IntegerEnum<MessageKind>,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<ApplicationId>,
    message_reference: Option<MessageReference>,
    flags: Option<IntegerEnum<MessageFlags>>,
    stickers: Option<Vec<Sticker>>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
}

impl Message {
    pub fn id(&self) -> MessageId {
        self.id
    }

    pub fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    pub fn author(&self) -> Option<&User> {
        self.author.as_ref()
    }

    pub fn member(&self) -> Option<&GuildMember> {
        self.member.as_ref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn timestamp(&self) -> DateTime<FixedOffset> {
        self.timestamp
    }

    pub fn edited_timestamp(&self) -> Option<DateTime<FixedOffset>> {
        self.edited_timestamp
    }

    pub fn tts(&self) -> bool {
        self.tts
    }

    pub fn mention_everyone(&self) -> bool {
        self.mention_everyone
    }

    pub fn mentions(&self) -> &[Mention] {
        &self.mentions
    }

    pub fn mention_roles(&self) -> &[RoleId] {
        &self.mention_roles
    }

    pub fn mention_channels(&self) -> Option<&[ChannelMention]> {
        self.mention_channels.as_deref()
    }

    pub fn attachments(&self) -> &[Attachment] {
        &self.attachments
    }

    pub fn embeds(&self) -> &[Embed] {
        &self.embeds
    }

    pub fn reactions(&self) -> Option<&[Reaction]> {
        self.reactions.as_deref()
    }

    pub fn nonce(&self) -> Option<&Nonce> {
        self.nonce.as_ref()
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }

    pub fn webhook_id(&self) -> Option<WebhookId> {
        self.webhook_id
    }

    pub fn try_kind(&self) -> Result<MessageKind, EnumFromIntegerError> {
        self.kind.try_unwrap()
    }

    pub fn kind(&self) -> MessageKind {
        self.kind.unwrap()
    }

    pub fn activity(&self) -> Option<&MessageActivity> {
        self.activity.as_ref()
    }

    pub fn application(&self) -> Option<&Application> {
        self.application.as_ref()
    }

    pub fn application_id(&self) -> Option<ApplicationId> {
        self.application_id
    }

    pub fn message_reference(&self) -> Option<&MessageReference> {
        self.message_reference.as_ref()
    }

    pub fn try_flags(
        &self,
    ) -> Option<Result<MessageFlags, EnumFromIntegerError>> {
        self.flags.map(IntegerEnum::try_unwrap)
    }

    pub fn flags(&self) -> Option<MessageFlags> {
        self.flags.map(IntegerEnum::unwrap)
    }

    pub fn stickers(&self) -> Option<&[Sticker]> {
        self.stickers.as_deref()
    }

    pub fn referenced_message(&self) -> Option<&Message> {
        self.referenced_message.as_deref()
    }

    pub fn interaction(&self) -> Option<&MessageInteraction> {
        self.interaction.as_ref()
    }

    pub fn thread(&self) -> Option<&Channel> {
        self.thread.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum MessageActivityKind {
    Join,
    Spectate,
    Listen,
    JoinRequest,
}

impl TryFrom<u64> for MessageActivityKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::Join,
            2 => Self::Spectate,
            3 => Self::Listen,
            5 => Self::JoinRequest,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<MessageActivityKind> for u64 {
    fn from(u: MessageActivityKind) -> Self {
        match u {
            MessageActivityKind::Join => 1,
            MessageActivityKind::Spectate => 2,
            MessageActivityKind::Listen => 3,
            MessageActivityKind::JoinRequest => 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    kind: IntegerEnum<MessageActivityKind>,
    party_id: Option<String>,
}

impl MessageActivity {
    pub fn try_kind(
        &self,
    ) -> Result<MessageActivityKind, EnumFromIntegerError> {
        self.kind.try_unwrap()
    }

    pub fn kind(&self) -> MessageActivityKind {
        self.kind.unwrap()
    }

    pub fn party_id(&self) -> Option<&str> {
        self.party_id.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    message_id: Option<MessageId>,
    channel_id: Option<ChannelId>,
    guild_id: Option<GuildId>,
    fail_if_not_exist: Option<bool>,
}

impl MessageReference {
    pub fn message_id(&self) -> Option<MessageId> {
        self.message_id
    }

    pub fn channel_id(&self) -> Option<ChannelId> {
        self.channel_id
    }

    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    pub fn fail_if_not_exist(&self) -> Option<bool> {
        self.fail_if_not_exist
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InteractionKind {
    Ping,
    ApplicationCommand,
}

impl TryFrom<u64> for InteractionKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::Ping,
            2 => Self::ApplicationCommand,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<InteractionKind> for u64 {
    fn from(k: InteractionKind) -> u64 {
        match k {
            InteractionKind::Ping => 1,
            InteractionKind::ApplicationCommand => 2,
        }
    }
}

pub type MessageInteractionId = Id<MessageInteraction>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInteraction {
    id: MessageInteractionId,
    #[serde(rename = "type")]
    kind: IntegerEnum<InteractionKind>,
    name: String,
    user: User,
}

impl MessageInteraction {
    pub fn id(&self) -> MessageInteractionId {
        self.id
    }

    pub fn try_kind(&self) -> Result<InteractionKind, EnumFromIntegerError> {
        self.kind.try_unwrap()
    }

    pub fn kind(&self) -> InteractionKind {
        self.kind.unwrap()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

pub type StickerId = Id<Sticker>;
pub type StickerPackId = Id<StickerPack>;

#[derive(Debug, Clone)]
pub struct StickerPack {
    _p: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    id: StickerId,
    pack_id: StickerPackId,
    name: String,
    description: String,
    tags: Option<String>,
    asset: String,
    #[serde(rename = "format_type")]
    format_kind: IntegerEnum<StickerFormat>,
}

impl Sticker {
    pub fn id(&self) -> StickerId {
        self.id
    }

    pub fn pack_id(&self) -> StickerPackId {
        self.pack_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn tags(&self) -> Option<&str> {
        self.tags.as_deref()
    }

    pub fn asset(&self) -> &str {
        &self.asset
    }

    pub fn try_format_kind(
        &self,
    ) -> Result<StickerFormat, EnumFromIntegerError> {
        self.format_kind.try_unwrap()
    }

    pub fn format_kind(&self) -> StickerFormat {
        self.format_kind.unwrap()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StickerFormat {
    Png,
    APng,
    Lottie,
}

impl TryFrom<u64> for StickerFormat {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::Png,
            2 => Self::APng,
            3 => Self::Lottie,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<StickerFormat> for u64 {
    fn from(u: StickerFormat) -> Self {
        match u {
            StickerFormat::Png => 1,
            StickerFormat::APng => 2,
            StickerFormat::Lottie => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    count: u64,
    me: bool,
    emoji: Emoji,
}

impl Reaction {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn me(&self) -> bool {
        self.me
    }

    pub fn emoji(&self) -> &Emoji {
        &self.emoji
    }
}

#[derive(Debug, Clone, Eq, Copy, PartialEq, Hash)]
pub enum MentionKind {
    Roles,
    Users,
    Everyone,
}

impl FromStr for MentionKind {
    type Err = ParseEnumError;

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        let r = match txt {
            "roles" => Self::Roles,
            "users" => Self::Users,
            "everyone" => Self::Everyone,

            other => return Err(ParseEnumError::new(other.to_owned())),
        };

        Ok(r)
    }
}

impl AsRef<str> for MentionKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Roles => "roles",
            Self::Users => "users",
            Self::Everyone => "everyone",
        }
    }
}

impl std::fmt::Display for MentionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let txt = self.as_ref();
        f.write_str(txt)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AllowedMentions {
    #[builder(setter(into))]
    parse: Vec<StringEnum<MentionKind>>,

    #[builder(setter(into))]
    roles: Vec<RoleId>,

    #[builder(setter(into))]
    users: Vec<UserId>,

    #[builder(default)]
    replied_user: bool,
}
