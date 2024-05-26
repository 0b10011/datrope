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

use self::{guild::GuildCreate, presence::PresenceUpdate, voice::VoiceState};
use super::Hello;
use crate::api::objects::{application::ApplicationFlags, guild::UnavailableGuild, user::User};
#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
#[cfg(feature = "serde")]
use serde_json::Value;
#[cfg(feature = "serde")]
use serde_repr::{Deserialize_repr, Serialize_repr};

// https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "serde", repr(u8))]
pub enum Opcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}

// https://discord.com/developers/docs/topics/gateway-events#payload-structure
// Could use `#[serde(tag = "op", content = "d")]` and `#[serde(rename = "2", skip_deserializing)]`
// if https://github.com/serde-rs/serde/issues/745 was fixed :(
// In the meantime, see `OpCode` for mapping of codes.
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[repr(u8)]
pub enum EventPayload {
    Dispatch(SequenceNumber, Event) = 0,
    Heartbeat(Option<SequenceNumber>) = 1,
    Identify(Identify) = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello(Hello) = 10,
    HeartbeatAck = 11,
}

impl EventPayload {
    pub const FIELD_OPCODE: &'static str = "op";
    pub const FIELD_DATA: &'static str = "d";
    pub const FIELD_SEQUENCE_NUMBER: &'static str = "s";
    pub const FIELD_EVENT_NAME: &'static str = "t";

    pub fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[cfg(feature = "serde")]
impl Serialize for EventPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("EventPayload", 2)?;
        state.serialize_field(Self::FIELD_OPCODE, &self.discriminant())?;

        match self {
            EventPayload::Heartbeat(data) => state.serialize_field(Self::FIELD_DATA, data)?,
            EventPayload::Identify(data) => state.serialize_field(Self::FIELD_DATA, data)?,
            EventPayload::PresenceUpdate => todo!(),
            EventPayload::VoiceStateUpdate => todo!(),
            EventPayload::Resume => todo!(),
            EventPayload::RequestGuildMembers => todo!(),

            EventPayload::Dispatch(_, _)
            | EventPayload::Reconnect
            | EventPayload::InvalidSession
            | EventPayload::Hello(_)
            | EventPayload::HeartbeatAck => panic!("These events should never be sent"),
        }

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EventPayload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map = serde_json::Map::deserialize(deserializer)?;

        let value = map
            .get(Self::FIELD_OPCODE)
            .ok_or_else(|| serde::de::Error::missing_field(Self::FIELD_OPCODE))?;
        Ok(
            match Opcode::deserialize(value).map_err(serde::de::Error::custom)? {
                Opcode::Dispatch => {
                    let sequence_number = SequenceNumber::deserialize(
                        map.get(Self::FIELD_SEQUENCE_NUMBER).ok_or_else(|| {
                            serde::de::Error::missing_field(Self::FIELD_SEQUENCE_NUMBER)
                        })?,
                    )
                    .map_err(serde::de::Error::custom)?;
                    Self::Dispatch(
                        sequence_number,
                        Event::deserialize(Value::Object(map)).map_err(serde::de::Error::custom)?,
                    )
                }
                Opcode::Heartbeat => Self::Heartbeat(None),
                Opcode::InvalidSession => Self::InvalidSession,
                Opcode::Hello => Self::Hello(
                    Hello::deserialize(
                        map.get(Self::FIELD_DATA)
                            .ok_or_else(|| serde::de::Error::missing_field(Self::FIELD_DATA))?,
                    )
                    .map_err(serde::de::Error::custom)?,
                ),
                Opcode::Reconnect => Self::Reconnect,
                Opcode::HeartbeatAck => Self::HeartbeatAck,
                Opcode::Identify => Self::Identify(
                    Identify::deserialize(
                        map.get(Self::FIELD_DATA)
                            .ok_or_else(|| serde::de::Error::missing_field(Self::FIELD_DATA))?,
                    )
                    .map_err(serde::de::Error::custom)?,
                ),
                Opcode::PresenceUpdate => Self::PresenceUpdate,
                Opcode::VoiceStateUpdate => Self::VoiceStateUpdate,
                Opcode::Resume => Self::Resume,
                Opcode::RequestGuildMembers => Self::RequestGuildMembers,
            },
        )
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceNumber(pub u64);

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
pub struct Identify {
    pub token: String,
    pub properties: ConnectionProperties,
    pub compress: Option<bool>,
    pub large_threshold: Option<usize>,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub shard: Option<(usize, usize)>,
    pub presence: PresenceUpdate,
    pub intents: usize,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
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
