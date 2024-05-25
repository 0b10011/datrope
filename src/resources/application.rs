use enumset::EnumSetType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    guild::{GuildId, UnavailableGuild},
    team::Team,
    user::UnavailableUser,
    ImageHash,
};

/// Discord docs: https://discord.com/developers/docs/resources/application#application-resource
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Application {
    id: ApplicationId,
    name: String,
    icon: Option<ImageHash>,
    description: String,
    #[cfg_attr(feature = "serde", serde(default))]
    rpc_origins: Vec<Url>,
    bot_public: bool,
    bot_require_code_grant: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    bot: Option<UnavailableUser>,
    #[cfg_attr(feature = "serde", serde(default))]
    terms_of_service_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    privacy_policy_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    owner: Option<UnavailableUser>,
    /// Deprecated and always an empty string.
    summary: String,
    verify_key: String,
    team: Option<Team>,
    #[cfg_attr(feature = "serde", serde(default))]
    guild_id: Option<GuildId>,
    #[cfg_attr(feature = "serde", serde(default))]
    guild: Option<UnavailableGuild>,
    #[cfg_attr(feature = "serde", serde(default))]
    primary_sku_id: Option<SkuId>,
    #[cfg_attr(feature = "serde", serde(default))]
    slug: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    cover_image: Option<ImageHash>,
    #[cfg_attr(feature = "serde", serde(default))]
    flags: Option<ApplicationFlags>,
    #[cfg_attr(feature = "serde", serde(default))]
    approximate_guild_count: Option<usize>,
    #[cfg_attr(feature = "serde", serde(default))]
    redirect_uris: Vec<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    interactions_endpoint_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    role_connections_verification_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    tags: Vec<Tag>,
    #[cfg_attr(feature = "serde", serde(default))]
    install_params: Option<InstallParams>,
    #[cfg_attr(feature = "serde", serde(default))]
    integration_types_config: Option<IntegrationTypesConfigurationMap>,
    #[cfg_attr(feature = "serde", serde(default))]
    custom_install_url: Option<Url>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum IntegrationType {
    GuildInstall = 0,
    UserInstall = 1,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IntegrationTypesConfigurationMap {
    guild_install: IntegrationTypeConfiguration,
    user_install: IntegrationTypeConfiguration,
}

/// Discord docs: https://discord.com/developers/docs/resources/application#application-object-application-integration-type-configuration-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IntegrationTypeConfiguration {
    #[cfg_attr(feature = "serde", serde(default))]
    oauth2_install_params: Option<InstallParams>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Tag(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SkuId(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationId(String);

/// Discord docs: https://discord.com/developers/docs/resources/application#install-params-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InstallParams {
    scopes: Scopes,
    permissions: Permissions,
}

/// https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(EnumSetType)]
pub enum Scopes {
    #[cfg_attr(feature = "serde", serde(rename = "activities.read"))]
    ActivitiesRead,
    #[cfg_attr(feature = "serde", serde(rename = "activities.write"))]
    ActivitiesWrite,
    #[cfg_attr(feature = "serde", serde(rename = "applications.builds.read"))]
    ApplicationsBuildsRead,
    #[cfg_attr(feature = "serde", serde(rename = "applications.builds.upload"))]
    ApplicationsBuildsUpload,
    #[cfg_attr(feature = "serde", serde(rename = "applications.commands"))]
    ApplicationsCommands,
    #[cfg_attr(feature = "serde", serde(rename = "applications.commands.update"))]
    ApplicationsCommandsUpdate,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "applications.commands.permissions.update")
    )]
    ApplicationsCommandsPermissionsUpdate,
    #[cfg_attr(feature = "serde", serde(rename = "applications.entitlements"))]
    ApplicationsEntitlements,
    #[cfg_attr(feature = "serde", serde(rename = "applications.store.update"))]
    ApplicationsStoreUpdate,
    Bot,
    Connections,
    #[cfg_attr(feature = "serde", serde(rename = "dm_channels.read"))]
    DmChannelsRead,
    Email,
    #[cfg_attr(feature = "serde", serde(rename = "gdm.join"))]
    GdmJoin,
    Guilds,
    #[cfg_attr(feature = "serde", serde(rename = "guilds.join"))]
    GuildsJoin,
    #[cfg_attr(feature = "serde", serde(rename = "guild.members.read"))]
    GuildsMembersRead,
    Identify,
    #[cfg_attr(feature = "serde", serde(rename = "messages.read"))]
    MessagesRead,
    #[cfg_attr(feature = "serde", serde(rename = "relationships.read"))]
    RelationshipsRead,
    #[cfg_attr(feature = "serde", serde(rename = "role_connections.write"))]
    RoleConnectionsWrite,
    Rpc,
    #[cfg_attr(feature = "serde", serde(rename = "rpc.activities.write"))]
    RpcActivitiesWrite,
    #[cfg_attr(feature = "serde", serde(rename = "rep.notifications.read"))]
    RpcNotificationsRead,
    #[cfg_attr(feature = "serde", serde(rename = "rpc.voice.read"))]
    RpcVoiceRead,
    #[cfg_attr(feature = "serde", serde(rename = "rpc.voice.write"))]
    RpcVoiceWrite,
    Voice,
    #[cfg_attr(feature = "serde", serde(rename = "webhook.incoming"))]
    WebhookIncoming,
}

bitflags::bitflags! {
    #[cfg_attr(feature = "clone", derive(Clone))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
    pub struct ApplicationFlags: u64 {
        const ApplicationAutoModerationRuleCreateBadge = 1 << 6;
        const GatewayPresence = 1 << 12;
        const GatewayPresenceLimited = 1 << 13;
        const GatewayGuildMembers = 1 << 14;
        const GatewayGuildMembersLimited = 1 << 15;
        const VerificationPendingGuildLimit = 1 << 16;
        const Embedded = 1 << 17;
        const GatewayMessageContent = 1 << 18;
        const GatewayMessagecontentLimited = 1 << 19;
        const ApplicationCommandBadge = 1 << 23;
    }
}

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
