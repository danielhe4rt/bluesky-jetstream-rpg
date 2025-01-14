use charybdis::macros::charybdis_model;
use charybdis::types::{Frozen, Map, Text, Timestamp};
use crate::models::character::Leveling;

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
    pub leveling_state: Leveling,
    pub event_at: Timestamp,
}