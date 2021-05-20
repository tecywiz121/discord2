use crate::channel::ChannelId;
use crate::guild::IntegrationId;
use crate::snowflake::Id;
use crate::user::BotId;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type RoleId = Id<Role>;

impl From<ChannelId> for RoleId {
    fn from(cid: ChannelId) -> Self {
        let id: u64 = cid.into();
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
    permissions: String,
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

    pub fn permissions(&self) -> &str {
        &self.permissions
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
