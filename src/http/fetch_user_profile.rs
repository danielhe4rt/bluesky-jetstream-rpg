use crate::http::AppState;
use crate::models::character::Character;
use crate::models::CharacterExperience;
use actix_web::{get, web, HttpResponse, Responder};
use charybdis::types::Counter;
use paris::info;
use serde_json::json;

#[get("/find/{profile_did}")]
pub async fn handle(
    app: web::Data<AppState>,
    profile_did: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let profile_did = profile_did.into_inner();

    let character = app
        .repository
        .character
        .find_by_partition_key(profile_did.clone())
        .await;
    info!("Finding character for user {}", profile_did);
    let character = match character {
        Some(character) => character,
        None => {
            let response = app
                .repository
                .bsky
                .get_author_profile(profile_did.clone())
                .await;
            info!("Creating new character for user {}", profile_did);
            let character = Character::from(response);

            let character_experience = CharacterExperience {
                user_id: profile_did.clone(),
                current_experience: Counter(0),
            };

            app.repository
                .character
                .increment_character_experience(
                    character_experience,
                    character.current_experience as i64,
                )
                .await;

            character
        }
    };

    Ok(HttpResponse::Ok().json(json!(character)))
}
