use charybdis::macros::{charybdis_model, charybdis_view_model};
use charybdis::types::{Counter, Frozen, Int, Map, Text, Timestamp};


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
    pub event_at: Timestamp
}

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

#[derive(Default)]
#[charybdis_model(
    table_name = characters,
    partition_keys = [user_id],
    clustering_keys = []
)]
pub struct Character {
    pub user_id: Text,
    pub name: Text,
    pub current_experience: Int,
    pub experience_to_next_level: Int,
    pub level: Int,
}

#[charybdis_model(
    table_name = characters_experience,
    partition_keys = [user_id],
    clustering_keys = []
)]
pub struct CharacterExperience {
    pub user_id: Text,
    pub current_experience: Counter,
}