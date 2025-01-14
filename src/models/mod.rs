pub mod character;
pub mod events;

use charybdis::macros::{charybdis_model, charybdis_view_model};
use charybdis::types::{Counter, Frozen, Int, Map, Text, Timestamp};
use crate::models::character::Leveling;
// SELECT * FROM user_events WHERE event_id = 1;



#[charybdis_model(
    table_name = characters_experience,
    partition_keys = [user_id],
    clustering_keys = []
)]
pub struct CharacterExperience {
    pub user_id: Text,
    pub current_experience: Counter,
}

impl CharacterExperience {
    pub fn get_experience(&self) -> i32 {
        let exp = self.current_experience.0 as i32;

        if exp < 0 {
            0
        } else {
            exp
        }
    }
}
