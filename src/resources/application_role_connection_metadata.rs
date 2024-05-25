#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Translation;

/// Discord docs: https://discord.com/developers/docs/resources/application-role-connection-metadata#application-role-connection-metadata-object
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ApplicationRoleConnectionMetadata {
    r#type: ApplicationRoleConnectionMetadataType,
    key: String,
    name: Name,
    #[cfg_attr(feature = "serde", serde(default))]
    name_localizations: Option<Translation<Name>>,
    description: Description,
    #[cfg_attr(feature = "serde", serde(default))]
    description_localizations: Option<Translation<Description>>,
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
pub struct Name(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Description(String);
