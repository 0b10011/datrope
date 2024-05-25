#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Discord docs: https://discord.com/developers/docs/resources/channel#channels-resource
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Channel {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#message-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Message {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#message-interaction-metadata-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MessageInteractionMetadata {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#message-call-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MessageCall {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#message-reference-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MessageReference {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#followed-channel-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FollowedChannel {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#reaction-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ReactionObject {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#reaction-count-details-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ReactionCountDetails {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#overwrite-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Overwrite {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#thread-metadata-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ThreadMetadata {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#thread-member-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ThreadMember {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#default-reaction-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DefaultReaction {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#forum-tag-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ForumTag {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#embed-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Embed {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#attachment-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Attachment {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#allowed-mentions-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AllowedMentions {}

/// Discord docs: https://discord.com/developers/docs/resources/channel#role-subscription-data-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RoleSubscriptionData {}
