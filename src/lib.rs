// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod application;
pub mod audit_log;
pub mod channel;
mod discord;
pub mod emoji;
pub mod game_sdk;
pub mod gateway;
pub mod guild;
pub mod guild_template;
pub mod invite;
pub mod permissions;
pub mod snowflake;
pub mod stage_instance;
mod str;
pub mod teams;
pub mod user;
pub mod voice;
pub mod webhook;

pub use self::discord::{Config, Discord, Error, Token};
