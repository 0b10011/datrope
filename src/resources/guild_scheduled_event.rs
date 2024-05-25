#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use time::serde::iso8601;
use time::OffsetDateTime;

use super::{
    auto_moderation::CreatorId,
    channel::ChannelId,
    guild::{GuildId, GuildMember},
    user::User,
    ImageHash,
};

/// Discord docs: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildScheduledEvent {
    pub id: GuildScheduledEventId,
    pub guild_id: GuildId,
    pub channel_id: Option<ChannelId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub creator_id: Option<CreatorId>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "iso8601"))]
    pub scheduled_start_time: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(with = "iso8601::option"))]
    pub scheduled_end_time: Option<OffsetDateTime>,
    pub privacy_level: GuildeScheduledEventPrivacyLevel,
    pub status: GuildScheduledEventStatus,
    pub entity_type: GuildScheduledEventEntityType,
    pub entity_id: Option<EntityId>,
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub creator: Option<User>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub user_count: Option<u64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub image: Option<ImageHash>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildScheduledEventId(pub String);

/// Discord docs: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum GuildeScheduledEventPrivacyLevel {
    GuildOnly = 2,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EntityId(pub String);

/// Discord docs: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum GuildScheduledEventEntityType {
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

/// Discord docs: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum GuildScheduledEventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

/// Discord docs: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildScheduledEventEntityMetadata {
    #[cfg_attr(feature = "serde", serde(default))]
    pub location: Option<String>,
}

/// Discord docs: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-user-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildScheduledEventUser {
    pub guild_scheduled_event_id: GuildScheduledEventId,
    pub user: User,
    #[cfg_attr(feature = "serde", serde(default))]
    pub member: Option<GuildMember>,
}
