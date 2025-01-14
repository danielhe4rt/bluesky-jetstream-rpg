use crate::events::events_handler;
use crate::repositories::DatabaseRepository;
use atrium_api::types::string::{Did, Nsid};
use jetstream_oxide::events::JetstreamEvent::Commit;
use jetstream_oxide::{
    DefaultJetstreamEndpoints, JetstreamCompression, JetstreamConfig, JetstreamConnector,
};
use paris::info;
use std::sync::Arc;

pub async fn start_jetstream(repository: &Arc<DatabaseRepository>) {
    let config = JetstreamConfig {
        endpoint: DefaultJetstreamEndpoints::USEastTwo.into(),
        wanted_collections: vec![
            Nsid::new("app.bsky.feed.post".to_string()).expect("Failed to create NSID"),
            Nsid::new("app.bsky.feed.like".to_string()).expect("Failed to create NSID"),
            Nsid::new("app.bsky.feed.repost".to_string()).expect("Failed to create NSID"),
        ],
        wanted_dids: vec![
            Did::new("did:plc:doqrpcaai4iqmkbdo3ztmlld".to_string()).expect("Failed to create DID"),
        ],
        compression: JetstreamCompression::Zstd,
        cursor: None,
    };

    let (receiver, _) = JetstreamConnector::new(config)
        .expect("Failed to create Jetstream connector")
        .connect()
        .await
        .expect("Failed to connect to Jetstream");

    info!("Starting Jetstream listener");

    while let Ok(event) = receiver.recv_async().await {
        if let Commit(commit) = event {
            events_handler(repository, commit).await;
        }
    }
}
