// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod integration;

use bitflags::bitflags;

use chrono::{DateTime, FixedOffset};

use crate::enums::{
    EnumFromIntegerError, IntegerEnum, ParseEnumError, StringEnum,
};
use crate::gateway::PresenceUpdateEvent;
use crate::permissions::{Role, RoleId};
use crate::resources::application::ApplicationId;
use crate::resources::channel::{Channel, ChannelId};
use crate::resources::emoji::{Emoji, EmojiId};
use crate::resources::user::{User, UserId};
use crate::resources::voice::VoiceState;
use crate::snowflake::Id;

pub use self::integration::*;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::str::FromStr;

pub type GuildId = Id<Guild>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl From<VerificationLevel> for u64 {
    fn from(u: VerificationLevel) -> Self {
        match u {
            VerificationLevel::None => 0,
            VerificationLevel::Low => 1,
            VerificationLevel::Medium => 2,
            VerificationLevel::High => 3,
            VerificationLevel::VeryHigh => 4,
        }
    }
}

impl TryFrom<u64> for VerificationLevel {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            0 => Self::None,
            1 => Self::Low,
            2 => Self::Medium,
            3 => Self::High,
            4 => Self::VeryHigh,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DefaultMessageNotificationLevel {
    AllMessages,
    OnlyMentions,
}

impl From<DefaultMessageNotificationLevel> for u64 {
    fn from(u: DefaultMessageNotificationLevel) -> Self {
        match u {
            DefaultMessageNotificationLevel::AllMessages => 0,
            DefaultMessageNotificationLevel::OnlyMentions => 1,
        }
    }
}

impl TryFrom<u64> for DefaultMessageNotificationLevel {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            0 => DefaultMessageNotificationLevel::AllMessages,
            1 => DefaultMessageNotificationLevel::OnlyMentions,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplicitContentFilterLevel {
    Disabled,
    MembersWithoutRoles,
    AllMembers,
}

impl TryFrom<u64> for ExplicitContentFilterLevel {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            0 => Self::Disabled,
            1 => Self::MembersWithoutRoles,
            2 => Self::AllMembers,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<ExplicitContentFilterLevel> for u64 {
    fn from(u: ExplicitContentFilterLevel) -> Self {
        match u {
            ExplicitContentFilterLevel::Disabled => 0,
            ExplicitContentFilterLevel::MembersWithoutRoles => 1,
            ExplicitContentFilterLevel::AllMembers => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GuildFeature {
    AnimatedIcon,
    Banner,
    Commerce,
    Community,
    Discoverable,
    Featurable,
    InviteSplash,
    MemberVerificationGateEnabled,
    News,
    Partnered,
    PreviewEnabled,
    VanityUrl,
    Verified,
    VipRegions,
    WelcomeScreenEnabled,
}

impl FromStr for GuildFeature {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "ANIMATED_ICON" => Self::AnimatedIcon,
            "BANNER" => Self::Banner,
            "COMMERCE" => Self::Commerce,
            "COMMUNITY" => Self::Community,
            "DISCOVERABLE" => Self::Discoverable,
            "FEATURABLE" => Self::Featurable,
            "INVITE_SPLASH" => Self::InviteSplash,
            "MEMBER_VERIFICATION_GATE_ENABLED" => {
                Self::MemberVerificationGateEnabled
            }
            "NEWS" => Self::News,
            "PARTNERED" => Self::Partnered,
            "PREVIEW_ENABLED" => Self::PreviewEnabled,
            "VANITY_URL" => Self::VanityUrl,
            "VERIFIED" => Self::Verified,
            "VIP_REGIONS" => Self::VipRegions,
            "WELCOME_SCREEN_ENABLED" => Self::WelcomeScreenEnabled,

            other => return Err(ParseEnumError::new(other.to_owned())),
        };

        Ok(r)
    }
}

impl AsRef<str> for GuildFeature {
    fn as_ref(&self) -> &str {
        match self {
            GuildFeature::AnimatedIcon => "ANIMATED_ICON",
            GuildFeature::Banner => "BANNER",
            GuildFeature::Commerce => "COMMERCE",
            GuildFeature::Community => "COMMUNITY",
            GuildFeature::Discoverable => "DISCOVERABLE",
            GuildFeature::Featurable => "FEATURABLE",
            GuildFeature::InviteSplash => "INVITE_SPLASH",
            GuildFeature::MemberVerificationGateEnabled => {
                "MEMBER_VERIFICATION_GATE_ENABLED"
            }
            GuildFeature::News => "NEWS",
            GuildFeature::Partnered => "PARTNERED",
            GuildFeature::PreviewEnabled => "PREVIEW_ENABLED",
            GuildFeature::VanityUrl => "VANITY_URL",
            GuildFeature::Verified => "VERIFIED",
            GuildFeature::VipRegions => "VIP_REGIONS",
            GuildFeature::WelcomeScreenEnabled => "WELCOME_SCREEN_ENABLED",
        }
    }
}

impl std::fmt::Display for GuildFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let txt = self.as_ref();
        f.write_str(txt)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MfaLevel {
    None,
    Elevated,
}

impl TryFrom<u64> for MfaLevel {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<MfaLevel, Self::Error> {
        let r = match u {
            0 => MfaLevel::None,
            1 => MfaLevel::Elevated,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<MfaLevel> for u64 {
    fn from(u: MfaLevel) -> Self {
        match u {
            MfaLevel::None => 0,
            MfaLevel::Elevated => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PremiumTier {
    None,
    Tier1,
    Tier2,
    Tier3,
}

impl TryFrom<u64> for PremiumTier {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            0 => Self::None,
            1 => Self::Tier1,
            2 => Self::Tier2,
            3 => Self::Tier3,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}
impl From<PremiumTier> for u64 {
    fn from(u: PremiumTier) -> Self {
        match u {
            PremiumTier::None => 0,
            PremiumTier::Tier1 => 1,
            PremiumTier::Tier2 => 2,
            PremiumTier::Tier3 => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>,
}

impl WelcomeScreen {
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn welcome_channels(&self) -> &[WelcomeScreenChannel] {
        &self.welcome_channels
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreenChannel {
    channel_id: ChannelId,
    description: String,
    emoji_id: Option<EmojiId>,
    emoji_name: Option<String>,
}

impl WelcomeScreenChannel {
    pub fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn emoji_id(&self) -> Option<EmojiId> {
        self.emoji_id
    }

    pub fn emoji_name(&self) -> Option<&str> {
        self.emoji_name.as_deref()
    }
}

mod unavailable {
    use serde::de::{Deserialize, Deserializer, Error as _, Unexpected};
    use serde::ser::Serializer;

    pub(super) fn serialize<S>(_: &(), s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_bool(true)
    }

    pub(super) fn deserialize<'de, D>(d: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<bool>::deserialize(d)?.unwrap_or_default();

        if value {
            Ok(())
        } else {
            Err(D::Error::invalid_value(Unexpected::Bool(value), &"true"))
        }
    }
}

mod available {
    use serde::de::{Deserialize, Deserializer, Error as _, Unexpected};
    use serde::ser::Serializer;

    pub(super) fn serialize<S>(_: &(), s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_bool(false)
    }

    pub(super) fn deserialize<'de, D>(d: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<bool>::deserialize(d)?.unwrap_or_default();

        if value {
            Err(D::Error::invalid_value(Unexpected::Bool(value), &"false"))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Guild {
    Available(AvailableGuild),
    Unavailable(UnavailableGuild),
}

impl Guild {
    pub fn id(&self) -> GuildId {
        match self {
            Self::Available(a) => a.id,
            Self::Unavailable(u) => u.id,
        }
    }

    pub fn unavailable(&self) -> bool {
        match self {
            Self::Unavailable(u) => u.unavailable(),
            Self::Available(a) => a.unavailable(),
        }
    }

    pub fn into_available(self) -> Option<AvailableGuild> {
        match self {
            Self::Available(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_available(&self) -> Option<&AvailableGuild> {
        match self {
            Self::Available(a) => Some(a),
            _ => None,
        }
    }

    pub fn into_unavailable(self) -> Option<UnavailableGuild> {
        match self {
            Self::Unavailable(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_unavailable(&self) -> Option<&UnavailableGuild> {
        match self {
            Self::Unavailable(u) => Some(u),
            _ => None,
        }
    }
}

bitflags! {
    pub struct SystemChannelFlags: u64 {
        const SUPRESS_JOIN_NOTIFICATIONS = 1<<0;
        const SUPRESS_PREMIUM_SUBSCRIPTIONS = 1<<1;
        const SUPRESS_GUILD_REMINDER_NOTIFICATIONS = 1<<2;
    }
}

impl TryFrom<u64> for SystemChannelFlags {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Self::from_bits(u).ok_or_else(|| Self::Error::new(u))
    }
}

impl From<SystemChannelFlags> for u64 {
    fn from(uf: SystemChannelFlags) -> u64 {
        uf.bits()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnavailableGuild {
    id: GuildId,
    #[serde(with = "unavailable")]
    unavailable: (),
}

impl UnavailableGuild {
    pub fn id(&self) -> GuildId {
        self.id
    }

    pub fn unavailable(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableGuild {
    id: GuildId,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: Option<bool>,
    owner_id: UserId,
    permissions: Option<String>,
    region: String,
    afk_channel_id: Option<ChannelId>,
    afk_timeout: u64,
    widget_enabled: Option<bool>,
    widget_channel_id: Option<ChannelId>,
    verification_level: IntegerEnum<VerificationLevel>,
    default_message_notifications: IntegerEnum<DefaultMessageNotificationLevel>,
    explicit_content_filter: IntegerEnum<ExplicitContentFilterLevel>,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<StringEnum<GuildFeature>>,
    mfa_level: IntegerEnum<MfaLevel>,
    application_id: Option<ApplicationId>,
    system_channel_id: Option<ChannelId>,
    system_channel_flags: IntegerEnum<SystemChannelFlags>,
    rules_channel_id: Option<ChannelId>,
    joined_at: Option<DateTime<FixedOffset>>,
    large: Option<bool>,
    #[serde(with = "available", default)]
    unavailable: (),
    member_count: Option<u64>,
    voice_states: Option<Vec<VoiceState>>,
    members: Option<Vec<GuildMember>>,
    channels: Option<Vec<Channel>>,
    threads: Option<Vec<Channel>>,
    presences: Option<Vec<PresenceUpdateEvent>>,
    max_presences: Option<u64>,
    max_members: Option<u64>,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: IntegerEnum<PremiumTier>,
    premium_subscription_count: Option<u64>,
    preferred_locale: String,
    public_updates_channel_id: Option<ChannelId>,
    max_video_channel_users: Option<u64>,
    approximate_member_count: Option<u64>,
    welcome_screen: Option<WelcomeScreen>,
    nsfw: Option<bool>,
}

impl AvailableGuild {
    pub fn id(&self) -> GuildId {
        self.id
    }

    pub fn unavailable(&self) -> bool {
        false
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn icon_hash(&self) -> Option<&str> {
        self.icon_hash.as_deref()
    }

    pub fn splash(&self) -> Option<&str> {
        self.splash.as_deref()
    }

    pub fn discovery_splash(&self) -> Option<&str> {
        self.discovery_splash.as_deref()
    }

    pub fn owner(&self) -> Option<bool> {
        self.owner
    }

    pub fn owner_id(&self) -> UserId {
        self.owner_id
    }

    pub fn permissions(&self) -> Option<&str> {
        self.permissions.as_deref()
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn afk_channel_id(&self) -> Option<ChannelId> {
        self.afk_channel_id
    }

    pub fn afk_timeout(&self) -> u64 {
        self.afk_timeout
    }

    pub fn widget_enabled(&self) -> Option<bool> {
        self.widget_enabled
    }

    pub fn widget_channel_id(&self) -> Option<ChannelId> {
        self.widget_channel_id
    }

    pub fn try_verification_level(
        &self,
    ) -> Result<VerificationLevel, EnumFromIntegerError> {
        self.verification_level.try_unwrap()
    }

    pub fn verification_level(&self) -> VerificationLevel {
        self.verification_level.unwrap()
    }

    pub fn try_default_message_notifications(
        &self,
    ) -> Result<DefaultMessageNotificationLevel, EnumFromIntegerError> {
        self.default_message_notifications.try_unwrap()
    }

    pub fn default_message_notifications(
        &self,
    ) -> DefaultMessageNotificationLevel {
        self.default_message_notifications.unwrap()
    }

    pub fn try_explicit_content_filter(
        &self,
    ) -> Result<ExplicitContentFilterLevel, EnumFromIntegerError> {
        self.explicit_content_filter.try_unwrap()
    }

    pub fn explicit_content_filter(&self) -> ExplicitContentFilterLevel {
        self.explicit_content_filter.unwrap()
    }

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    pub fn emojis(&self) -> &[Emoji] {
        &self.emojis
    }

    pub fn try_features(
        &self,
    ) -> impl Iterator<Item = &StringEnum<GuildFeature>> {
        self.features.iter()
    }

    pub fn features(&self) -> impl Iterator<Item = GuildFeature> + '_ {
        self.features.iter().map(|x| x.unwrap())
    }

    pub fn try_mfa_level(&self) -> Result<MfaLevel, EnumFromIntegerError> {
        self.mfa_level.try_unwrap()
    }

    pub fn mfa_level(&self) -> MfaLevel {
        self.mfa_level.unwrap()
    }

    pub fn application_id(&self) -> Option<ApplicationId> {
        self.application_id
    }

    pub fn system_channel_id(&self) -> Option<ChannelId> {
        self.system_channel_id
    }

    pub fn try_system_channel_flags(
        &self,
    ) -> Result<SystemChannelFlags, EnumFromIntegerError> {
        self.system_channel_flags.try_unwrap()
    }

    pub fn system_channel_flags(&self) -> SystemChannelFlags {
        self.system_channel_flags.unwrap()
    }

    pub fn rules_channel_id(&self) -> Option<ChannelId> {
        self.rules_channel_id
    }

    pub fn joined_at(&self) -> Option<DateTime<FixedOffset>> {
        self.joined_at
    }

    pub fn large(&self) -> Option<bool> {
        self.large
    }

    pub fn member_count(&self) -> Option<u64> {
        self.member_count
    }

    pub fn voice_states(&self) -> Option<&[VoiceState]> {
        self.voice_states.as_deref()
    }

    pub fn members(&self) -> Option<&[GuildMember]> {
        self.members.as_deref()
    }

    pub fn channels(&self) -> Option<&[Channel]> {
        self.channels.as_deref()
    }

    pub fn threads(&self) -> Option<&[Channel]> {
        self.threads.as_deref()
    }

    pub fn presences(&self) -> Option<&[PresenceUpdateEvent]> {
        self.presences.as_deref()
    }

    pub fn max_presences(&self) -> Option<u64> {
        self.max_presences
    }

    pub fn max_members(&self) -> Option<u64> {
        self.max_members
    }

    pub fn vanity_url_code(&self) -> Option<&str> {
        self.vanity_url_code.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn banner(&self) -> Option<&str> {
        self.banner.as_deref()
    }

    pub fn try_premium_tier(
        &self,
    ) -> Result<PremiumTier, EnumFromIntegerError> {
        self.premium_tier.try_unwrap()
    }

    pub fn premium_tier(&self) -> PremiumTier {
        self.premium_tier.unwrap()
    }

    pub fn premium_subscription_count(&self) -> Option<u64> {
        self.premium_subscription_count
    }

    pub fn preferred_locale(&self) -> &str {
        &self.preferred_locale
    }

    pub fn public_updates_channel_id(&self) -> Option<ChannelId> {
        self.public_updates_channel_id
    }

    pub fn max_video_channel_users(&self) -> Option<u64> {
        self.max_video_channel_users
    }

    pub fn approximate_member_count(&self) -> Option<u64> {
        self.approximate_member_count
    }

    pub fn welcome_screen(&self) -> Option<&WelcomeScreen> {
        self.welcome_screen.as_ref()
    }

    pub fn nsfw(&self) -> Option<bool> {
        self.nsfw
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Vec<RoleId>,
    joined_at: DateTime<FixedOffset>,
    premium_since: Option<DateTime<FixedOffset>>,
    deaf: bool,
    mute: bool,
    pending: Option<bool>,
    permissions: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_guild_features() {
        let json = json!(["ANIMATED_ICON", "BANNER", "FLOOP"]);

        let features: Vec<StringEnum<GuildFeature>> =
            serde_json::from_value(json).unwrap();

        assert_eq!(features.len(), 3);
        assert_eq!(features[0].unwrap(), GuildFeature::AnimatedIcon);
        assert_eq!(features[1].unwrap(), GuildFeature::Banner);
        assert_eq!(features[2].to_string(), "FLOOP");
    }

    #[test]
    fn deserialize_guild_available() {
        let json = json!({
            "id": "197038439483310086",
            "name": "Discord Testers",
            "icon": "f64c482b807da4f539cff778d174971c",
            "description": "The official place to report Discord Bugs!",
            "splash": null,
            "discovery_splash": null,
            "features": [
                "ANIMATED_ICON",
                "VERIFIED",
                "NEWS",
                "VANITY_URL",
                "DISCOVERABLE",
                "MORE_EMOJI",
                "INVITE_SPLASH",
                "BANNER",
                "COMMUNITY"
            ],
            "emojis": [],
            "banner": "9b6439a7de04f1d26af92f84ac9e1e4a",
            "owner_id": "73193882359173120",
            "application_id": null,
            "region": "us-west",
            "afk_channel_id": null,
            "afk_timeout": 300,
            "system_channel_id": null,
            "widget_enabled": true,
            "widget_channel_id": null,
            "verification_level": 3,
            "roles": [],
            "default_message_notifications": 1,
            "mfa_level": 1,
            "explicit_content_filter": 2,
            "max_presences": 40000,
            "max_members": 250000,
            "vanity_url_code": "discord-testers",
            "premium_tier": 3,
            "premium_subscription_count": 33,
            "system_channel_flags": 0,
            "preferred_locale": "en-US",
            "rules_channel_id": "441688182833020939",
            "public_updates_channel_id": "281283303326089216"
        });

        let guild: Guild = serde_json::from_value(json).unwrap();
        let avail = guild.into_available().unwrap();

        assert_eq!(avail.id(), 197038439483310086.into());
        assert_eq!(avail.name(), "Discord Testers");
        assert_eq!(avail.icon(), Some("f64c482b807da4f539cff778d174971c"));
        assert_eq!(
            avail.description(),
            Some("The official place to report Discord Bugs!")
        );

        assert!(avail.splash().is_none());
        assert!(avail.discovery_splash().is_none());

        let features: Vec<_> = avail.try_features().collect();
        assert_eq!(features.len(), 9);
        assert_eq!(features[0].unwrap(), GuildFeature::AnimatedIcon);
        assert_eq!(features[1].unwrap(), GuildFeature::Verified);
        assert_eq!(features[2].unwrap(), GuildFeature::News);
        assert_eq!(features[3].unwrap(), GuildFeature::VanityUrl);
        assert_eq!(features[4].unwrap(), GuildFeature::Discoverable);
        assert_eq!(features[5].to_string(), "MORE_EMOJI");
        assert_eq!(features[6].unwrap(), GuildFeature::InviteSplash);
        assert_eq!(features[7].unwrap(), GuildFeature::Banner);
        assert_eq!(features[8].unwrap(), GuildFeature::Community);

        assert!(avail.emojis().is_empty());

        assert_eq!(avail.banner(), Some("9b6439a7de04f1d26af92f84ac9e1e4a"));
        assert_eq!(avail.owner_id(), 73193882359173120.into());
        assert_eq!(avail.application_id(), None);
        assert_eq!(avail.region(), "us-west");
        assert_eq!(avail.afk_channel_id(), None);
        assert_eq!(avail.afk_timeout(), 300);
        assert_eq!(avail.system_channel_id(), None);
        assert_eq!(avail.widget_enabled(), Some(true));
        assert_eq!(avail.widget_channel_id(), None);
        assert_eq!(avail.verification_level(), VerificationLevel::High);
        assert!(avail.roles().is_empty());
        assert_eq!(
            avail.default_message_notifications(),
            DefaultMessageNotificationLevel::OnlyMentions
        );
        assert_eq!(avail.mfa_level(), MfaLevel::Elevated);
        assert_eq!(
            avail.explicit_content_filter(),
            ExplicitContentFilterLevel::AllMembers
        );
        assert_eq!(avail.max_presences(), Some(40_000));
        assert_eq!(avail.max_members(), Some(250_000));
        assert_eq!(avail.vanity_url_code(), Some("discord-testers"));
        assert_eq!(avail.premium_tier(), PremiumTier::Tier3);
        assert_eq!(avail.premium_subscription_count(), Some(33));
        assert_eq!(avail.system_channel_flags(), SystemChannelFlags::empty());
        assert_eq!(avail.preferred_locale(), "en-US");
        assert_eq!(avail.rules_channel_id(), Some(441688182833020939.into()));
        assert_eq!(
            avail.public_updates_channel_id(),
            Some(281283303326089216.into())
        );
    }

    #[test]
    fn deserialize_guild_unavailable() {
        let json = json!({
            "id": "41771983423143937",
            "unavailable": true
        });

        let guild: Guild = serde_json::from_value(json).unwrap();
        let unavailable = guild.into_unavailable().unwrap();
        assert_eq!(unavailable.id(), 41771983423143937.into());
        assert_eq!(unavailable.unavailable(), true);
    }

    #[test]
    fn deserialize_unavailable_guild_available() {
        let json = json!({
            "id": "41771983423143937",
            "unavailable": false
        });

        let guild: Result<UnavailableGuild, _> = serde_json::from_value(json);
        guild.unwrap_err();
    }

    #[test]
    fn deserialize_available_guild_unavailable() {
        let json = json!({
            "unavailable": true,
            "id": "197038439483310086",
            "name": "Discord Testers",
            "icon": "f64c482b807da4f539cff778d174971c",
            "description": "The official place to report Discord Bugs!",
            "splash": null,
            "discovery_splash": null,
            "features": [
                "ANIMATED_ICON",
                "VERIFIED",
                "NEWS",
                "VANITY_URL",
                "DISCOVERABLE",
                "MORE_EMOJI",
                "INVITE_SPLASH",
                "BANNER",
                "COMMUNITY"
            ],
            "emojis": [],
            "banner": "9b6439a7de04f1d26af92f84ac9e1e4a",
            "owner_id": "73193882359173120",
            "application_id": null,
            "region": "us-west",
            "afk_channel_id": null,
            "afk_timeout": 300,
            "system_channel_id": null,
            "widget_enabled": true,
            "widget_channel_id": null,
            "verification_level": 3,
            "roles": [],
            "default_message_notifications": 1,
            "mfa_level": 1,
            "explicit_content_filter": 2,
            "max_presences": 40000,
            "max_members": 250000,
            "vanity_url_code": "discord-testers",
            "premium_tier": 3,
            "premium_subscription_count": 33,
            "system_channel_flags": 0,
            "preferred_locale": "en-US",
            "rules_channel_id": "441688182833020939",
            "public_updates_channel_id": "281283303326089216"
        });

        let guild: Result<AvailableGuild, _> = serde_json::from_value(json);
        guild.unwrap_err();
    }

    #[test]
    fn deserialize_welcome_screen() {
        let json = json!({
            "description": "Discord Developers...",
            "welcome_channels": [
            {
                "channel_id": "697138785317814292",
                "description": "Follow for official Discord API updates",
                "emoji_id": null,
                "emoji_name": "📡"
            },
            {
                "channel_id": "697236247739105340",
                "description": "Get help with Bot Verifications",
                "emoji_id": null,
                "emoji_name": "📸"
            },
            {
                "channel_id": "697489244649816084",
                "description": "Create amazing things with Discord's API",
                "emoji_id": null,
                "emoji_name": "🔬"
            },
            {
                "channel_id": "613425918748131338",
                "description": "Integrate Discord into your game",
                "emoji_id": null,
                "emoji_name": "🎮"
            },
            {
                "channel_id": "646517734150242346",
                "description": "Find more places to help you on your quest",
                "emoji_id": null,
                "emoji_name": "🔦"
            }
            ]
        });

        let scrn: WelcomeScreen = serde_json::from_value(json).unwrap();

        assert_eq!(scrn.description(), Some("Discord Developers..."));

        let channels = scrn.welcome_channels();
        assert_eq!(channels.len(), 5);

        assert_eq!(channels[0].channel_id(), 697138785317814292.into());
        assert_eq!(
            channels[0].description(),
            "Follow for official Discord API updates"
        );
        assert_eq!(channels[0].emoji_id(), None);
        assert_eq!(channels[0].emoji_name(), Some("\u{1F4E1}"));

        assert_eq!(channels[1].channel_id(), 697236247739105340.into());
        assert_eq!(
            channels[1].description(),
            "Get help with Bot Verifications"
        );
        assert_eq!(channels[1].emoji_id(), None);
        assert_eq!(channels[1].emoji_name(), Some("\u{1F4F8}"));

        assert_eq!(channels[2].channel_id(), 697489244649816084.into());
        assert_eq!(
            channels[2].description(),
            "Create amazing things with Discord's API"
        );
        assert_eq!(channels[2].emoji_id(), None);
        assert_eq!(channels[2].emoji_name(), Some("\u{1F52C}"));

        assert_eq!(channels[3].channel_id(), 613425918748131338.into());
        assert_eq!(
            channels[3].description(),
            "Integrate Discord into your game"
        );
        assert_eq!(channels[3].emoji_id(), None);
        assert_eq!(channels[3].emoji_name(), Some("\u{1F3AE}"));

        assert_eq!(channels[4].channel_id(), 646517734150242346.into());
        assert_eq!(
            channels[4].description(),
            "Find more places to help you on your quest"
        );
        assert_eq!(channels[4].emoji_id(), None);
        assert_eq!(channels[4].emoji_name(), Some("\u{1F526}"));
    }
}
