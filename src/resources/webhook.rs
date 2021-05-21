// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::enums::{EnumFromIntegerError, IntegerEnum};
use crate::resources::application::ApplicationId;
use crate::resources::channel::{Channel, ChannelId};
use crate::resources::guild::GuildId;
use crate::resources::user::User;
use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

pub type WebhookId = Id<Webhook>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceGuild {
    id: GuildId,
    name: Option<String>,
    icon: Option<String>,
}

impl SourceGuild {
    pub fn id(&self) -> GuildId {
        self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    id: WebhookId,
    #[serde(rename = "type")]
    kind: IntegerEnum<WebhookKind>,
    guild_id: Option<GuildId>,
    channel_id: Option<ChannelId>,
    user: Option<User>,
    name: Option<String>,
    avatar: Option<String>,
    token: Option<String>,
    application_id: Option<ApplicationId>,
    source_guild: Option<SourceGuild>,
    source_channel: Option<Channel>,
    url: Option<String>,
}

impl Webhook {
    pub fn id(&self) -> WebhookId {
        self.id
    }

    pub fn try_kind(&self) -> Result<WebhookKind, EnumFromIntegerError> {
        self.kind.try_unwrap()
    }

    pub fn kind(&self) -> WebhookKind {
        self.kind.unwrap()
    }

    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    pub fn channel_id(&self) -> Option<ChannelId> {
        self.channel_id
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn application_id(&self) -> Option<ApplicationId> {
        self.application_id
    }

    pub fn source_guild(&self) -> Option<&SourceGuild> {
        self.source_guild.as_ref()
    }

    pub fn source_channel(&self) -> Option<&Channel> {
        self.source_channel.as_ref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookKind {
    Incoming,
    ChannelFollower,
    Application,
}

impl TryFrom<u64> for WebhookKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::Incoming,
            2 => Self::ChannelFollower,
            3 => Self::Application,
            raw => return Err(EnumFromIntegerError::new(raw)),
        };

        Ok(r)
    }
}

impl From<WebhookKind> for u64 {
    fn from(u: WebhookKind) -> Self {
        match u {
            WebhookKind::Incoming => 1,
            WebhookKind::ChannelFollower => 2,
            WebhookKind::Application => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resources::user::UserFlags;

    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_webhook_incoming() {
        let json = json!({
            "name": "test webhook",
            "type": 1,
            "channel_id": "199737254929760256",
            "token": "3d89bb7572e0fb30d8128367b3b1b44fecd1726de135cbe28a41f8b2f777c372ba2939e72279b94526ff5d1bd4358d65cf11",
            "avatar": null,
            "guild_id": "199737254929760256",
            "id": "223704706495545344",
            "application_id": null,
            "user": {
                "username": "test",
                "discriminator": "7479",
                "id": "190320984123768832",
                "avatar": "b004ec1740a63ca06ae2e14c5cee11f3",
                "public_flags": 131328
            }
        });

        let wh: Webhook = serde_json::from_value(json).unwrap();

        assert_eq!(wh.name(), Some("test webhook"));
        assert_eq!(wh.kind(), WebhookKind::Incoming);
        assert_eq!(wh.channel_id(), Some(199737254929760256.into()));
        assert_eq!(
            wh.token(),
            Some("3d89bb7572e0fb30d8128367b3b1b44fecd1726de135cbe28a41f8b2f777c372ba2939e72279b94526ff5d1bd4358d65cf11")
        );
        assert_eq!(wh.avatar(), None);
        assert_eq!(wh.guild_id(), Some(199737254929760256.into()));
        assert_eq!(wh.id(), 223704706495545344.into());
        assert_eq!(wh.application_id(), None);

        let user = wh.user().unwrap();
        assert_eq!(user.username(), "test");
        assert_eq!(user.discriminator(), "7479");
        assert_eq!(user.id(), 190320984123768832.into());
        assert_eq!(user.avatar(), Some("b004ec1740a63ca06ae2e14c5cee11f3"));
        assert_eq!(
            user.public_flags(),
            Some(
                UserFlags::HOUSE_BALANCE
                    | UserFlags::EARLY_VERIFIED_BOT_DEVELOPER
            )
        );
    }

    #[test]
    fn deserialize_webhook_channel_follower() {
        let json = json!({
            "type": 2,
            "id": "752831914402115456",
            "name": "Guildy name",
            "avatar": "bb71f469c158984e265093a81b3397fb",
            "channel_id": "561885260615255432",
            "guild_id": "56188498421443265",
            "application_id": null,
            "source_guild": {
                "id": "56188498421476534",
                "name": "Guildy name",
                "icon": "bb71f469c158984e265093a81b3397fb"
            },
            "source_channel": {
                "id": "5618852344134324",
                "name": "announcements"
            },
            "user": {
                "username": "test",
                "discriminator": "7479",
                "id": "190320984123768832",
                "avatar": "b004ec1740a63ca06ae2e14c5cee11f3",
                "public_flags": 131328
            }
        });

        let wh: Webhook = serde_json::from_value(json).unwrap();

        assert_eq!(wh.name(), Some("Guildy name"));
        assert_eq!(wh.kind(), WebhookKind::ChannelFollower);
        assert_eq!(wh.channel_id(), Some(561885260615255432.into()));
        assert_eq!(wh.token(), None);
        assert_eq!(wh.avatar(), Some("bb71f469c158984e265093a81b3397fb"));
        assert_eq!(wh.guild_id(), Some(56188498421443265.into()));
        assert_eq!(wh.id(), 752831914402115456.into());
        assert_eq!(wh.application_id(), None);

        let sg = wh.source_guild().unwrap();
        assert_eq!(sg.id(), 56188498421476534.into());
        assert_eq!(sg.name(), Some("Guildy name"));
        assert_eq!(sg.icon(), Some("bb71f469c158984e265093a81b3397fb"));

        let sc = wh.source_channel().unwrap();
        assert_eq!(sc.id(), 5618852344134324.into());
        assert_eq!(sc.name(), Some("announcements"));

        let user = wh.user().unwrap();
        assert_eq!(user.username(), "test");
        assert_eq!(user.discriminator(), "7479");
        assert_eq!(user.id(), 190320984123768832.into());
        assert_eq!(user.avatar(), Some("b004ec1740a63ca06ae2e14c5cee11f3"));
        assert_eq!(
            user.public_flags(),
            Some(
                UserFlags::HOUSE_BALANCE
                    | UserFlags::EARLY_VERIFIED_BOT_DEVELOPER
            )
        );
    }

    #[test]
    fn deserialize_webhook_application() {
        let json = json!({
            "type": 3,
            "id": "658822586720976555",
            "name": "Clyde",
            "avatar": "689161dc90ac261d00f1608694ac6bfd",
            "channel_id": null,
            "guild_id": null,
            "application_id": "658822586720976555"
        });

        let wh: Webhook = serde_json::from_value(json).unwrap();

        assert_eq!(wh.name(), Some("Clyde"));
        assert_eq!(wh.kind(), WebhookKind::Application);
        assert_eq!(wh.channel_id(), None);
        assert_eq!(wh.token(), None);
        assert_eq!(wh.avatar(), Some("689161dc90ac261d00f1608694ac6bfd"));
        assert_eq!(wh.guild_id(), None);
        assert_eq!(wh.id(), 658822586720976555.into());
        assert_eq!(wh.application_id(), Some(658822586720976555.into()));
        assert!(wh.guild_id().is_none());
        assert!(wh.channel_id().is_none());
    }
}
