use crate::events::EventRecord;
use crate::leveling::{calculate_experience, LevelResponse};
use crate::models::{Character, CharacterExperience};
use crate::repositories::DatabaseRepository;
use jetstream_oxide::events::commit::CommitInfo;
use jetstream_oxide::events::EventInfo;
use paris::info;
use std::sync::Arc;

pub async fn delete_event_handler(repository: &Arc<DatabaseRepository>, user_info: EventInfo, commit: CommitInfo) {
    let mut character = repository.character.find_by_partition_key(user_info.did.to_string()).await;
    let character_experience = repository.character.find_character_experience_by_partition_key(user_info.did.to_string()).await;
    let event = repository.event.find_event_by_partition_key(commit.rkey.clone()).await;

    if event.is_none() {
        repository.event.insert_event(
            user_info.did.to_string(),
            commit.rkey.to_string(),
            user_info.time_us,
            LevelResponse {
                ..Default::default()
            },
        ).await;
        info!("Event not found, inserted new event");
        return;
    }

    let event = event.unwrap();
    let experience_lost = event.xp as i32;
    let current_experience = character_experience.current_experience.0 as i32 - experience_lost;
    let current_experience = if current_experience < 0 {
        1
    } else {
        current_experience
    };

    match EventRecord::from_string(commit.collection.to_string().as_str()) {
        EventRecord::AppBskyFeedPost => {
            let new_experience_dto = calculate_experience(
                current_experience,
                0
            );
            persist_character_changes(repository, &mut character, character_experience, new_experience_dto).await;
            info!("[Deleted][Post] User {} lost {} experience", user_info.did.to_string(), experience_lost);
        }
        EventRecord::AppBskyFeedRepost => {
            let lost_experience = 5;
            let new_experience_dto = calculate_experience(
                character_experience.current_experience.0 as i32 - lost_experience,
                0,
            );
            persist_character_changes(repository, &mut character, character_experience, new_experience_dto).await;
            info!("[Deleted][Repost] User {} lost {} experience", user_info.did.to_string(), experience_lost);
        }
        EventRecord::AppBskyFeedLike => {
            let lost_experience = 15;
            let new_experience_dto = calculate_experience(
                character_experience.current_experience.0 as i32 - lost_experience,
                0,
            );
            persist_character_changes(repository, &mut character, character_experience, new_experience_dto).await;
            info!("[Deleted][Like] User {} lost {} experience", user_info.did.to_string(), experience_lost);
        }
    }
}


async fn persist_character_changes(
    repository: &Arc<DatabaseRepository>,
    mut character: &mut Character,
    character_experience: CharacterExperience,
    new_experience_dto: LevelResponse,
) {
    repository.character.increment_character_experience(
        character_experience,
        new_experience_dto.clone(),
    ).await;

    repository.character.update_character(
        &mut character,
        new_experience_dto,
    ).await;

}