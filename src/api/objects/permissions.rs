#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::flags;

use super::{application::ApplicationId, guild::IntegrationId, ImageHash};

flags!(permissions: i64 {
    CreateInstantInvite = 1 << 0,
    KickMembers = 1 << 1,
    BanMembers = 1 << 2,
    Administrator = 1 << 3,
    ManageChannels = 1 << 4,
    ManageGuild = 1 << 5,
    AddReactions = 1 << 6,
    ViewAuditLog = 1 << 7,
    PrioritySpeaker = 1 << 8,
    Stream = 1 << 9,
    ViewChannel = 1 << 10,
    SendMessages = 1 << 11,
    SendTtsMessages = 1 << 12,
    ManageMessages = 1 << 13,
    EmbedLinks = 1 << 14,
    AttachFiles = 1 << 15,
    ReadMessageHistory = 1 << 16,
    MentionEveryone = 1 << 17,
    UseExternalEmojis = 1 << 18,
    ViewGuildInsights = 1 << 19,
    Connect = 1 << 20,
    Speak = 1 << 21,
    MuteMembers = 1 << 22,
    DeafenMembers = 1 << 23,
    MoveMembers = 1 << 24,
    UseVad = 1 << 25,
    ChangeNickname = 1 << 26,
    ManageNicknames = 1 << 27,
    ManageRoles = 1 << 28,
    ManageWebhooks = 1 << 29,
    ManageGuildExpressions = 1 << 30,
    UseApplicationCommands = 1 << 31,
    RequestToSpeak = 1 << 32,
    ManageEvents = 1 << 33,
    ManageThreads = 1 << 34,
    CreatePublicThreads = 1 << 35,
    CreatePrivateThreads = 1 << 36,
    UseExternalStickers = 1 << 37,
    SendMessagesInThreads = 1 << 38,
    UseEmbeddedActivities = 1 << 39,
    ModerateMembers = 1 << 40,
    ViewCreatorMonetizationAnalytics = 1 << 41,
    UseSoundboard = 1 << 42,
    CreateGuildExpressions = 1 << 43,
    CreateEvents = 1 << 44,
    UseExternalSounds = 1 << 45,
    SendVoiceMessages = 1 << 46,
    SendPolls = 1 << 49,
});
pub use permissions::Flags as Permissions;

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

#[cfg(feature = "serde")]
#[test]
fn permissions() {
    let permissions_to_check = [
        ("0", flags!(permissions())),
        ("1", flags!(permissions(CreateInstantInvite))),
        ("2", flags!(permissions(KickMembers))),
        ("3", flags!(permissions(CreateInstantInvite | KickMembers))),
        ("4", flags!(permissions(BanMembers))),
        ("5", flags!(permissions(BanMembers | CreateInstantInvite))),
        ("6", flags!(permissions(BanMembers | KickMembers))),
        (
            "7",
            flags!(permissions(BanMembers | CreateInstantInvite | KickMembers)),
        ),
    ];
    for (expected_serialization, permissions) in permissions_to_check {
        let serialized = serde_json::to_string(&permissions).unwrap();
        assert_eq!(expected_serialization, serialized);
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(permissions, deserialized);
    }
}
