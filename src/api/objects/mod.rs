#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod application;
pub mod application_command;
pub mod application_role_connection_metadata;
pub mod audit_log;
pub mod auto_moderation;
pub mod channel;
pub mod emoji;
pub mod guild;
pub mod guild_scheduled_event;
pub mod guild_template;
pub mod interactions;
pub mod invite;
pub mod permissions;
pub mod poll;
pub mod stage_instance;
pub mod sticker;
pub mod team;
pub mod user;
pub mod voice;
pub mod webhook;

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ImageHash(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Translation<T> {
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    pub indonesian: T,
    #[cfg_attr(feature = "serde", serde(rename = "da"))]
    pub danish: T,
    #[cfg_attr(feature = "serde", serde(rename = "de"))]
    pub german: T,
    #[cfg_attr(feature = "serde", serde(rename = "en-GB"))]
    pub english_uk: T,
    #[cfg_attr(feature = "serde", serde(rename = "en-US"))]
    pub english_us: T,
    #[cfg_attr(feature = "serde", serde(rename = "es-ES"))]
    pub spanish: T,
    #[cfg_attr(feature = "serde", serde(rename = "es-419"))]
    pub spanish_latam: T,
    #[cfg_attr(feature = "serde", serde(rename = "fr"))]
    pub french: T,
    #[cfg_attr(feature = "serde", serde(rename = "hr"))]
    pub croatian: T,
    #[cfg_attr(feature = "serde", serde(rename = "it"))]
    pub italian: T,
    #[cfg_attr(feature = "serde", serde(rename = "lt"))]
    pub lithuanian: T,
    #[cfg_attr(feature = "serde", serde(rename = "hu"))]
    pub hungarian: T,
    #[cfg_attr(feature = "serde", serde(rename = "nl"))]
    pub dutch: T,
    #[cfg_attr(feature = "serde", serde(rename = "no"))]
    pub norwegian: T,
    #[cfg_attr(feature = "serde", serde(rename = "pl"))]
    pub polish: T,
    #[cfg_attr(feature = "serde", serde(rename = "pt-BR"))]
    pub portuguese_brazilian: T,
    #[cfg_attr(feature = "serde", serde(rename = "ro"))]
    pub romanian_romania: T,
    #[cfg_attr(feature = "serde", serde(rename = "fi"))]
    pub finnish: T,
    #[cfg_attr(feature = "serde", serde(rename = "sv-SE"))]
    pub swedish: T,
    #[cfg_attr(feature = "serde", serde(rename = "vi"))]
    pub vietnamese: T,
    #[cfg_attr(feature = "serde", serde(rename = "tr"))]
    pub turkish: T,
    #[cfg_attr(feature = "serde", serde(rename = "cs"))]
    pub czech: T,
    #[cfg_attr(feature = "serde", serde(rename = "el"))]
    pub greek: T,
    #[cfg_attr(feature = "serde", serde(rename = "bg"))]
    pub bulgarian: T,
    #[cfg_attr(feature = "serde", serde(rename = "ru"))]
    pub russian: T,
    #[cfg_attr(feature = "serde", serde(rename = "uk"))]
    pub ukrainian: T,
    #[cfg_attr(feature = "serde", serde(rename = "hi"))]
    pub hindi: T,
    #[cfg_attr(feature = "serde", serde(rename = "th"))]
    pub thai: T,
    #[cfg_attr(feature = "serde", serde(rename = "zh-CN"))]
    pub chinese_china: T,
    #[cfg_attr(feature = "serde", serde(rename = "ja"))]
    pub japanese: T,
    #[cfg_attr(feature = "serde", serde(rename = "zh-TW"))]
    pub chinese_taiwan: T,
    #[cfg_attr(feature = "serde", serde(rename = "ko"))]
    pub korean: T,
}
