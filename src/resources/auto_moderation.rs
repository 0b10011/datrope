#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AutoModerationRule {}

/// Discord docs: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TriggerType {
    Keyword = 1,
    Spam = 3,
    KeywordPreset = 4,
    MentionSpam = 5,
}
