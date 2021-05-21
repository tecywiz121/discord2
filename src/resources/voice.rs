// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, FixedOffset};

use crate::resources::channel::ChannelId;
use crate::resources::guild::{GuildId, GuildMember};
use crate::resources::user::UserId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceState {
    guild_id: Option<GuildId>,
    channel_id: Option<ChannelId>,
    user_id: UserId,
    member: Option<GuildMember>,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: Option<bool>,
    self_video: Option<bool>,
    suppress: bool,
    request_to_speak_timestamp: Option<DateTime<FixedOffset>>,
}

impl VoiceState {
    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    pub fn channel_id(&self) -> Option<ChannelId> {
        self.channel_id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn member(&self) -> Option<&GuildMember> {
        self.member.as_ref()
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn deaf(&self) -> bool {
        self.deaf
    }

    pub fn mute(&self) -> bool {
        self.mute
    }

    pub fn self_deaf(&self) -> bool {
        self.self_deaf
    }

    pub fn self_mute(&self) -> bool {
        self.self_mute
    }

    pub fn self_stream(&self) -> Option<bool> {
        self.self_stream
    }

    pub fn self_video(&self) -> Option<bool> {
        self.self_video
    }

    pub fn suppress(&self) -> bool {
        self.suppress
    }

    pub fn request_to_speak_timestamp(&self) -> Option<DateTime<FixedOffset>> {
        self.request_to_speak_timestamp
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_voice_state() {
        let json = json!({
            "channel_id": "157733188964188161",
            "user_id": "80351110224678912",
            "session_id": "90326bd25d71d39b9ef95b299e3872ff",
            "deaf": false,
            "mute": false,
            "self_deaf": false,
            "self_mute": true,
            "suppress": false,
            "request_to_speak_timestamp": "2021-03-31T18:45:31.297561+00:00"
        });

        let voice: VoiceState = serde_json::from_value(json).unwrap();

        assert_eq!(voice.channel_id(), Some(157733188964188161.into()));
        assert_eq!(voice.user_id(), 80351110224678912.into());
        assert_eq!(voice.session_id(), "90326bd25d71d39b9ef95b299e3872ff");
        assert_eq!(voice.deaf(), false);
        assert_eq!(voice.mute(), false);
        assert_eq!(voice.self_deaf(), false);
        assert_eq!(voice.self_mute(), true);
        assert_eq!(voice.suppress(), false);

        let ts = Utc.ymd(2021, 03, 31).and_hms_micro(18, 45, 31, 297561);
        assert_eq!(voice.request_to_speak_timestamp().unwrap(), ts);
    }
}
