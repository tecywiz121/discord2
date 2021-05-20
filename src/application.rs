mod commands;

use crate::game_sdk::SkuId;
use crate::guild::GuildId;
use crate::snowflake::Id;
use crate::teams::Team;
use crate::user::User;

pub use self::commands::*;

use serde::{Deserialize, Serialize};

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
    flags: Option<u64>,
}

impl Application {
    pub fn id(&self) -> ApplicationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
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

    pub fn cover_image(&self) -> Option<&str> {
        self.cover_image.as_deref()
    }

    pub fn flags(&self) -> Option<u64> {
        self.flags
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(app.cover_image(), Some("31deabb7e45b6c8ecfef77d2f99c81a5"));
        assert_eq!(app.description(), "Test");
        assert_eq!(app.guild_id(), Some(290926798626357260.into()));
        assert_eq!(app.icon(), None);
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
        assert_eq!(owner.avatar(), None);
    }
}
