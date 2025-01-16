use crate::models::udts::leveling::Leveling;
use charybdis::macros::charybdis_model;
use charybdis::types::{Frozen, Map, Text, Timestamp};

#[charybdis_model(
    table_name = events,
    partition_keys = [user_did],
    clustering_keys = [event_at],
    table_options = r#"
          CLUSTERING ORDER BY (event_at DESC)
    "#
)]
pub struct Events {
    pub user_did: Text,
    pub event_type: Text,
    pub event_id: Text,
    pub event_data: Frozen<Map<Text, Text>>,
    pub leveling_state: Leveling,
    pub event_at: Timestamp,
}
