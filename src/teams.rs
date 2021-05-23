// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::enums::{EnumFromIntegerError, IntegerEnum};
use crate::image;
use crate::resources::user::{User, UserId};
use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

pub type TeamId = Id<Team>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MembershipState {
    Invited,
    Accepted,
}

impl From<MembershipState> for u64 {
    fn from(u: MembershipState) -> Self {
        match u {
            MembershipState::Invited => 1,
            MembershipState::Accepted => 2,
        }
    }
}

impl TryFrom<u64> for MembershipState {
    type Error = EnumFromIntegerError;

    fn try_from(u: u64) -> Result<Self, Self::Error> {
        let r = match u {
            1 => Self::Invited,
            2 => Self::Accepted,
            other => return Err(EnumFromIntegerError::new(other)),
        };

        Ok(r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    membership_state: IntegerEnum<MembershipState>,
    permissions: Vec<String>,
    team_id: TeamId,
    user: User,
}

impl TeamMember {
    pub fn try_membership_state(
        &self,
    ) -> Result<MembershipState, EnumFromIntegerError> {
        self.membership_state.try_unwrap()
    }

    pub fn membership_state(&self) -> MembershipState {
        self.membership_state.unwrap()
    }

    pub fn permissions(&self) -> &[String] {
        &self.permissions
    }

    pub fn team_id(&self) -> TeamId {
        self.team_id
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

#[derive(Debug, Clone)]
pub struct TeamIcon {
    bare_path: String,
}

impl image::Image for TeamIcon {
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

impl TeamIcon {
    fn new(app_id: TeamId, hash: &str) -> Self {
        Self {
            bare_path: format!("team-icons/{}/{}", app_id, hash),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    id: TeamId,
    icon: Option<String>,
    members: Vec<TeamMember>,
    name: Option<String>,
    owner_user_id: Option<UserId>,
}

impl Team {
    pub fn id(&self) -> TeamId {
        self.id
    }

    pub fn icon(&self) -> Option<TeamIcon> {
        self.icon.as_deref().map(|i| TeamIcon::new(self.id, i))
    }

    pub fn members(&self) -> &[TeamMember] {
        self.members.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn owner_user_id(&self) -> Option<UserId> {
        self.owner_user_id
    }
}
