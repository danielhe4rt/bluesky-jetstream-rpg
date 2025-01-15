pub mod create;
mod delete;
pub mod dto;

use crate::events::create::create_event_handler;
use crate::repositories::DatabaseRepository;
use jetstream_oxide::events::commit::{CommitData, CommitEvent};
use jetstream_oxide::events::EventInfo;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::Semaphore;

enum AppBskyEventRecord {
    Post,
    Like,
    Repost,
}

impl Display for AppBskyEventRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppBskyEventRecord::Post => write!(f, "app.bsky.feed.post"),
            AppBskyEventRecord::Like => write!(f, "app.bsky.feed.like"),
            AppBskyEventRecord::Repost => write!(f, "app.bsky.feed.repost"),
        }
    }
}

pub struct CreateEventPayload {
    event_info: EventInfo,
    commit_data: CommitData,
}

impl CreateEventPayload {
    fn new(event_info: EventInfo, commit_data: CommitData) -> Self {
        CreateEventPayload {
            event_info,
            commit_data,
        }
    }
}

pub async fn events_handler(
    repository: &Arc<DatabaseRepository>,
    commit: CommitEvent,
    semaphore: Arc<Semaphore>,
) {
    match commit {
        CommitEvent::Create {
            info: user_info,
            commit,
        } => {
            let payload = CreateEventPayload::new(user_info, commit);

            create_event_handler(repository, payload, semaphore).await;
        }
        CommitEvent::Delete { .. } => {
            // delete_event_handler(repository, info, commit).await;
        }
        CommitEvent::Update { .. } => {
            // update_event_handler(repository, info, commit).await;
        }
    }
}
