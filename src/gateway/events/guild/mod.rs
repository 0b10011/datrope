#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use time::serde::iso8601;
use time::OffsetDateTime;

use crate::api::objects::{
    channel::Channel,
    guild::{Guild, GuildMember, UnavailableGuild},
    guild_scheduled_event::GuildScheduledEvent,
    stage_instance::StageInstance,
};

use super::{presence::PresenceUpdate, voice::VoiceState};

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GuildCreate {
    Available(Guild, GuildCreateExtraFields),
    Unavailable(UnavailableGuild),
}

/// Discord docs: https://discord.com/developers/docs/topics/gateway-events#guild-create-guild-create-extra-fields
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildCreateExtraFields {
    #[cfg_attr(feature = "serde", serde(with = "iso8601"))]
    pub joined_at: OffsetDateTime,
    pub large: bool,
    pub unavailable: Option<bool>,
    pub member_count: usize,
    pub voice_states: Vec<VoiceState>,
    pub members: Vec<GuildMember>,
    pub channels: Option<Vec<Channel>>,
    pub threads: Vec<Channel>,
    pub presences: Vec<PresenceUpdate>,
    pub stage_instances: Vec<StageInstance>,
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
}
