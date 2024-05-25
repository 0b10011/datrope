#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod application;
pub mod application_role_connection_metadata;
pub mod audit_log;
pub mod auto_moderation;
pub mod channel;
pub mod guild;
pub mod guild_scheduled_event;
pub mod guild_template;
pub mod invite;
pub mod poll;
pub mod stage_instance;
pub mod sticker;
pub mod team;
pub mod user;
pub mod voice;

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ImageHash(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
struct Translation<T> {
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    indonesian: T,
    #[cfg_attr(feature = "serde", serde(rename = "da"))]
    danish: T,
    #[cfg_attr(feature = "serde", serde(rename = "de"))]
    german: T,
    #[cfg_attr(feature = "serde", serde(rename = "en-GB"))]
    english_uk: T,
    #[cfg_attr(feature = "serde", serde(rename = "en-US"))]
    english_us: T,
    #[cfg_attr(feature = "serde", serde(rename = "es-ES"))]
    spanish: T,
    #[cfg_attr(feature = "serde", serde(rename = "es-419"))]
    spanish_latam: T,
    #[cfg_attr(feature = "serde", serde(rename = "fr"))]
    french: T,
    #[cfg_attr(feature = "serde", serde(rename = "hr"))]
    croatian: T,
    #[cfg_attr(feature = "serde", serde(rename = "it"))]
    italian: T,
    #[cfg_attr(feature = "serde", serde(rename = "lt"))]
    lithuanian: T,
    #[cfg_attr(feature = "serde", serde(rename = "hu"))]
    hungarian: T,
    #[cfg_attr(feature = "serde", serde(rename = "nl"))]
    dutch: T,
    #[cfg_attr(feature = "serde", serde(rename = "no"))]
    norwegian: T,
    #[cfg_attr(feature = "serde", serde(rename = "pl"))]
    polish: T,
    #[cfg_attr(feature = "serde", serde(rename = "pt-BR"))]
    portuguese_brazilian: T,
    #[cfg_attr(feature = "serde", serde(rename = "ro"))]
    romanian_romania: T,
    #[cfg_attr(feature = "serde", serde(rename = "fi"))]
    finnish: T,
    #[cfg_attr(feature = "serde", serde(rename = "sv-SE"))]
    swedish: T,
    #[cfg_attr(feature = "serde", serde(rename = "vi"))]
    vietnamese: T,
    #[cfg_attr(feature = "serde", serde(rename = "tr"))]
    turkish: T,
    #[cfg_attr(feature = "serde", serde(rename = "cs"))]
    czech: T,
    #[cfg_attr(feature = "serde", serde(rename = "el"))]
    greek: T,
    #[cfg_attr(feature = "serde", serde(rename = "bg"))]
    bulgarian: T,
    #[cfg_attr(feature = "serde", serde(rename = "ru"))]
    russian: T,
    #[cfg_attr(feature = "serde", serde(rename = "uk"))]
    ukrainian: T,
    #[cfg_attr(feature = "serde", serde(rename = "hi"))]
    hindi: T,
    #[cfg_attr(feature = "serde", serde(rename = "th"))]
    thai: T,
    #[cfg_attr(feature = "serde", serde(rename = "zh-CN"))]
    chinese_china: T,
    #[cfg_attr(feature = "serde", serde(rename = "ja"))]
    japanese: T,
    #[cfg_attr(feature = "serde", serde(rename = "zh-TW"))]
    chinese_taiwan: T,
    #[cfg_attr(feature = "serde", serde(rename = "ko"))]
    korean: T,
}
