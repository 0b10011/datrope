#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Discord docs: https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-structure
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Webhook {}

/// Discord docs: https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
    Application = 3,
}
