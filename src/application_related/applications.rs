use serde::{Deserialize, Serialize};

use crate::{user::User, Snowflake};

use super::teams::Team;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    id: String,
    name: Option<String>,
    icon: Option<String>,
    description: Option<String>,
    rpc_origins: Option<Vec<String>>,
    bot_public: Option<bool>,
    bot_require_code_grant: Option<bool>,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: Option<User>,
    summary: Option<String>,
    verify_key: Option<String>,
    team: Option<Team>,
    guild_id: Option<Snowflake>,
    primary_sku_id: Option<Snowflake>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: Option<i32>,
}
