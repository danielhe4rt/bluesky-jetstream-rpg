use crate::leveling::LevelResponse;
use crate::models::{BlueskyEventTracker, EventTracker};
use charybdis::operations::{Find, Insert};
use charybdis::types::Timestamp;
use scylla::CachingSession;
use std::collections::HashMap;
use std::sync::Arc;

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

    pub async fn find_event_by_partition_key(&self, event_id: String) -> Option<BlueskyEventTracker> {
        let event = BlueskyEventTracker {
            event_id,
            ..Default::default()
        };

        event.find_by_partition_key()
            .execute(&self.session)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
            .pop()
    }
}