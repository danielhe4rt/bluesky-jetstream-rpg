use crate::leveling::{get_base_level_from_bsky_profile, LevelResponse};
use atrium_api::app::bsky::actor::defs::ProfileViewDetailed;
use charybdis::macros::{charybdis_model, charybdis_udt_model};
use charybdis::types::{Float, Int, Text};
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
    pub leveling: Leveling,
}

impl From<ProfileViewDetailed> for Character {
    fn from(response: ProfileViewDetailed) -> Self {
        let level_response = get_base_level_from_bsky_profile(&response);

        Self {
            user_id: response.did.clone().to_string(),
            name: response.handle.clone().to_string(),
            leveling: Leveling::from(level_response),
        }
    }
}

#[derive(Default, Serialize)]
#[charybdis_udt_model(type_name = leveling)]
pub struct Leveling {
    pub level: Int,
    pub experience: Int,
    pub experience_to_next_level: Int,
    pub levels_gained: Int,
    pub progress_percentage: Float,
}

impl From<LevelResponse> for Leveling {
    fn from(response: LevelResponse) -> Self {
        Self {
            level: response.level.into(),
            experience: response.experience.into(),
            experience_to_next_level: response.experience_to_next_level.into(),
            levels_gained: response._levels_gained.into(),
            progress_percentage: (response._progress_percentage * 100.0).round(),
        }
    }
}