#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Discord docs: https://discord.com/developers/docs/resources/poll#poll-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Poll {}

/// Discord docs: https://discord.com/developers/docs/resources/poll#poll-create-request-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PollCreateRequest {}

/// Discord docs: https://discord.com/developers/docs/resources/poll#layout-type
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum LayoutType {
    Default = 1,
}

/// Discord docs: https://discord.com/developers/docs/resources/poll#poll-media-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PollMedia {}

/// Discord docs: https://discord.com/developers/docs/resources/poll#poll-answer-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PollAnswer {}

/// Discord docs: https://discord.com/developers/docs/resources/poll#poll-results-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PollAnswers {}
