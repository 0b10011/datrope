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
    application_commands: Vec<ApplicationCommand>,
    audit_log_entries: Vec<AuditLogEntry>,
    auto_moderation_rules: Vec<AutoModerationRule>,
    guild_scheduled_events: Vec<GuildScheduledEvent>,
    integrations: Vec<UnavailableIntegration>,
    threads: Vec<Channel>,
    users: Vec<User>,
    webhooks: Vec<Webhook>,
}

/// Discord docs: https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuditLogEntry {
    target_id: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    changes: Vec<AuditLogChange>,
    #[cfg_attr(feature = "serde", serde(default))]
    user_id: Option<UserId>,
    id: AuditLogEntryId,
    action_type: AuditLogEvent,
    #[cfg_attr(feature = "serde", serde(default))]
    options: Option<OptionalAuditEntryInfo>,
    #[cfg_attr(feature = "serde", serde(default))]
    reason: String,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuditLogEntryId(String);

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
    application_id: ApplicationId,
    auto_moderation_rule_name: String,
    auto_moderation_rule_trigger_type: String,
    channel_id: ChannelId,
    count: u64,
    delete_member_days: u64,
    id: String,
    members_removed: u64,
    message_id: MessageId,
    role_name: Option<String>,
    r#type: OverwrittenType,
    integration_type: IntegrationType,
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
    new_value: Option<Rc<dyn Any>>,
    #[cfg(feature = "serde")]
    new_value: Option<Value>,

    #[cfg(not(feature = "serde"))]
    old_value: Option<Rc<dyn Any>>,
    #[cfg(feature = "serde")]
    old_value: Option<Value>,

    key: String,
}
