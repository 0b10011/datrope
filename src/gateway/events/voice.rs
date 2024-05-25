#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::resources::guild::GuildMember;

/// Discord docs: https://discord.com/developers/docs/resources/voice#voice-state-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VoiceState {
    guild_id: Option<String>,
    channel_id: Option<String>,
    user_id: String,
    member: Option<GuildMember>,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: Option<bool>,
    self_video: bool,
    suppress: bool,
    #[cfg_attr(feature = "serde", serde(with = "iso8601"))]
    request_to_speak_timestamp: OffsetDateTime,
}
