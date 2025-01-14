use crate::events::dto::NewEventDTO;
use crate::leveling::LevelResponse;
use charybdis::operations::{Find, Insert};
use charybdis::types::Timestamp;
use scylla::CachingSession;
use std::sync::Arc;
use crate::models::character::Leveling;
use crate::models::events::EventTracker;

pub struct EventRepository {
    pub session: Arc<CachingSession>,
}

impl EventRepository {
    pub fn new(connection: Arc<CachingSession>) -> Self {
        Self {
            session: Arc::clone(&connection),
        }
    }

    pub async fn insert_event(&self, payload: &NewEventDTO, level_response: LevelResponse) {
        let event = EventTracker {
            user_id: payload.user_did.to_string(),
            event_type: payload.event_type.to_string(),
            event_id: payload.event_id.to_string(),
            event_data: payload.context.clone(),
            leveling_state: Leveling::from(level_response),
            event_at: Timestamp::from_timestamp_nanos(payload.posted_at as i64),
        };

        event
            .insert()
            .execute(&self.session)
            .await
            .expect("Failed to insert event");
    }

}
