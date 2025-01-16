use crate::leveling::LevelResponse;
use charybdis::macros::charybdis_udt_model;
use charybdis::types::{Float, Int};
use serde::Serialize;

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
