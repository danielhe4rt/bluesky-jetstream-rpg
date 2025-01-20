use crate::models::udts::leveling::Leveling;
use charybdis::macros::charybdis_view_model;
use charybdis::types::{Frozen, Map, Text, Timestamp};

#[charybdis_view_model(
    base_table = events,
    table_name = events_by_type,
    partition_keys = [user_did, event_type],
    clustering_keys = [event_at],
)]
pub struct Events {
    pub user_did: Text,
    pub event_type: Text,
    pub event_id: Text,
    pub event_data: Frozen<Map<Text, Text>>,
    pub leveling_state: Leveling,
    pub event_at: Timestamp,
}
