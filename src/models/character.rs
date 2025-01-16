use crate::leveling::get_base_level_from_bsky_profile;
use crate::models::udts::leveling::Leveling;
use atrium_api::app::bsky::actor::defs::ProfileViewDetailed;
use charybdis::macros::charybdis_model;
use charybdis::types::Text;
use serde::Serialize;

#[derive(Default, Serialize)]
#[charybdis_model(
    table_name = characters,
    partition_keys = [user_did],
    clustering_keys = []
)]
pub struct Character {
    pub user_did: Text,           // profile_did
    pub name: Text,               // handle
    pub leveling_state: Leveling, // udt leveling state
}

impl From<ProfileViewDetailed> for Character {
    fn from(response: ProfileViewDetailed) -> Self {
        let level_response = get_base_level_from_bsky_profile(&response);

        Self {
            user_did: response.did.clone().to_string(),
            name: response.handle.clone().to_string(),
            leveling_state: Leveling::from(level_response),
        }
    }
}
