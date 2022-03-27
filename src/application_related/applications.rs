use serde::{Deserialize, Serialize};

use crate::{Snowflake, user::User};

use super::teams::Team; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: bool,
    bot_require_code_grant: bool,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: Option<User>,
    summary: String,
    verify_key: String,
    team: Option<Team>,
    guild_id: Snowflake,
    primary_sku_id: Snowflake,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: Option<i32>
}