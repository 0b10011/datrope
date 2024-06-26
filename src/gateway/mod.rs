#[cfg(feature = "gateway")]
use std::sync::mpsc;

#[cfg(feature = "gateway")]
use futures_util::{SinkExt, StreamExt};
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
        ConnectionProperties, EventPayload, GatewayIntents, Identify, SequenceNumber,
    },
};

pub mod events;

#[cfg(feature = "gateway")]
pub async fn connect(token: String, intents: GatewayIntents) {
    use events::Hello;
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
            let deserializer = &mut serde_json::Deserializer::from_str(&message);
            let event_payload: Result<EventPayload, serde_path_to_error::Error<serde_json::Error>> =
                serde_path_to_error::deserialize(deserializer);
            match event_payload {
                Ok(event_payload) => incoming_sender
                    .send(event_payload)
                    .expect("failed to forward incoming message"),
                Err(err) => {
                    let value: serde_json::Result<Value> = serde_json::from_str(&message);
                    let newline = "\n";
                    panic!(
                        r#"error "{}" in path `{}` while parsing: {:#?}{newline}{message}"#,
                        err,
                        err.path(),
                        value
                    )
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
        let mut intents_wrapped = Some(intents);
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
                                intents: intents_wrapped
                                    .take()
                                    .expect("This should only ever be unwrapped once"),
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
