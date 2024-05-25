/// Discord docs: https://discord.com/developers/docs/topics/teams

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    user::{UnavailableUser, UserId},
    ImageHash,
};

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TeamId(pub String);

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Team {
    pub icon: Option<ImageHash>,
    pub id: TeamId,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: UserId,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TeamMember {
    pub membership_state: MembershipState,
    pub team_id: TeamId,
    pub user: UnavailableUser,
    pub role: Role,
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
