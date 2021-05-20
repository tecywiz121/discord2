use crate::snowflake::Id;

use serde::{Deserialize, Serialize};

use super::ApplicationId;

use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChoiceValue {
    Integer(u64),
    String(String),
}

impl ChoiceValue {
    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(u) => Some(u),
            _ => None,
        }
    }

    pub fn into_u64(self) -> Option<u64> {
        match self {
            Self::Integer(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Integer(u) => Some(*u),
            _ => None,
        }
    }
}

impl From<&str> for ChoiceValue {
    fn from(u: &str) -> Self {
        Self::String(u.to_owned())
    }
}

impl From<String> for ChoiceValue {
    fn from(u: String) -> Self {
        Self::String(u)
    }
}

impl From<u64> for ChoiceValue {
    fn from(u: u64) -> Self {
        Self::Integer(u)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ApplicationCommandOptionChoice {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    value: ChoiceValue,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "u64", into = "u64")]
pub enum ApplicationCommandOptionKind {
    SubCommand,
    SubCommandGroup,
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,

    Other(u64),
}

impl From<ApplicationCommandOptionKind> for u64 {
    fn from(u: ApplicationCommandOptionKind) -> Self {
        match u {
            ApplicationCommandOptionKind::SubCommand => 1,
            ApplicationCommandOptionKind::SubCommandGroup => 2,
            ApplicationCommandOptionKind::String => 3,
            ApplicationCommandOptionKind::Integer => 4,
            ApplicationCommandOptionKind::Boolean => 5,
            ApplicationCommandOptionKind::User => 6,
            ApplicationCommandOptionKind::Channel => 7,
            ApplicationCommandOptionKind::Role => 8,
            ApplicationCommandOptionKind::Mentionable => 9,

            ApplicationCommandOptionKind::Other(other) => other,
        }
    }
}

impl From<u64> for ApplicationCommandOptionKind {
    fn from(u: u64) -> Self {
        match u {
            1 => Self::SubCommand,
            2 => Self::SubCommandGroup,
            3 => Self::String,
            4 => Self::Integer,
            5 => Self::Boolean,
            6 => Self::User,
            7 => Self::Channel,
            8 => Self::Role,
            9 => Self::Mentionable,

            other => ApplicationCommandOptionKind::Other(other),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    kind: ApplicationCommandOptionKind,

    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    description: String,

    #[builder(default, setter(strip_option))]
    required: Option<bool>,

    #[builder(default, setter(into, strip_option))]
    choices: Option<Vec<ApplicationCommandOptionChoice>>,

    #[builder(default, setter(into, strip_option))]
    options: Option<Vec<ApplicationCommandOption>>,
}

pub type ApplicationCommandId = Id<ApplicationCommand>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommand {
    id: ApplicationCommandId,
    application_id: ApplicationId,
    name: String,
    description: String,
    options: Option<Vec<ApplicationCommandOption>>,
    default_permission: Option<bool>,
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct NewApplicationCommand {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    description: String,

    #[builder(default, setter(strip_option, into))]
    options: Option<Vec<ApplicationCommandOption>>,

    #[builder(default, setter(strip_option, into))]
    default_permission: Option<bool>,
}
