// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use crate::enums::{ParseEnumError, StringEnum};
use crate::resources::guild::{GuildId, IntegrationId};
use crate::resources::user::BotId;
use crate::snowflake::Id;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::str::FromStr;

pub type RoleId = Id<Role>;

impl RoleId {
    pub fn everyone(guild_id: GuildId) -> Self {
        let id: u64 = guild_id.into();
        id.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    id: RoleId,
    name: String,
    color: u32,
    hoist: bool,
    position: u64,
    permissions: StringEnum<Permissions>,
    managed: bool,
    mentionable: bool,
    tags: Option<Vec<RoleTag>>,
}

impl Role {
    pub fn id(&self) -> RoleId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> u32 {
        self.color
    }

    pub fn hoist(&self) -> bool {
        self.hoist
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn try_permissions(&self) -> Result<Permissions, ParseEnumError> {
        self.permissions.try_unwrap()
    }

    pub fn permissions(&self) -> Permissions {
        self.permissions.unwrap()
    }

    pub fn managed(&self) -> bool {
        self.managed
    }

    pub fn mentionable(&self) -> bool {
        self.mentionable
    }

    pub fn tags(&self) -> Option<&[RoleTag]> {
        self.tags.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct RoleTag {
    bot_id: Option<BotId>,
    integration_id: Option<IntegrationId>,
    premium_subscriber: bool,
}

mod role_tag {
    // TODO: Uh, figure out the correct way to serde this struct.
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    enum Void {}

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct NormalRoleTag {
        pub bot_id: Option<BotId>,
        pub integration_id: Option<IntegrationId>,
    }

    impl From<&RoleTag> for NormalRoleTag {
        fn from(rt: &RoleTag) -> Self {
            Self {
                bot_id: rt.bot_id,
                integration_id: rt.integration_id,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(super) struct PremiumRoleTag {
        pub bot_id: Option<BotId>,
        pub integration_id: Option<IntegrationId>,
        premium_subscriber: Option<Void>,
    }

    impl From<&RoleTag> for PremiumRoleTag {
        fn from(rt: &RoleTag) -> Self {
            Self {
                bot_id: rt.bot_id,
                integration_id: rt.integration_id,
                premium_subscriber: None,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub(super) enum MaybeRoleTag {
        NormalRoleTag(NormalRoleTag),
        PremiumRoleTag(PremiumRoleTag),
    }
}

impl Serialize for RoleTag {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.premium_subscriber {
            role_tag::PremiumRoleTag::from(self).serialize(s)
        } else {
            role_tag::NormalRoleTag::from(self).serialize(s)
        }
    }
}

impl<'de> Deserialize<'de> for RoleTag {
    fn deserialize<D>(d: D) -> Result<RoleTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe = role_tag::MaybeRoleTag::deserialize(d)?;

        let result = match maybe {
            role_tag::MaybeRoleTag::NormalRoleTag(n) => RoleTag {
                bot_id: n.bot_id,
                integration_id: n.integration_id,
                premium_subscriber: false,
            },
            role_tag::MaybeRoleTag::PremiumRoleTag(p) => RoleTag {
                bot_id: p.bot_id,
                integration_id: p.integration_id,
                premium_subscriber: true,
            },
        };

        Ok(result)
    }
}

impl RoleTag {
    pub fn bot_id(&self) -> Option<BotId> {
        self.bot_id
    }

    pub fn integration_id(&self) -> Option<IntegrationId> {
        self.integration_id
    }

    pub fn premium_subscriber(&self) -> bool {
        self.premium_subscriber
    }
}

bitflags! {
    pub struct Permissions: u64 {
        const CREATE_INSTANT_INVITE = 1 << 0;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        const MANAGE_EMOJIS = 1 << 30;
        const USE_SLASH_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        const MANAGE_THREADS = 1 << 34;
        const USE_PUBLIC_THREADS = 1 << 35;
        const USE_PRIVATE_THREADS = 1 << 36;
    }
}

impl FromStr for Permissions {
    type Err = ParseEnumError;

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        let num: u64 = txt
            .parse()
            .map_err(|_| ParseEnumError::new(txt.to_owned()))?;
        let parsed = Permissions::from_bits(num)
            .ok_or_else(|| ParseEnumError::new(txt.to_owned()))?;

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_role() {
        let json = json!({
            "id": "41771983423143936",
            "name": "WE DEM BOYZZ!!!!!!",
            "color": 3447003,
            "hoist": true,
            "position": 1,
            "permissions": "66321471",
            "managed": false,
            "mentionable": false
        });

        let role: Role = serde_json::from_value(json).unwrap();

        assert_eq!(role.id(), 41771983423143936.into());
        assert_eq!(role.name(), "WE DEM BOYZZ!!!!!!");
        assert_eq!(role.color(), 3447003);
        assert_eq!(role.hoist(), true);
        assert_eq!(role.position(), 1);

        let permissions = Permissions::CREATE_INSTANT_INVITE
            | Permissions::KICK_MEMBERS
            | Permissions::BAN_MEMBERS
            | Permissions::ADMINISTRATOR
            | Permissions::MANAGE_CHANNELS
            | Permissions::MANAGE_GUILD
            | Permissions::VIEW_CHANNEL
            | Permissions::SEND_MESSAGES
            | Permissions::SEND_TTS_MESSAGES
            | Permissions::MANAGE_MESSAGES
            | Permissions::EMBED_LINKS
            | Permissions::ATTACH_FILES
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::MENTION_EVERYONE
            | Permissions::CONNECT
            | Permissions::SPEAK
            | Permissions::MUTE_MEMBERS
            | Permissions::DEAFEN_MEMBERS
            | Permissions::MOVE_MEMBERS
            | Permissions::USE_VAD;
        assert_eq!(role.permissions(), permissions);
    }

    #[test]
    fn deserialize_role_tag_normal() {
        let json = json!({});

        let tag: RoleTag = serde_json::from_value(json).unwrap();

        assert!(!tag.premium_subscriber());
    }

    #[test]
    fn deserialize_role_tag_premium_subscriber() {
        let json = json!({
            "premium_subscriber": null,
        });

        let tag: RoleTag = serde_json::from_value(json).unwrap();

        assert!(tag.premium_subscriber());
    }
}
