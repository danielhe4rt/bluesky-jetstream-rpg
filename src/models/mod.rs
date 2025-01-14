pub mod character;

use charybdis::macros::{charybdis_model, charybdis_view_model};
use charybdis::types::{Counter, Frozen, Int, Map, Text, Timestamp};

// SELECT * FROM user_events WHERE event_id = 1;

#[charybdis_model(
    table_name = user_events,
    partition_keys = [user_id],
    clustering_keys = [event_at, event_type],
    table_options = r#"
          CLUSTERING ORDER BY (event_at DESC)
    "#
)]
pub struct EventTracker {
    pub user_id: Text,
    pub event_type: Text,
    pub event_id: Text,
    pub event_data: Frozen<Map<Text, Text>>,
    pub xp: Int,
    pub event_at: Timestamp,
}

#[derive(Default)]
#[charybdis_view_model(
    table_name=user_events_by_bluesky_id,
    base_table=user_events,
    partition_keys = [event_id],
    clustering_keys = [event_at, event_type, user_id]
)]
pub struct BlueskyEventTracker {
    pub user_id: Text,
    pub event_type: Text,
    pub event_id: Text,
    pub event_data: Frozen<Map<Text, Text>>,
    pub xp: Int,
    pub event_at: Timestamp,
}

// Lightweight Transactions = LWT = IF NOT EXISTS
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
