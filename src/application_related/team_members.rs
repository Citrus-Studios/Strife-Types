use serde::{Deserialize, Serialize};

use crate::{Snowflake, user::User}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMember {
    membership_state: i32,
    premissions: Vec<String>,
    team_id: Snowflake,
    user: User,
}