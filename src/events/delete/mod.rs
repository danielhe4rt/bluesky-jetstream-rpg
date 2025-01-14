use crate::repositories::DatabaseRepository;
use jetstream_oxide::events::commit::CommitInfo;
use jetstream_oxide::events::EventInfo;
use std::sync::Arc;

pub async fn _delete_event_handler(_: &Arc<DatabaseRepository>, _: EventInfo, _: CommitInfo) {}
