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
use crate::{
    api::objects::{application::ApplicationFlags, guild::UnavailableGuild, user::User},
    flags,
};
#[cfg(feature = "serde")]
use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize};
#[cfg(feature = "serde")]
use serde_json::{json, Value};
#[cfg(feature = "serde")]
use serde_repr::{Deserialize_repr, Serialize_repr};
#[cfg(not(feature = "serde"))]
use std::{any::Any, rc::Rc};

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
pub enum EventPayload {
    Dispatch(SequenceNumber, Event),
    Heartbeat(Option<SequenceNumber>),
    Identify(Identify),
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello(Hello),
    HeartbeatAck,
}

impl EventPayload {
    pub const FIELD_OPCODE: &'static str = "op";
    pub const FIELD_DATA: &'static str = "d";
    pub const FIELD_SEQUENCE_NUMBER: &'static str = "s";
    pub const FIELD_EVENT_NAME: &'static str = "t";

    pub fn opcode(&self) -> Opcode {
        match self {
            EventPayload::Dispatch(_, _) => Opcode::Dispatch,
            EventPayload::Heartbeat(_) => Opcode::Heartbeat,
            EventPayload::Identify(_) => Opcode::Identify,
            EventPayload::PresenceUpdate => Opcode::PresenceUpdate,
            EventPayload::VoiceStateUpdate => Opcode::VoiceStateUpdate,
            EventPayload::Resume => Opcode::Resume,
            EventPayload::Reconnect => Opcode::Reconnect,
            EventPayload::RequestGuildMembers => Opcode::RequestGuildMembers,
            EventPayload::InvalidSession => Opcode::InvalidSession,
            EventPayload::Hello(_) => Opcode::Hello,
            EventPayload::HeartbeatAck => Opcode::HeartbeatAck,
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for EventPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("EventPayload", 2)?;
        state.serialize_field(Self::FIELD_OPCODE, &self.opcode())?;

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
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
        struct RawEventPayload {
            #[serde(rename = "op")]
            opcode: Opcode,
            #[serde(rename = "s", default)]
            sequence_number: Option<SequenceNumber>,
            #[serde(rename = "d", default)]
            data: Value,
            #[serde(rename = "t", default)]
            event_name: Value,
        }

        let raw_event = RawEventPayload::deserialize(deserializer)?;
        match raw_event.opcode {
            Opcode::Dispatch => Ok(EventPayload::Dispatch(
                raw_event.sequence_number.ok_or(de::Error::custom(
                    "No sequence number provided for a `Dispatch` event",
                ))?,
                serde_json::from_value(json!({
                    Self::FIELD_DATA: raw_event.data,
                    Self::FIELD_EVENT_NAME: raw_event.event_name
                }))
                .map_err(de::Error::custom)?,
            )),
            Opcode::Heartbeat => Ok(EventPayload::Heartbeat(raw_event.sequence_number)),
            Opcode::Identify => Ok(EventPayload::Identify(
                serde_json::from_value(raw_event.data).map_err(de::Error::custom)?,
            )),
            Opcode::PresenceUpdate => Ok(EventPayload::PresenceUpdate),
            Opcode::VoiceStateUpdate => Ok(EventPayload::VoiceStateUpdate),
            Opcode::Resume => Ok(EventPayload::Resume),
            Opcode::Reconnect => Ok(EventPayload::Reconnect),
            Opcode::RequestGuildMembers => Ok(EventPayload::RequestGuildMembers),
            Opcode::InvalidSession => Ok(EventPayload::InvalidSession),
            Opcode::Hello => Ok(EventPayload::Hello(
                serde_json::from_value(raw_event.data).map_err(de::Error::custom)?,
            )),
            Opcode::HeartbeatAck => Ok(EventPayload::HeartbeatAck),
        }
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
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
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
    pub intents: GatewayIntents,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

flags!(gateway_intents: i32 {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildModeration = 1 << 2,
    GuildEmojisAndStickers = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhooks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessages = 1 << 9,
    GuildMessageReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectMessageReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
    MessageContent = 1 << 15,
    GuildScheduledEvents = 1 << 16,
    AutoModerationConfiguration = 1 << 20,
    AutoModerationExecution = 1 << 21,
    GuildMessagePolls = 1 << 24,
    DirectMessagePolls = 1 << 25,
});
pub use gateway_intents::Flags as GatewayIntents;

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Hello {
    pub heartbeat_interval: usize,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Unimplemented(
    #[cfg(not(feature = "serde"))] pub Rc<dyn Any>,
    #[cfg(feature = "serde")] pub Value,
);

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

#[test]
fn test() {
    use std::assert_matches::assert_matches;
    let json = r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-us-east1-c-kz49\",{\"micros\":0.0}]"]}}"#;
    let deserializer = &mut serde_json::Deserializer::from_str(json);
    let actual_message: EventPayload =
        serde_path_to_error::deserialize(deserializer).expect("failed to deserialize");
    assert_matches!(actual_message, EventPayload::Hello(_));
}

/// JSON taken from https://discord.com/developers/docs/topics/gateway#heartbeat-interval-example-heartbeat-ack
#[test]
fn test_example_heartbeat_ack() {
    use std::assert_matches::assert_matches;
    let json = r#"{"op": 11}"#;
    let deserializer = &mut serde_json::Deserializer::from_str(json);
    let event: EventPayload =
        serde_path_to_error::deserialize(deserializer).expect("failed to deserialize");
    assert_matches!(event, EventPayload::HeartbeatAck);
}

#[test]
fn test_dispatch_without_sequence_number() {
    let json = r#"{"op": 0}"#;
    let deserializer = &mut serde_json::Deserializer::from_str(json);
    let error = serde_path_to_error::deserialize::<_, EventPayload>(deserializer)
        .expect_err("Deserializing should fail because of the missing sequence number");

    assert_eq!(".", error.path().to_string());
    assert_eq!(
        "No sequence number provided for a `Dispatch` event",
        error.to_string()
    );
}

/// Taken from a gateway response on 2024-06-04 with IDs/hashes randomized
#[test]
fn test_voice_state_update() {
    let json = r#"{
        "t": "VOICE_STATE_UPDATE",
        "s": 3,
        "op": 0,
        "d": {
            "member": {
                "user": {
                    "username": "foobar",
                    "public_flags": 0,
                    "id": "123456789012345678",
                    "global_name": null,
                    "display_name": null,
                    "discriminator": "0",
                    "clan": null,
                    "bot": false,
                    "avatar_decoration_data": {
                        "sku_id": "2345678901234567890",
                        "asset": "a_123456789012345678901234567890ab"
                    },
                    "avatar": "ab123456789012345678901234567890"
                },
                "roles": [],
                "premium_since": null,
                "pending": false,
                "nick": null,
                "mute": false,
                "joined_at": "2024-04-20T19:19:19.190000+00:00",
                "flags": 0,
                "deaf": false,
                "communication_disabled_until": null,
                "avatar": null
            },
            "user_id": "123456789012345678",
            "suppress": false,
            "session_id": "098765432109876543210987654321ab",
            "self_video": false,
            "self_mute": false,
            "self_deaf": false,
            "request_to_speak_timestamp": null,
            "mute": false,
            "guild_id": "2345678901234567890",
            "deaf": false,
            "channel_id": "3456789012345678901"
        }
    }"#;
    let deserializer = &mut serde_json::Deserializer::from_str(json);
    let _value = serde_path_to_error::deserialize::<_, EventPayload>(deserializer)
        .expect("Deserializing should succeed");
}

#[test]
fn test_path() {
    let json = r#"{
        "t": "VOICE_STATE_UPDATE",
        "s": 3,
        "op": 0,
        "d": {
            "member": null,
            "user_id": "123456789012345678",
            "suppress": false,
            "session_id": "098765432109876543210987654321ab",
            "self_video": false,
            "self_mute": false,
            "self_deaf": false,
            "request_to_speak_timestamp": "definitely not a date",
            "mute": false,
            "guild_id": "2345678901234567890",
            "deaf": false,
            "channel_id": "3456789012345678901"
        }
    }"#;
    let deserializer = &mut serde_json::Deserializer::from_str(json);
    let error = serde_path_to_error::deserialize::<_, EventPayload>(deserializer).expect_err(
        "Deserializing should fail because of the bad `request_to_speak_timestamp` value",
    );

    // The actual error message doesn't matter that much,
    // it's just used to confirm the error is with the date field we're expecting.
    assert_eq!(
        "the 'year' component could not be parsed",
        error.to_string()
    );
    assert_eq!("d.request_to_speak_timestamp", error.path().to_string());
}
