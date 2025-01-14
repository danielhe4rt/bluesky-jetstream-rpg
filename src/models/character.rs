use crate::leveling::get_base_level_from_bsky_profile;
use atrium_api::app::bsky::actor::defs::ProfileViewDetailed;
use charybdis::macros::charybdis_model;
use charybdis::types::{Int, Text};
use serde::Serialize;

#[derive(Default, Serialize)]
#[charybdis_model(
    table_name = characters,
    partition_keys = [user_id],
    clustering_keys = []
)]
pub struct Character {
    pub user_id: Text,
    pub name: Text,
    pub current_experience: Int,
    pub experience_to_next_level: Int,
    pub level: Int,
}

impl From<ProfileViewDetailed> for Character {
    fn from(response: ProfileViewDetailed) -> Self {
        let level_response = get_base_level_from_bsky_profile(&response);

        Self {
            user_id: response.did.clone().to_string(),
            name: response.handle.clone().to_string(),
            current_experience: level_response.experience,
            experience_to_next_level: level_response.experience_to_next_level,
            level: level_response.level,
        }
    }
}
