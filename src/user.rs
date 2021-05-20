// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::application::ApplicationId;
use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[doc(hidden)]
pub struct Bot {
    _p: (),
}

pub type BotId = Id<Bot>;

impl From<BotId> for UserId {
    fn from(bid: BotId) -> UserId {
        let id: u64 = bid.into();
        id.into()
    }
}

impl From<UserId> for BotId {
    fn from(uid: UserId) -> BotId {
        let id: u64 = uid.into();
        id.into()
    }
}

pub type UserId = Id<User>;

impl From<UserId> for ApplicationId {
    fn from(uid: UserId) -> ApplicationId {
        let id: u64 = uid.into();
        id.into()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "u64", into = "u64")]
pub enum PremiumKind {
    None,
    NitroClassic,
    Nitro,
    Other(u64),
}

impl From<u64> for PremiumKind {
    fn from(u: u64) -> PremiumKind {
        match u {
            0 => Self::None,
            1 => Self::NitroClassic,
            2 => Self::Nitro,
            other => Self::Other(other),
        }
    }
}

impl From<PremiumKind> for u64 {
    fn from(u: PremiumKind) -> Self {
        match u {
            PremiumKind::None => 0,
            PremiumKind::NitroClassic => 1,
            PremiumKind::Nitro => 2,
            PremiumKind::Other(other) => other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<u64>,
    #[serde(rename = "premium_type")]
    premium_kind: Option<PremiumKind>,
    public_flags: Option<u64>,
}

impl User {
    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }

    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    pub fn bot(&self) -> Option<bool> {
        self.bot
    }

    pub fn system(&self) -> Option<bool> {
        self.system
    }

    pub fn mfa_enabled(&self) -> Option<bool> {
        self.mfa_enabled
    }

    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    pub fn verified(&self) -> Option<bool> {
        self.verified
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn flags(&self) -> Option<u64> {
        self.flags
    }

    pub fn premium_kind(&self) -> Option<PremiumKind> {
        self.premium_kind
    }

    pub fn public_flags(&self) -> Option<u64> {
        self.public_flags
    }
}
