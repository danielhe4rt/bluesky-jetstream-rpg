//! A very basic example of how to listen for create/delete events on a specific DID and NSID.

mod events;
mod leveling;

mod models;
mod repositories;

use crate::events::events_handler;
use atrium_api::types::string::Nsid;
use std::sync::Arc;

use jetstream_oxide::{
    events::JetstreamEvent::Commit,
    DefaultJetstreamEndpoints,
    JetstreamCompression,
    JetstreamConfig,
    JetstreamConnector,
};
use paris::{info, Logger};
use scylla::{CachingSession, SessionBuilder};

#[tokio::main]
async fn main() {
    Logger::new();


    let config = JetstreamConfig {
        endpoint: DefaultJetstreamEndpoints::USEastTwo.into(),
        wanted_collections: vec![
            Nsid::new("app.bsky.feed.post".to_string()).expect("Failed to create NSID"),
            Nsid::new("app.bsky.feed.like".to_string()).expect("Failed to create NSID"),
            Nsid::new("app.bsky.feed.repost".to_string()).expect("Failed to create NSID"),
        ].into(),
        wanted_dids: vec![],
        compression: JetstreamCompression::Zstd,
        cursor: None,
    };

    let session = SessionBuilder::new()
        .known_nodes(vec!["localhost:19042", "localhost:19043", "localhost:19044"])
        .build()
        .await
        .expect("Failed to create Scylla session");

    session.use_keyspace("fodase", false).await.expect("Failed to use keyspace");

    let caching_session = Arc::new(CachingSession::from(session, 50));

    let (receiver, _) = JetstreamConnector::new(config)
        .expect("Failed to create Jetstream connector")
        .connect()
        .await
        .expect("Failed to connect to Jetstream");

    info!("Starting Jetstream listener");

    let repository = Arc::new(repositories::DatabaseRepository::new(Arc::clone(&caching_session)));

    while let Ok(event) = receiver.recv_async().await {

        if let Commit(commit) = event {
            events_handler(&repository, commit).await;
        }
    }

    println!("Connection to Jetstream lost.");
}