#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Discord docs: https://discord.com/developers/docs/resources/emoji#emoji-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Emoji {}
