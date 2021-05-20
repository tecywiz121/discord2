// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::snowflake::Id;
use crate::user::{User, UserId};

use serde::{Deserialize, Serialize};

pub type TeamId = Id<Team>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "u64", into = "u64")]
pub enum MembershipState {
    Invited,
    Accepted,
    Other(u64),
}

impl From<MembershipState> for u64 {
    fn from(u: MembershipState) -> Self {
        match u {
            MembershipState::Invited => 1,
            MembershipState::Accepted => 2,
            MembershipState::Other(other) => other,
        }
    }
}

impl From<u64> for MembershipState {
    fn from(u: u64) -> Self {
        match u {
            1 => Self::Invited,
            2 => Self::Accepted,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    membership_state: MembershipState,
    permissions: Vec<String>,
    team_id: TeamId,
    user: User,
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

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
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
