#[cfg(feature = "gateway")]
use std::sync::mpsc;

#[cfg(feature = "gateway")]
use futures_util::{SinkExt, StreamExt};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "gateway")]
use tokio::task::JoinSet;
#[cfg(feature = "gateway")]
use tokio_tungstenite::{connect_async, tungstenite::Message};
#[cfg(feature = "gateway")]
use url::Url;

#[cfg(feature = "gateway")]
use crate::{
    api::client::Api,
    gateway::events::{
        presence::{PresenceUpdate, Status},
        ConnectionProperties, EventPayload, Identify, SequenceNumber,
    },
};

pub mod events;

#[cfg(feature = "gateway")]
pub async fn connect(token: String, intents: GatewayIntents) {
    use serde_json::Value;

    let base_url = Url::parse("https://discord.com/api/").expect("Failed to parse base URL");
    let api = Api::new(base_url).expect("Failed to build API");

    let mut gateway = api.gateway().get_gateway().await.unwrap();

    gateway.url.set_query(Some("version=10&encoding=json"));

    let (stream, _response) = connect_async(&gateway.url)
        .await
        .expect("Failed to connect");

    let (mut write, read) = stream.split();

    let (outgoing_sender, outgoing_receiver) = mpsc::channel::<EventPayload>();
    let (incoming_sender, incoming_receiver) = mpsc::channel::<EventPayload>();

    let handle_outgoing = async move {
        while let Ok(message) = outgoing_receiver.recv() {
            let message =
                Message::Text(serde_json::to_string(&message).expect("failed to convert"));
            write.send(message).await.expect("failed to send message");
        }
    };

    let handle_incoming = async move {
        read.for_each(|message| async {
            let Ok(Message::Text(message)) = message else {
                return;
            };
            let event_payload: serde_json::Result<EventPayload> = serde_json::from_str(&message);
            match event_payload {
                Ok(event_payload) => incoming_sender
                    .send(event_payload)
                    .expect("failed to forward incoming message"),
                Err(err) => {
                    let value: serde_json::Result<Value> = serde_json::from_str(&message);
                    panic!(r#"error "{}" while parsing: {:#?}"#, err, value)
                }
            }
        })
        .await;
    };

    let message_handler = async move {
        #[allow(unused_variables)]
        let mut latest_sequence_number: Option<SequenceNumber> = None;
        let mut has_identified = false;
        #[allow(unused_variables)]
        let mut received_heartbeat_ack = false;
        while let Ok(payload) = incoming_receiver.recv() {
            match payload {
                #[allow(unused_assignments)]
                EventPayload::Dispatch(sequence_number, _) => {
                    latest_sequence_number = Some(sequence_number);
                }
                EventPayload::Heartbeat(_) => {
                    outgoing_sender
                        .send(EventPayload::Heartbeat(latest_sequence_number))
                        .expect("Failed to queue heartbeat");
                }
                EventPayload::Identify(_) => todo!(),
                EventPayload::PresenceUpdate => todo!(),
                EventPayload::VoiceStateUpdate => todo!(),
                EventPayload::Resume => todo!(),
                EventPayload::Reconnect => todo!(),
                EventPayload::RequestGuildMembers => todo!(),
                EventPayload::InvalidSession => todo!(),
                #[allow(unused_variables)]
                EventPayload::Hello(Hello { heartbeat_interval }) => {
                    outgoing_sender
                        .send(EventPayload::Heartbeat(None))
                        .expect("Failed to queue heartbeat");
                }
                #[allow(unused_assignments)]
                EventPayload::HeartbeatAck => {
                    received_heartbeat_ack = true;
                    if !has_identified {
                        has_identified = true;
                        outgoing_sender
                            .send(EventPayload::Identify(Identify {
                                token: token.clone(),
                                properties: ConnectionProperties {
                                    os: String::from("0b0"),
                                    browser: String::from("0b1"),
                                    device: String::from("0b10"),
                                },
                                compress: None,
                                large_threshold: None,
                                shard: None,
                                presence: PresenceUpdate {
                                    since: None,
                                    activities: vec![],
                                    status: Status::Online,
                                    afk: Some(false),
                                },
                                intents: intents.bits() as usize,
                            }))
                            .expect("Failed to queue identify");
                    }
                }
            }
        }
    };

    let mut set = JoinSet::new();
    set.spawn(handle_incoming);
    set.spawn(message_handler);
    set.spawn(handle_outgoing);
    while let Some(_response) = set.join_next().await {}
}

bitflags::bitflags! {
    #[cfg_attr(feature = "clone", derive(Clone))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
    pub struct GatewayIntents: u64 {
        const GUILDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_MODERATION = 1 << 2;
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        const MESSAGE_CONTENT = 1 << 15;
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        const GUILD_MESSAGE_POLLS = 1 << 24;
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Hello {
    pub heartbeat_interval: usize,
}
