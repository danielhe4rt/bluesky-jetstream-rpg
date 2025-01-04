use crate::events::create::create_post::CreatePostEvent;
use crate::events::create::like_post::LikePostEvent;
use crate::leveling::{calculate_experience, LevelResponse};
use crate::models::{Character, CharacterExperience};
use crate::repositories::DatabaseRepository;
use atrium_api::record::KnownRecord;
use atrium_api::record::KnownRecord::AppBskyFeedPost;
use jetstream_oxide::events::commit::CommitData;
use jetstream_oxide::events::EventInfo;
use std::sync::Arc;
use paris::info;

mod create_post;
mod like_post;
mod repost;

pub async fn create_event_handler(repository: &Arc<DatabaseRepository>, user_info: EventInfo, commit: CommitData) {
    let mut character = repository.character.find_by_partition_key(user_info.did.to_string()).await;
    let character_experience = repository.character.find_character_experience_by_partition_key(user_info.did.to_string()).await;
    match commit.record {
        AppBskyFeedPost(record) => {
            let mut post = CreatePostEvent::new();
            post.handle(KnownRecord::AppBskyFeedPost(record));
            let gained_experience = post.calculate_exp();

            let new_experience_dto = calculate_experience(
                character_experience.current_experience.0 as i32,
                gained_experience,
            );

            persist_character_changes(repository, &mut character, character_experience, new_experience_dto).await;

            info!("[Created][Post] User {} gained {} experience", user_info.did.to_string(), gained_experience);
        }
        KnownRecord::AppBskyFeedLike(record) => {
            let mut like = LikePostEvent::new();
            like.handle(KnownRecord::AppBskyFeedLike(record));
            let gained_experience = like.calculate_exp();

            let new_experience_dto = calculate_experience(
                character_experience.current_experience.0 as i32,
                gained_experience,
            );

            persist_character_changes(repository, &mut character, character_experience, new_experience_dto.clone()).await;
            repository.event.insert_event(
                user_info.did.to_string(),
                "like".to_string(),
                user_info.time_us,
                new_experience_dto.clone()
            ).await;

            info!("[Created][Like] User {} gained {} experience", user_info.did.to_string(), gained_experience);
        }
        KnownRecord::AppBskyFeedRepost(record) => {
            let mut repost = repost::RepostEvent::new();
            repost.handle(KnownRecord::AppBskyFeedRepost(record));
            let gained_experience = repost.calculate_exp();

            let new_experience_dto = calculate_experience(
                character_experience.current_experience.0 as i32,
                gained_experience,
            );

            persist_character_changes(repository, &mut character, character_experience, new_experience_dto).await;
            info!("[Created][Repost] User {} gained {} experience", user_info.did.to_string(), gained_experience);
        }
        _ => {}
    }
}

async fn persist_character_changes(repository: &Arc<DatabaseRepository>, mut character: &mut Character, character_experience: CharacterExperience, new_experience_dto: LevelResponse) {
    repository.character.increment_character_experience(
        character_experience,
        new_experience_dto.clone(),
    ).await;

    repository.character.update_character(
        &mut character,
        new_experience_dto,
    ).await;
}