#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{channel::ChannelId, guild::GuildId, permissions::RoleId};

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AutoModerationRule {
    pub id: AutoModerationRuleId,
    pub guild_id: GuildId,
    pub name: String,
    pub creator_id: CreatorId,
    pub event_type: EventType,
    pub trigger_type: TriggerType,
    pub trigger_metadata: TriggerMetadata,
    pub actions: Vec<AutoModerationAction>,
    pub enabled: bool,
    pub exempt_roles: Vec<RoleId>,
    pub exempt_channels: Vec<ChannelId>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AutoModerationRuleId(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CreatorId(pub String);

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum TriggerType {
    Keyword = 1,
    Spam = 3,
    KeywordPreset = 4,
    MentionSpam = 5,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TriggerMetadata {
    pub keyword_filter: Vec<String>,
    pub regex_patterns: Vec<String>,
    pub presets: Vec<KeywordPresetType>,
    pub allow_list: Vec<String>,
    pub mention_total_limit: u8,
    pub mention_raid_protection_enabled: bool,
}

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-preset-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum KeywordPresetType {
    Profanity = 1,
    SexualContent = 2,
    Slurs = 3,
}

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum EventType {
    MessageSend = 1,
}

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-auto-moderation-action-structure
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AutoModerationAction {
    pub r#type: ActionType,
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Option<ActionMetadata>,
}

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ActionType {
    BlockMessage = 1,
    SendAlertMessage = 2,
    Timeout = 3,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ActionMetadata {
    pub channel_id: ChannelId,
    pub duration_seconds: u32,
    #[cfg_attr(feature = "serde", serde(default))]
    pub custom_message: Option<String>,
}
