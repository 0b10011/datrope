use crate::flags;
use enumset::EnumSetType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    guild::{GuildId, UnavailableGuild},
    permissions::Permissions,
    team::Team,
    user::UnavailableUser,
    ImageHash,
};

/// Discord docs: https://discord.com/developers/docs/resources/application#application-resource
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Application {
    pub id: ApplicationId,
    pub name: String,
    pub icon: Option<ImageHash>,
    pub description: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rpc_origins: Vec<Url>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bot: Option<UnavailableUser>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub terms_of_service_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub privacy_policy_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub owner: Option<UnavailableUser>,
    /// Deprecated and always an empty string.
    pub summary: String,
    pub verify_key: String,
    pub team: Option<Team>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub guild_id: Option<GuildId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub guild: Option<UnavailableGuild>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub primary_sku_id: Option<SkuId>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub slug: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cover_image: Option<ImageHash>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub flags: Option<ApplicationFlags>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub approximate_guild_count: Option<usize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub redirect_uris: Vec<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interactions_endpoint_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub role_connections_verification_url: Option<Url>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tags: Vec<Tag>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub install_params: Option<InstallParams>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub integration_types_config: Option<IntegrationTypesConfigurationMap>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub custom_install_url: Option<Url>,
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
    pub guild_install: IntegrationTypeConfiguration,
    pub user_install: IntegrationTypeConfiguration,
}

/// Discord docs: https://discord.com/developers/docs/resources/application#application-object-application-integration-type-configuration-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IntegrationTypeConfiguration {
    #[cfg_attr(feature = "serde", serde(default))]
    pub oauth2_install_params: Option<InstallParams>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Tag(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SkuId(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationId(pub String);

/// Discord docs: https://discord.com/developers/docs/resources/application#install-params-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InstallParams {
    pub scopes: Scopes,
    pub permissions: Permissions,
}

/// Discord docs: https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
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

flags!(application_flags {
    ApplicationAutoModerationRuleCreateBadge = 1 << 6,
    GatewayPresence = 1 << 12,
    GatewayPresenceLimited = 1 << 13,
    GatewayGuildMembers = 1 << 14,
    GatewayGuildMembersLimited = 1 << 15,
    VerificationPendingGuildLimit = 1 << 16,
    Embedded = 1 << 17,
    GatewayMessageContent = 1 << 18,
    GatewayMessagecontentLimited = 1 << 19,
    ApplicationCommandBadge = 1 << 23,
});
pub use application_flags::Flags as ApplicationFlags;

#[cfg(feature = "serde")]
#[test]
fn flags() {
    let flags = flags!(application_flags(
        ApplicationAutoModerationRuleCreateBadge | GatewayPresence
    ));
    let serialized = serde_json::to_string(&flags).unwrap();
    assert_eq!("4160", serialized);
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(flags, deserialized);
}
