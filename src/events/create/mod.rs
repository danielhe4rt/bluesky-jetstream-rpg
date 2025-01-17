use crate::events::create::create_post::CreatePostEvent;
use crate::events::create::like_post::LikePostEvent;
use crate::events::create::repost::RepostEvent;
use crate::events::dto::NewEventDTO;
use crate::events::CreateEventPayload;
use crate::leveling::{calculate_experience, LevelResponse};
use crate::models::character::Character;
use crate::models::character_experience::CharacterExperience;
use crate::repositories::DatabaseRepository;
use atrium_api::record::KnownRecord;
use atrium_api::record::KnownRecord::AppBskyFeedPost;
use charybdis::types::Counter;
use paris::info;
use std::sync::Arc;
use tokio::sync::Semaphore;
use KnownRecord::{AppBskyFeedLike, AppBskyFeedRepost};

pub mod create_post;
mod like_post;
mod repost;

#[async_trait::async_trait]
trait CreateEventHandler {
    async fn handle(
        &mut self,
        repository: &Arc<DatabaseRepository>,
        payload: &NewEventDTO,
    ) -> LevelResponse {
        // find all the data we need
        let character = repository
            .character
            .find_by_partition_key(payload.user_did.clone())
            .await;

        let mut character = match character {
            Some(character) => character,
            None => {
                let response = repository
                    .bsky
                    .get_author_profile(payload.user_did.clone())
                    .await;
                info!("Creating new character for user {}", payload.user_did);
                Character::from(response)
            }
        };

        character.alignment.update_alignment_from_text(payload.context.get("text")).await;

        let character_experience = repository
            .character
            .find_character_experience_by_partition_key(payload.user_did.clone())
            .await;

        let character_experience = match character_experience {
            Some(character_experience) => character_experience,
            None => {
                let character_experience = CharacterExperience {
                    user_did: payload.user_did.clone(),
                    current_experience: Counter(0),
                };

                repository
                    .character
                    .increment_character_experience(
                        character_experience,
                        character.leveling_state.experience as i64,
                    )
                    .await;

                CharacterExperience {
                    user_did: payload.user_did.clone(),
                    current_experience: Counter(character.leveling_state.experience as i64),
                }
            }
        };

        // calculate the experience
        let current_experience = character_experience.get_experience();
        let action_gained_experience = self.calculate_exp(payload);
        let new_experience = current_experience.saturating_add(action_gained_experience);
        let leveling_response_dto = calculate_experience(current_experience, new_experience);

        repository
            .character
            .update_character(&mut character, leveling_response_dto.clone())
            .await;

        repository
            .event
            .insert_event(payload, leveling_response_dto.clone())
            .await;

        // persist the changes
        repository
            .character
            .increment_character_experience(character_experience, action_gained_experience as i64)
            .await;

        leveling_response_dto
    }

    fn calculate_exp(&self, payload: &NewEventDTO) -> i32;
}

pub async fn create_event_handler(
    repository: &Arc<DatabaseRepository>,
    payload: CreateEventPayload,
    semaphore: Arc<Semaphore>,
) {
    let event_payload = NewEventDTO::from(&payload);

    let repo = Arc::clone(repository);
    let permit = semaphore.acquire_owned().await.unwrap(); // Acquire a semaphore permit

    tokio::spawn(async move {
        let response = select_event_handler(&payload.commit_data.record)
            .handle(&repo, &event_payload)
            .await;
        info!(
            "[Created][{}] User {} gained {} experience",
            event_payload.event_type, event_payload.user_did, response.experience
        );
        drop(permit); // Release the semaphore permit
    });
}

fn select_event_handler(record: &KnownRecord) -> Box<dyn CreateEventHandler + Send + Sync> {
    match record {
        AppBskyFeedPost(_) => Box::new(CreatePostEvent::new()),
        AppBskyFeedLike(_) => Box::new(LikePostEvent::new()),
        AppBskyFeedRepost(_) => Box::new(RepostEvent::new()),
        _ => panic!("Unknown event type"),
    }
}
