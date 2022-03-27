use serde::{Deserialize, Serialize};

use crate::Snowflake;

use super::team_members::TeamMember; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    icon: Option<String>,
    id: Snowflake,
    members: Vec<TeamMember>,
    name: String,
    owner_user_id: Snowflake
}