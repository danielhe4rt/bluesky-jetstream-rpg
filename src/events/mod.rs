pub mod create;
mod delete;
pub mod dto;

use crate::events::create::create_event_handler;
use crate::repositories::DatabaseRepository;
use jetstream_oxide::events::commit::{CommitData, CommitEvent};
use jetstream_oxide::events::EventInfo;
use paris::info;
use std::fmt::Display;
use std::sync::Arc;

enum AppBskyEventRecord {
    Post,
    Like,
    Repost,
}

impl AppBskyEventRecord {
    fn from_string(record: &str) -> Self {
        match record {
            "app.bsky.feed.post" => AppBskyEventRecord::Post,
            "app.bsky.feed.like" => AppBskyEventRecord::Like,
            "app.bsky.feed.repost" => AppBskyEventRecord::Repost,
            _ => {
                info!("Unknown collection: {}", record);
                AppBskyEventRecord::Post
            }
        }
    }
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

struct CreateEventPayload {
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

pub async fn events_handler(repository: &Arc<DatabaseRepository>, commit: CommitEvent) {
    match commit {
        CommitEvent::Create {
            info: user_info,
            commit,
        } => {
            let payload = CreateEventPayload::new(user_info, commit);
            create_event_handler(repository, payload).await;
        }
        CommitEvent::Delete { .. } => {
            // delete_event_handler(repository, info, commit).await;
        }
        _ => {}
    }
}
