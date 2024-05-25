#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use time::serde::iso8601;
use time::OffsetDateTime;

use super::{
    auto_moderation::CreatorId,
    guild::{GuildId, UnavailableGuild},
    user::User,
};

/// Discord docs: https://discord.com/developers/docs/resources/guild-template#guild-template-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildTemplate {
    pub code: GuildTemplateId,
    pub name: String,
    pub description: Option<String>,
    pub usage_count: u64,
    pub creator_id: CreatorId,
    pub creator: User,
    #[cfg_attr(feature = "serde", serde(with = "iso8601"))]
    pub created_at: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "iso8601"))]
    pub updated_at: OffsetDateTime,
    pub source_guild_id: GuildId,
    pub serialized_source_guild: UnavailableGuild,
    pub is_dirty: Option<bool>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GuildTemplateId(pub String);
