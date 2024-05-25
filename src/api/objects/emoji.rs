#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{permissions::RoleId, user::User};

/// Discord docs: https://discord.com/developers/docs/resources/emoji#emoji-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Emoji {
    pub id: Option<EmojiId>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roles: Vec<RoleId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: Option<User>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub require_colons: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub managed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub animated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub available: Option<bool>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EmojiId(pub String);
