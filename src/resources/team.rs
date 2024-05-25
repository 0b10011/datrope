/// https://discord.com/developers/docs/topics/teams

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    user::{UnavailableUser, UserId},
    ImageHash,
};

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TeamId(String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Team {
    icon: Option<ImageHash>,
    id: TeamId,
    members: Vec<TeamMember>,
    name: String,
    owner_user_id: UserId,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TeamMember {
    membership_state: MembershipState,
    team_id: TeamId,
    user: UnavailableUser,
    role: Role,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Role {
    Admin,
    Developer,
    ReadOnly,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum MembershipState {
    Invited,
    Accepted,
}
