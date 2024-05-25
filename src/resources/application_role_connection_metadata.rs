#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Translation;

/// Discord docs: https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationRoleConnectionMetadata {
    pub r#type: ApplicationRoleConnectionMetadataType,
    pub key: String,
    pub name: Name,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_localizations: Option<Translation<Name>>,
    pub description: Description,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description_localizations: Option<Translation<Description>>,
}

/// Discord docs: https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object-application-role-connection-metadata-type
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ApplicationRoleConnectionMetadataType {
    IntegerLessThanOrEqual = 1,
    IntegerGreaterThanOrEqual = 2,
    IntegerEqual = 3,
    IntegerNotEqual = 4,
    DatetimeLessThanOrEqual = 5,
    DatetimeGreaterThanOrEqual = 6,
    BooleanEqual = 7,
    BooleanNotEqual = 8,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Name(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Description(pub String);
