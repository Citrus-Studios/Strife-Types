use serde::{Deserialize, Serialize};

use crate::application_related::applications::Application;

use super::{user::User, guild_related::guild::Guild}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Ready {
    v: i32,
    user: User,
    guilds: Vec<Guild>,
    session_id: String,
    shard: Option<Vec<i32>>,
    application: Vec<Application>,
}