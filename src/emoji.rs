// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::permissions::RoleId;
use crate::snowflake::Id;
use crate::user::User;

use serde::{Deserialize, Serialize};

pub type EmojiId = Id<Emoji>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    id: Option<EmojiId>,
    name: Option<String>,
    roles: Option<Vec<RoleId>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}

impl Emoji {
    pub fn id(&self) -> Option<EmojiId> {
        self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn roles(&self) -> Option<&[RoleId]> {
        self.roles.as_deref()
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn require_colons(&self) -> Option<bool> {
        self.require_colons
    }

    pub fn managed(&self) -> Option<bool> {
        self.managed
    }

    pub fn animated(&self) -> Option<bool> {
        self.animated
    }

    pub fn available(&self) -> Option<bool> {
        self.available
    }
}

#[cfg(test)]
mod tests {
    use crate::user::UserFlags;

    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_emoji() {
        let json = json!({
            "id": "41771983429993937",
            "name": "LUL",
            "roles": ["41771983429993000", "41771983429993111"],
            "user": {
                "username": "Luigi",
                "discriminator": "0002",
                "id": "96008815106887111",
                "avatar": "5500909a3274e1812beb4e8de6631111",
                "public_flags": 131328
            },
            "require_colons": true,
            "managed": false,
            "animated": false
        });

        let emoji: Emoji = serde_json::from_value(json).unwrap();

        assert_eq!(emoji.id(), Some(41771983429993937.into()));
        assert_eq!(emoji.name(), Some("LUL"));
        assert_eq!(
            emoji.roles(),
            Some(&[41771983429993000u64.into(), 41771983429993111u64.into()]
                as &[_])
        );
        assert_eq!(emoji.require_colons(), Some(true));
        assert_eq!(emoji.managed(), Some(false));
        assert_eq!(emoji.animated(), Some(false));

        let user = emoji.user().unwrap();
        assert_eq!(user.username(), "Luigi");
        assert_eq!(user.discriminator(), "0002");
        assert_eq!(user.id(), 96008815106887111.into());
        assert_eq!(user.avatar(), Some("5500909a3274e1812beb4e8de6631111"));
        assert_eq!(
            user.public_flags(),
            Some(
                UserFlags::HOUSE_BALANCE
                    | UserFlags::EARLY_VERIFIED_BOT_DEVELOPER
            )
        );
    }

    #[test]
    fn deserialize_emoji_gateway_reaction_standard() {
        let json = json!({
            "id": null,
            "name": "🔥"
        });

        let emoji: Emoji = serde_json::from_value(json).unwrap();

        assert_eq!(emoji.id(), None);
        assert_eq!(emoji.name(), Some("\u{1F525}"));
    }

    #[test]
    fn deserialize_emoji_gateway_reaction_custom() {
        let json = json!({
            "id": "41771983429993937",
            "name": "LUL",
            "animated": true
        });

        let emoji: Emoji = serde_json::from_value(json).unwrap();

        assert_eq!(emoji.id(), Some(41771983429993937.into()));
        assert_eq!(emoji.name(), Some("LUL"));
        assert_eq!(emoji.animated(), Some(true))
    }

    #[test]
    fn deserialize_emoji_gateway_reaction_custom2() {
        let json = json!({
            "id": "41771983429993937",
            "name": null
        });

        let emoji: Emoji = serde_json::from_value(json).unwrap();

        assert_eq!(emoji.id(), Some(41771983429993937.into()));
        assert_eq!(emoji.name(), None);
    }
}
