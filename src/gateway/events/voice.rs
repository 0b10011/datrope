#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use time::serde::iso8601;
use time::OffsetDateTime;

use crate::api::objects::guild::GuildMember;

/// Discord docs: https://discord.com/developers/docs/resources/voice#voice-state-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user_id: String,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    #[cfg_attr(feature = "serde", serde(with = "iso8601"))]
    pub request_to_speak_timestamp: OffsetDateTime,
}
