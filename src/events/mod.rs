mod create;
mod delete;

use std::sync::Arc;
use jetstream_oxide::events::commit::CommitEvent;
use paris::info;
use crate::events::create::create_event_handler;
use crate::events::delete::delete_event_handler;
use crate::repositories::DatabaseRepository;

enum EventRecord {
    AppBskyFeedPost,
    AppBskyFeedLike,
    AppBskyFeedRepost,
}

impl EventRecord {
    fn from_string(record: &str) -> Self {
        match record {
            "app.bsky.feed.post" => EventRecord::AppBskyFeedPost,
            "app.bsky.feed.like" => EventRecord::AppBskyFeedLike,
            "app.bsky.feed.repost" => EventRecord::AppBskyFeedRepost,
            _ => {
                info!("Unknown collection: {}", record);
                EventRecord::AppBskyFeedPost
            }
        }
    }
}


pub async fn events_handler(repository: &Arc<DatabaseRepository>, commit: CommitEvent) {
    match commit {
        CommitEvent::Create { info: user_info, commit } => {
            create_event_handler(repository, user_info, commit).await;
        },
        CommitEvent::Delete { commit, info } => {
            delete_event_handler(repository, info, commit).await;
        }
        _ => {}
    }
}

