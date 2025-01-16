use charybdis::macros::charybdis_model;
use charybdis::types::{Counter, Text};

#[charybdis_model(
    table_name = characters_experience,
    partition_keys = [user_did],
    clustering_keys = []
)]
pub struct CharacterExperience {
    pub user_did: Text,
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
