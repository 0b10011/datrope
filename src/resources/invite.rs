#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Discord docs: https://discord.com/developers/docs/resources/invite#invite-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Invite {}

/// Discord docs: https://discord.com/developers/docs/resources/invite#invite-object-invite-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InviteType {
    Guild = 0,
    GroupDm = 1,
    Friend = 2,
}

/// Discord docs: https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InviteTargetType {
    Stream = 1,
    EmbeddedApplication = 2,
}

/// Discord docs: https://discord.com/developers/docs/resources/invite#invite-metadata-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InviteMetadata {}

/// Discord docs: https://discord.com/developers/docs/resources/invite#invite-stage-instance-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InviteStageInstanceObject {}
