pub mod activity;
pub mod auto_moderation;
pub mod channel;
pub mod entitlement;
pub mod guild;
pub mod integration;
pub mod invite;
pub mod message;
pub mod presence;
pub mod stage_instance;
pub mod thread;
pub mod user;
pub mod voice;
pub mod webhooks;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::api::objects::{application::ApplicationFlags, guild::UnavailableGuild, user::User};

use self::{guild::GuildCreate, presence::PresenceUpdate, voice::VoiceState};

// https://discord.com/developers/docs/topics/gateway-events#receive-events
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "t", content = "d"))]
pub enum Event {
    Ready(Ready),
    Resumed(Unimplemented),
    ApplicationCommandPermissionsUpdate(Unimplemented),
    AutoModerationRuleCreate(Unimplemented),
    AutoModerationRuleUpdate(Unimplemented),
    AutoModerationRuleDelete(Unimplemented),
    AutoModerationActionExecution(Unimplemented),
    ChannelCreate(Unimplemented),
    ChannelUpdate(Unimplemented),
    ChannelDelete(Unimplemented),
    ChannelPinsUpdate(Unimplemented),
    ThreadCreate(Unimplemented),
    ThreadUpdate(Unimplemented),
    ThreadDelete(Unimplemented),
    ThreadListSync(Unimplemented),
    ThreadMemberUpdate(Unimplemented),
    ThreadMembersUpdate(Unimplemented),
    EntitlementCreate(Unimplemented),
    EntitlementUpdate(Unimplemented),
    EntitlementDelete(Unimplemented),
    GuildCreate(GuildCreate),
    GuildUpdate(Unimplemented),
    GuildDelete(Unimplemented),
    GuildAuditLogEntryCreate(Unimplemented),
    GuildBanAdd(Unimplemented),
    GuildBanRemove(Unimplemented),
    GuildEmojisUpdate(Unimplemented),
    GuildStickersUpdate(Unimplemented),
    GuildIntegrationsUpdate(Unimplemented),
    GuildMemberAdd(Unimplemented),
    GuildMemberRemove(Unimplemented),
    GuildMemberUpdate(Unimplemented),
    GuildMembersChunk(Unimplemented),
    GuildRoleCreate(Unimplemented),
    GuildRoleUpdate(Unimplemented),
    GuildRoleDelete(Unimplemented),
    GuildScheduledEventCreate(Unimplemented),
    GuildScheduledEventUpdate(Unimplemented),
    GuildScheduledEventDelete(Unimplemented),
    GuildScheduledEventUserAdd(Unimplemented),
    GuildScheduledEventUserRemove(Unimplemented),
    IntegrationCreate(Unimplemented),
    IntegrationUpdate(Unimplemented),
    IntegrationDelete(Unimplemented),
    InteractionCreate(Unimplemented),
    InviteCreate(Unimplemented),
    InviteDelete(Unimplemented),
    MessageCreate(Unimplemented),
    MessageUpdate(Unimplemented),
    MessageDelete(Unimplemented),
    MessageDeleteBulk(Unimplemented),
    MessageReactionAdd(Unimplemented),
    MessageReactionRemove(Unimplemented),
    MessageReactionRemoveAll(Unimplemented),
    MessageReactionRemoveEmoji(Unimplemented),
    PresenceUpdate(PresenceUpdate),
    StageInstanceCreate(Unimplemented),
    StageInstanceUpdate(Unimplemented),
    StageInstanceDelete(Unimplemented),
    TypingStart(Unimplemented),
    UserUpdate(Unimplemented),
    VoiceStateUpdate(VoiceState),
    VoiceServerUpdate(Unimplemented),
    WebhooksUpdate(Unimplemented),
    MessagePollVoteAdd(Unimplemented),
    MessagePollVoteRemove(Unimplemented),
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Unimplemented {}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Ready {
    pub v: usize,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<(usize, usize)>,
    pub application: UnavailableApplication,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UnavailableApplication {
    pub id: String,
    pub flags: ApplicationFlags,
}
