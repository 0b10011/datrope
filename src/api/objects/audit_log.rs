#[cfg(not(feature = "serde"))]
use std::{any::Any, rc::Rc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_json::Value;

use super::{
    application::{ApplicationId, IntegrationType},
    application_command::ApplicationCommand,
    auto_moderation::AutoModerationRule,
    channel::{Channel, ChannelId, MessageId},
    guild::UnavailableIntegration,
    guild_scheduled_event::GuildScheduledEvent,
    user::{User, UserId},
    webhook::Webhook,
};

/// Discord docs: https://discord.com/developers/docs/resources/audit-log#audit-log-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuditLog {
    pub application_commands: Vec<ApplicationCommand>,
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub auto_moderation_rules: Vec<AutoModerationRule>,
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub integrations: Vec<UnavailableIntegration>,
    pub threads: Vec<Channel>,
    pub users: Vec<User>,
    pub webhooks: Vec<Webhook>,
}

/// Discord docs: https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuditLogEntry {
    pub target_id: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub changes: Vec<AuditLogChange>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub user_id: Option<UserId>,
    pub id: AuditLogEntryId,
    pub action_type: AuditLogEvent,
    #[cfg_attr(feature = "serde", serde(default))]
    pub options: Option<OptionalAuditEntryInfo>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reason: String,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuditLogEntryId(pub String);

/// Discord docs: https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-events
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AuditLogEvent {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
    ApplicationCommandPermissionUpdate = 121,
    AutoModerationRuleCreate = 140,
    AutoModerationRuleUpdate = 141,
    AutoModerationRuleDelete = 142,
    AutoModerationBlockMessage = 143,
    AutoModerationFlagToChannel = 144,
    AutoModerationUserCommunicationDisabled = 145,
    CreatorMonetizationRequestCreated = 150,
    CreatorMonetizationTermsAccepted = 151,
    OnboardingPromptCreate = 163,
    OnboardingPromptUpdate = 164,
    OnboardingPromptDelete = 165,
    OnboardingCreate = 166,
    OnboardingUpdate = 167,
    HomeSettingsCreate = 190,
    HomeSettingsUpdate = 191,
}

/// Discord docs: https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-optional-audit-entry-info
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct OptionalAuditEntryInfo {
    pub application_id: ApplicationId,
    pub auto_moderation_rule_name: String,
    pub auto_moderation_rule_trigger_type: String,
    pub channel_id: ChannelId,
    pub count: u64,
    pub delete_member_days: u64,
    pub id: String,
    pub members_removed: u64,
    pub message_id: MessageId,
    pub role_name: Option<String>,
    pub r#type: OverwrittenType,
    pub integration_type: IntegrationType,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum OverwrittenType {
    Role = 0,
    Member = 1,
}

/// Discord docs: https://discord.com/developers/docs/resources/audit-log#audit-log-change-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuditLogChange {
    #[cfg(not(feature = "serde"))]
    pub new_value: Option<Rc<dyn Any>>,
    #[cfg(feature = "serde")]
    new_value: Option<Value>,

    #[cfg(not(feature = "serde"))]
    pub old_value: Option<Rc<dyn Any>>,
    #[cfg(feature = "serde")]
    old_value: Option<Value>,

    pub key: String,
}
