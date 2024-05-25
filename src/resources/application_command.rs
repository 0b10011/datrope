#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    application::{ApplicationId, IntegrationType, Permissions},
    channel::ChannelType,
    guild::GuildId,
    interactions::InteractionContextType,
    Translation,
};

/// Discord docs: https://discord.com/developers/docs/interactions/application-commands#application-command-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationCommand {
    id: ApplicationCommandId,
    #[cfg_attr(feature = "serde", serde(default))]
    r#type: Option<ApplicationCommandType>,
    application_id: ApplicationId,
    #[cfg_attr(feature = "serde", serde(default))]
    guild_id: Option<GuildId>,
    name: Name,
    #[cfg_attr(feature = "serde", serde(default))]
    name_localizations: Option<Translation<Name>>,
    description: Description,
    #[cfg_attr(feature = "serde", serde(default))]
    description_localizations: Option<Translation<Description>>,
    #[cfg_attr(feature = "serde", serde(default))]
    options: Vec<ApplicationCommandOption>,
    default_member_permissions: Option<Permissions>,
    #[cfg_attr(feature = "serde", serde(default))]
    dm_permission: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    default_permission: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    nsfw: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    integration_types: Option<IntegrationType>,
    #[cfg_attr(feature = "serde", serde(default))]
    contexts: Vec<InteractionContextType>,
    version: Version,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationCommandId(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Name(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Description(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Version(String);

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
    r#type: ApplicationCommandOptionType,
    name: OptionName,
    #[cfg_attr(feature = "serde", serde(default))]
    name_localizations: Option<Translation<OptionName>>,
    description: OptionDescription,
    #[cfg_attr(feature = "serde", serde(default))]
    description_localizations: Option<Translation<OptionDescription>>,
    #[cfg_attr(feature = "serde", serde(default))]
    required: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    choices: Vec<ApplicationCommandOptionChoice>,
    #[cfg_attr(feature = "serde", serde(default))]
    options: Vec<ApplicationCommandOptionChoice>,
    #[cfg_attr(feature = "serde", serde(default))]
    channel_types: Vec<ChannelType>,
    #[cfg_attr(feature = "serde", serde(default))]
    min_value: Option<Value>,
    #[cfg_attr(feature = "serde", serde(default))]
    max_value: Option<Value>,
    #[cfg_attr(feature = "serde", serde(default))]
    min_length: Option<u16>,
    #[cfg_attr(feature = "serde", serde(default))]
    max_length: Option<u16>,
    #[cfg_attr(feature = "serde", serde(default))]
    autocomplete: Option<bool>,
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
pub struct OptionName(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct OptionDescription(String);

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
    name: ChoiceName,
    #[cfg_attr(feature = "serde", serde(default))]
    name_localizations: Option<Translation<ChoiceName>>,
    value: ChoiceValue,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ChoiceName(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ChoiceValue {
    String(String),
    Integer(i64),
    Double(f64),
}
