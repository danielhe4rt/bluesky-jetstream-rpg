mod create;
mod delete;

use std::sync::Arc;
use jetstream_oxide::events::commit::CommitEvent;
use paris::info;
use crate::events::create::create_event_handler;
use crate::repositories::DatabaseRepository;

pub async fn events_handler(repository: &Arc<DatabaseRepository>, commit: CommitEvent) {

    match commit {
        CommitEvent::Create { info: user_info, commit } => {
            create_event_handler(repository, user_info, commit).await;
        }
        CommitEvent::Update { info: user_info, commit } => {
            info!("Update event received: {:?}", user_info);
        },
        CommitEvent::Delete { info: user_info, commit } => {

            info!("Delete event received: {:?}", commit.collection.to_string());
        }
    }
}

