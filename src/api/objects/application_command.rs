#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    application::{ApplicationId, IntegrationType},
    channel::ChannelType,
    guild::GuildId,
    interactions::InteractionContextType,
    permissions::Permissions,
    Translation,
};

/// Discord docs: https://discord.com/developers/docs/interactions/application-commands#application-command-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationCommand {
    pub id: ApplicationCommandId,
    #[cfg_attr(feature = "serde", serde(default))]
    pub r#type: Option<ApplicationCommandType>,
    pub application_id: ApplicationId,
    #[cfg_attr(feature = "serde", serde(default))]
    pub guild_id: Option<GuildId>,
    pub name: Name,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_localizations: Option<Translation<Name>>,
    pub description: Description,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description_localizations: Option<Translation<Description>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub options: Vec<ApplicationCommandOption>,
    pub default_member_permissions: Option<Permissions>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub dm_permission: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub default_permission: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub nsfw: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub integration_types: Option<IntegrationType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub contexts: Vec<InteractionContextType>,
    pub version: Version,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationCommandId(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Name(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Description(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Version(pub String);

/// Discord docs: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

/// Discord docs: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationCommandOption {
    pub r#type: ApplicationCommandOptionType,
    pub name: OptionName,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_localizations: Option<Translation<OptionName>>,
    pub description: OptionDescription,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description_localizations: Option<Translation<OptionDescription>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub required: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub choices: Vec<ApplicationCommandOptionChoice>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub options: Vec<ApplicationCommandOptionChoice>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub channel_types: Vec<ChannelType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub min_value: Option<Value>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub max_value: Option<Value>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub min_length: Option<u16>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub max_length: Option<u16>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub autocomplete: Option<bool>,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Value {
    Integer(i64),
    Number(f64),
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct OptionName(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct OptionDescription(pub String);

/// Discord docs: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationCommandOptionChoice {
    pub name: ChoiceName,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_localizations: Option<Translation<ChoiceName>>,
    pub value: ChoiceValue,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ChoiceName(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ChoiceValue {
    String(String),
    Integer(i64),
    Double(f64),
}
