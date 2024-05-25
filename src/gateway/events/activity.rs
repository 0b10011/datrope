#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Unimplemented;

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Activity {
    pub name: String,
    pub r#type: usize,
    pub url: Option<String>,
    // #[cfg_attr(feature = "serde", serde(with = "time::serde::timestamp::milliseconds::option"))]
    // i128 is broken with this
    pub created_at: usize,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<Unimplemented>,
    pub assets: Option<Unimplemented>,
    pub secrets: Option<Unimplemented>,
    pub instance: Option<bool>,
    pub flags: Option<usize>,
    pub buttons: Option<Unimplemented>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ActivityEmoji {
    name: String,
    id: Option<String>,
    animated: Option<bool>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ActivityTimestamps {
    #[cfg_attr(feature = "serde", serde(default))]
    // #[cfg_attr(feature = "serde", serde(with = "time::serde::timestamp::milliseconds::option"))]
    // i128 is broken with this
    start: Option<usize>,
    #[cfg_attr(feature = "serde", serde(default))]
    // #[cfg_attr(feature = "serde", serde(with = "time::serde::timestamp::milliseconds::option"))]
    // i128 is broken with this
    end: Option<usize>,
}
