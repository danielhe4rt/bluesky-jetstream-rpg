use std::collections::HashMap;
use std::sync::Arc;
use charybdis::types::Timestamp;
use scylla::CachingSession;
use charybdis::operations::{Find, Insert};
use scylla::_macro_internal::SerializeRow;
use crate::leveling::LevelResponse;
use crate::models::{Character, EventTracker};

pub struct EventRepository {
    pub session: Arc<CachingSession>,
}

impl EventRepository {
    pub fn new(connection: Arc<CachingSession>) -> Self {
        Self {
            session: Arc::clone(&connection),
        }
    }

    pub async fn insert_event(&self, user_id: String, event_id: String, event_at: u64, level_response: LevelResponse) {
        let map = HashMap::new();

        let event = EventTracker {
            user_id,
            event_type: "repost".to_string(),
            event_id,
            event_data: map,
            xp: level_response.experience,
            event_at: Timestamp::from_timestamp_nanos(event_at as i64),
        };

        event
            .insert()
            .execute(&self.session)
            .await
            .expect("Failed to insert event");
    }
}