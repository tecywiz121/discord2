// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod commands;

use bitflags::bitflags;

use crate::enums::{EnumFromIntegerError, IntegerEnum};
use crate::game_sdk::SkuId;
use crate::image;
use crate::resources::guild::GuildId;
use crate::resources::user::User;
use crate::snowflake::Id;
use crate::teams::Team;

pub use self::commands::*;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

bitflags! {
    // From: https://github.com/discordjs/discord.js/blob/3c175cb5116fe50ba3084163565dd244a25b657f/src/util/ApplicationFlags.js
    pub struct ApplicationFlags: u64 {
        const MANAGED_EMOJI = 1<<2;
        const GROUP_DM_CREATE = 1<<4;
        const RPC_HAS_CONNECTED = 1<<11;
        const GATEWAY_PRESENCE = 1<<12;
        const GATEWAY_PRESENCE_LIMITED = 1<<13;
        const GATEWAY_GUILD_MEMBERS = 1<<14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1<<15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1<<16;
        const EMBEDDED = 1<<17;
    }
}

impl TryFrom<u64> for ApplicationFlags {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Self::from_bits(u).ok_or_else(|| Self::Error::new(u))
    }
}

impl From<ApplicationFlags> for u64 {
    fn from(uf: ApplicationFlags) -> u64 {
        uf.bits()
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationIcon {
    bare_path: String,
}

impl image::Image for ApplicationIcon {
    fn supports(&self, format: image::Format) -> bool {
        matches!(
            format,
            image::Format::Jpeg | image::Format::Png | image::Format::WebP
        )
    }

    fn bare_path(&self) -> &str {
        &self.bare_path
    }
}

impl ApplicationIcon {
    fn new(app_id: ApplicationId, hash: &str) -> Self {
        Self {
            bare_path: format!("app-icons/{}/{}", app_id, hash),
        }
    }
}

pub type ApplicationId = Id<Application>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    id: ApplicationId,
    name: String,
    icon: Option<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: bool,
    bot_require_code_grant: bool,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: User,
    summary: String,
    verify_key: String,
    team: Option<Team>,
    guild_id: Option<GuildId>,
    primary_sku_id: Option<SkuId>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: Option<IntegerEnum<ApplicationFlags>>,
}

impl Application {
    pub fn id(&self) -> ApplicationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> Option<ApplicationIcon> {
        self.icon
            .as_deref()
            .map(|i| ApplicationIcon::new(self.id, i))
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn rpc_origins(&self) -> Option<&[String]> {
        self.rpc_origins.as_deref()
    }

    pub fn bot_public(&self) -> bool {
        self.bot_public
    }

    pub fn bot_require_code_grant(&self) -> bool {
        self.bot_require_code_grant
    }

    pub fn terms_of_service_url(&self) -> Option<&str> {
        self.terms_of_service_url.as_deref()
    }

    pub fn privacy_policy_url(&self) -> Option<&str> {
        self.privacy_policy_url.as_deref()
    }

    pub fn owner(&self) -> &User {
        &self.owner
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn verify_key(&self) -> &str {
        &self.verify_key
    }

    pub fn team(&self) -> Option<&Team> {
        self.team.as_ref()
    }

    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    pub fn primary_sku_id(&self) -> Option<SkuId> {
        self.primary_sku_id
    }

    pub fn slug(&self) -> Option<&str> {
        self.slug.as_deref()
    }

    pub fn cover_image(&self) -> Option<ApplicationIcon> {
        self.cover_image
            .as_deref()
            .map(|i| ApplicationIcon::new(self.id, i))
    }

    pub fn try_flags(
        &self,
    ) -> Option<Result<ApplicationFlags, EnumFromIntegerError>> {
        self.flags.map(IntegerEnum::try_unwrap)
    }

    pub fn flags(&self) -> Option<ApplicationFlags> {
        self.flags.map(IntegerEnum::unwrap)
    }
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_application() {
        let json = json!({
            "bot_public": true,
            "bot_require_code_grant": false,
            "cover_image": "31deabb7e45b6c8ecfef77d2f99c81a5",
            "description": "Test",
            "guild_id": "290926798626357260",
            "icon": null,
            "id": "172150183260323840",
            "name": "Baba O-Riley",
            "owner": {
                "avatar": null,
                "discriminator": "1738",
                "flags": 1024,
                "id": "172150183260323840",
                "username": "i own a bot"
            },
            "primary_sku_id": "172150183260323840",
            "slug": "test",
            "summary": "This is a game",
            "team": {
                "icon": "dd9b7dcfdf5351b9c3de0fe167bacbe1",
                "id": "531992624043786253",
                "members": [
                {
                    "membership_state": 2,
                    "permissions": ["*"],
                    "team_id": "531992624043786253",
                    "user": {
                        "avatar": "d9e261cd35999608eb7e3de1fae3688b",
                        "discriminator": "0001",
                        "id": "511972282709709995",
                        "username": "Mr Owner"
                    }
                }
                ]
            },
            "verify_key": "1e0a356058d627ca38a5c8c9648818061d49e49bd9da9e3ab17d98ad4d6bg2u8"
        });

        let app: Application = serde_json::from_value(json).unwrap();

        assert_eq!(app.bot_public(), true);
        assert_eq!(app.bot_require_code_grant(), false);
        assert_eq!(
            app.cover_image().unwrap().bare_path(),
            "app-icons/172150183260323840/31deabb7e45b6c8ecfef77d2f99c81a5"
        );
        assert_eq!(app.description(), "Test");
        assert_eq!(app.guild_id(), Some(290926798626357260.into()));
        assert!(app.icon().is_none());
        assert_eq!(app.name(), "Baba O-Riley");

        assert_eq!(app.primary_sku_id(), Some(172150183260323840.into()));
        assert_eq!(app.slug(), Some("test"));
        assert_eq!(app.summary(), "This is a game");

        assert_eq!(
            app.verify_key(),
            "1e0a356058d627ca38a5c8c9648818061d49e49bd9da9e3ab17d98ad4d6bg2u8"
        );

        // TODO: Team

        let owner = app.owner();
        assert_eq!(owner.username(), "i own a bot");
        assert_eq!(owner.discriminator(), "1738");
        assert_eq!(owner.id(), 172150183260323840.into());
        assert_eq!(owner.avatar_or_default().bare_path(), "embed/avatars/3");
    }
}
