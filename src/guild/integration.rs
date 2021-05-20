// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, FixedOffset};

use crate::permissions::RoleId;
use crate::snowflake::Id;
use crate::user::User;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "u64", into = "u64")]
pub enum IntegrationExpireBehavior {
    RemoveRole,
    Kick,

    Other(u64),
}

impl From<IntegrationExpireBehavior> for u64 {
    fn from(u: IntegrationExpireBehavior) -> Self {
        match u {
            IntegrationExpireBehavior::RemoveRole => 0,
            IntegrationExpireBehavior::Kick => 1,
            IntegrationExpireBehavior::Other(other) => other,
        }
    }
}

impl From<u64> for IntegrationExpireBehavior {
    fn from(u: u64) -> Self {
        match u {
            0 => Self::RemoveRole,
            1 => Self::Kick,
            other => Self::Other(other),
        }
    }
}

pub type IntegrationId = Id<Integration>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    id: IntegrationId,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    enabled: bool,
    syncing: Option<bool>,
    role_id: Option<RoleId>,
    enable_emoticons: Option<bool>,
    expire_behavior: Option<IntegrationExpireBehavior>,
    expire_grace_period: Option<u64>,
    user: Option<User>,
    account: IntegrationAccount,
    synced_at: DateTime<FixedOffset>,
    subscriber_count: Option<u64>,
    revoked: Option<bool>,
    application: Option<IntegrationApplication>,
}

impl Integration {
    pub fn id(&self) -> IntegrationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn syncing(&self) -> Option<bool> {
        self.syncing
    }

    pub fn role_id(&self) -> Option<RoleId> {
        self.role_id
    }

    pub fn enable_emoticons(&self) -> Option<bool> {
        self.enable_emoticons
    }

    pub fn expire_behavior(&self) -> Option<IntegrationExpireBehavior> {
        self.expire_behavior
    }

    pub fn expire_grace_period(&self) -> Option<u64> {
        self.expire_grace_period
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn account(&self) -> &IntegrationAccount {
        &self.account
    }

    pub fn synced_at(&self) -> DateTime<FixedOffset> {
        self.synced_at
    }

    pub fn subscriber_count(&self) -> Option<u64> {
        self.subscriber_count
    }

    pub fn revoked(&self) -> Option<bool> {
        self.revoked
    }

    pub fn application(&self) -> Option<&IntegrationApplication> {
        self.application.as_ref()
    }
}

pub type IntegrationAccountId = Id<IntegrationAccount>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAccount {
    id: IntegrationAccountId,
    name: String,
}

impl IntegrationAccount {
    pub fn id(&self) -> IntegrationAccountId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub type IntegrationApplicationId = Id<IntegrationApplication>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationApplication {
    id: IntegrationApplicationId,
    name: String,
    icon: Option<String>,
    description: String,
    summary: String,
    bot: Option<User>,
}

impl IntegrationApplication {
    pub fn id(&self) -> IntegrationApplicationId {
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

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn bot(&self) -> Option<&User> {
        self.bot.as_ref()
    }
}
