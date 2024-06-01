#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::activity::Activity;

/// Discord docs: https://discord.com/developers/docs/topics/gateway-events#update-presence
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PresenceUpdate {
    /// Unix time (in milliseconds) of when the client went idle, or `None` if the client is not idle.
    /// Example: `Some(time::macros::datetime!(2024-05-20 20:33:05 EDT).unix_timestamp_nanos() / 1_000)`.
    #[cfg_attr(feature = "serde", serde(default))]
    // #[serde(with = "time::serde::timestamp::milliseconds::option")]
    // i128 is broken with this
    pub since: Option<usize>,

    /// User's activities
    pub activities: Vec<Activity>,

    /// User's new status
    pub status: Status,

    /// Whether the client is AFK
    pub afk: Option<bool>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Status {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}
