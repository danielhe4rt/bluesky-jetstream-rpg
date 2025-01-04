use crate::leveling::LevelResponse;
use crate::models::{Character, CharacterExperience};
use charybdis::operations::{Find, Insert};
use charybdis::types::Counter;
use scylla::CachingSession;
use std::sync::Arc;

pub struct CharacterRepository {
    pub session: Arc<CachingSession>,
}

impl CharacterRepository {

    pub fn new(connection: Arc<CachingSession>) -> Self {
        Self {
            session: connection,
        }
    }
    pub async fn find_by_partition_key(&self, user_id: String) -> Character {
        let character = Character {
            user_id,
            ..Default::default()
        };

        character.find_by_partition_key()
            .execute(&self.session)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
            .pop()
            .unwrap_or(character)
    }

    pub async fn find_character_experience_by_partition_key(&self, user_id: String) -> CharacterExperience {
        let character_experience = CharacterExperience {
            user_id,
            current_experience: Counter(0),
        };

        character_experience.find_by_partition_key()
            .execute(&self.session)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
            .pop()
            .unwrap_or(character_experience)
    }

    pub async fn increment_character_experience(&self, character_experience: CharacterExperience, response: LevelResponse) {
        character_experience.increment_current_experience(response.experience as i64)
            .execute(&self.session)
            .await
            .expect("Failed to increment experience");
    }

    pub async fn update_character(&self, character: &mut Character, response: LevelResponse) {
        character.level = response.level;
        character.current_experience = response.experience;
        character.experience_to_next_level = response.experience_to_next_level;
        character.insert().execute(&self.session).await.expect("Failed to update character");
    }
}