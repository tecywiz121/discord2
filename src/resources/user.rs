// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

use crate::enums::{EnumFromIntegerError, IntegerEnum};
use crate::resources::application::ApplicationId;
use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PremiumKind {
    None,
    NitroClassic,
    Nitro,
}

impl TryFrom<u64> for PremiumKind {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<PremiumKind, Self::Error> {
        let r = match u {
            0 => Self::None,
            1 => Self::NitroClassic,
            2 => Self::Nitro,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

impl From<PremiumKind> for u64 {
    fn from(u: PremiumKind) -> Self {
        match u {
            PremiumKind::None => 0,
            PremiumKind::NitroClassic => 1,
            PremiumKind::Nitro => 2,
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
    flags: Option<IntegerEnum<UserFlags>>,
    #[serde(rename = "premium_type")]
    premium_kind: Option<IntegerEnum<PremiumKind>>,
    public_flags: Option<IntegerEnum<UserFlags>>,
}

bitflags! {
    pub struct UserFlags: u64 {
        const NONE = 0;
        const DISCORD_EMPLOYEE = 1<<0;
        const PARTNERED_SERVER_OWNER = 1<<1;
        const HYPE_SQUAD_EVENTS = 1<<2;
        const BUG_HUNTER_LEVEL_1 = 1<<3;
        const HOUSE_BRAVERY = 1<<6;
        const HOUSE_BRILLIANCE = 1<<7;
        const HOUSE_BALANCE = 1<<8;
        const EARLY_SUPPORTER = 1<<9;
        const TEAM_USER = 1<<10;
        const BUG_HUNTER_LEVEL_2 = 1<<14;
        const VERIFIED_BOT = 1<<16;
        const EARLY_VERIFIED_BOT_DEVELOPER = 1<<17;
    }
}

impl TryFrom<u64> for UserFlags {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Self::from_bits(u).ok_or_else(|| Self::Error::new(u))
    }
}

impl From<UserFlags> for u64 {
    fn from(uf: UserFlags) -> u64 {
        uf.bits()
    }
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

    pub fn try_flags(&self) -> Option<Result<UserFlags, EnumFromIntegerError>> {
        self.flags.map(IntegerEnum::try_unwrap)
    }

    pub fn flags(&self) -> Option<UserFlags> {
        self.flags.map(IntegerEnum::unwrap)
    }

    pub fn try_premium_kind(
        &self,
    ) -> Option<Result<PremiumKind, EnumFromIntegerError>> {
        self.premium_kind.map(IntegerEnum::try_unwrap)
    }

    pub fn premium_kind(&self) -> Option<PremiumKind> {
        self.premium_kind.map(IntegerEnum::unwrap)
    }

    pub fn try_public_flags(
        &self,
    ) -> Option<Result<UserFlags, EnumFromIntegerError>> {
        self.public_flags.map(IntegerEnum::try_unwrap)
    }

    pub fn public_flags(&self) -> Option<UserFlags> {
        self.public_flags.map(IntegerEnum::unwrap)
    }
}
