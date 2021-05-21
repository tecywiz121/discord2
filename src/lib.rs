// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod discord;
pub mod enums;
pub mod game_sdk;
pub mod gateway;
pub mod permissions;
pub mod resources;
pub mod snowflake;
mod str;
pub mod teams;

pub use self::discord::{Config, Discord, Error, Token};
