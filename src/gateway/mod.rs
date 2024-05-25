#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod events;

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
