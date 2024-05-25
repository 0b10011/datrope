#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{application::ApplicationId, guild::IntegrationId, ImageHash};

bitflags::bitflags! {
    #[cfg_attr(feature = "clone", derive(Clone))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
    pub struct Permissions: u64 {
        const CreateInstantInvite = 1 << 0;
        const KickMembers = 1 << 1;
        const BanMembers = 1 << 2;
        const Administrator = 1 << 3;
        const ManageChannels = 1 << 4;
        const ManageGuild = 1 << 5;
        const AddReactions = 1 << 6;
        const ViewAuditLog = 1 << 7;
        const PrioritySpeaker = 1 << 8;
        const Stream = 1 << 9;
        const ViewChannel = 1 << 10;
        const SendMessages = 1 << 11;
        const SendTtsMessages = 1 << 12;
        const ManageMessages = 1 << 13;
        const EmbedLinks = 1 << 14;
        const AttachFiles = 1 << 15;
        const ReadMessageHistory = 1 << 16;
        const MentionEveryone = 1 << 17;
        const UseExternalEmojis = 1 << 18;
        const ViewGuildInsights = 1 << 19;
        const Connect = 1 << 20;
        const Speak = 1 << 21;
        const MuteMembers = 1 << 22;
        const DeafenMembers = 1 << 23;
        const MoveMembers = 1 << 24;
        const UseVad = 1 << 25;
        const ChangeNickname = 1 << 26;
        const ManageNicknames = 1 << 27;
        const ManageRoles = 1 << 28;
        const ManageWebhooks = 1 << 29;
        const ManageGuildExpressions = 1 << 30;
        const UseApplicationCommands = 1 << 31;
        const RequestToSpeak = 1 << 32;
        const ManageEvents = 1 << 33;
        const ManageThreads = 1 << 34;
        const CreatePublicThreads = 1 << 35;
        const CreatePrivateThreads = 1 << 36;
        const UseExternalStickers = 1 << 37;
        const SendMessagesInThreads = 1 << 38;
        const UseEmbeddedActivities = 1 << 39;
        const ModerateMembers = 1 << 40;
        const ViewCreatorMonetizationAnalytics = 1 << 41;
        const UseSoundboard = 1 << 42;
        const CreateGuildExpressions = 1 << 43;
        const CreateEvents = 1 << 44;
        const UseExternalSounds = 1 << 45;
        const SendVoiceMessages = 1 << 46;
        const SendPolls = 1 << 49;
    }
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub color: Rgb,
    pub hoist: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub icon: Option<ImageHash>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unicode_emoji: Option<String>,
    pub position: u64,
    pub permissions: Permissions,
    pub managed: bool,
    pub mentionable: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tags: Option<RoleTags>,
    pub flags: RoleFlags,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RoleId(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Rgb(pub u8, pub u8, pub u8);

/// Discord docs: https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RoleTags {
    #[cfg_attr(feature = "serde", serde(default))]
    pub bot_id: Option<ApplicationId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub integration_id: Option<IntegrationId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub premium_subscriber: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subscription_listing_id: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub available_for_purchase: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub guild_connections: bool,
}

/// Discord docs: https://discord.com/developers/docs/topics/permissions#role-object-role-flags
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum RoleFlags {
    InPrompt = 1 << 0,
}
