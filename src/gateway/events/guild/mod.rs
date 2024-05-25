#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

use crate::resources::{
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
    joined_at: OffsetDateTime,
    large: bool,
    unavailable: Option<bool>,
    member_count: usize,
    voice_states: Vec<VoiceState>,
    members: Vec<GuildMember>,
    channels: Option<Vec<Channel>>,
    threads: Vec<Channel>,
    presences: Vec<PresenceUpdate>,
    stage_instances: Vec<StageInstance>,
    guild_scheduled_events: Vec<GuildScheduledEvent>,
}
