use crate::events::events_handler;
use crate::repositories::DatabaseRepository;
use atrium_api::types::string::{Did, Nsid};
use jetstream_oxide::events::JetstreamEvent::Commit;
use jetstream_oxide::{
    DefaultJetstreamEndpoints, JetstreamCompression, JetstreamConfig, JetstreamConnector,
};
use paris::info;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::args::AppSettings;

pub async fn start_jetstream(settings: Arc<AppSettings>, repository: &Arc<DatabaseRepository>) {
    let config = JetstreamConfig {
        endpoint: DefaultJetstreamEndpoints::USEastTwo.into(),
        wanted_collections: settings
            .bsky_topics
            .iter()
            .map(|s| Nsid::new(s.to_string()).expect("Failed to create NSID"))
            .collect(),
        wanted_dids: settings
            .bsky_dids
            .as_ref()
            .map(|dids| dids.iter().map(|s| Did::new(s.to_string()).expect("Failed to create DID")).collect())
            .unwrap_or_default(),
        compression: JetstreamCompression::Zstd,
        cursor: None,
    };

    let (receiver, _) = JetstreamConnector::new(config)
        .expect("Failed to create Jetstream connector")
        .connect()
        .await
        .expect("Failed to connect to Jetstream");

    info!("Starting Jetstream listener");

    let semaphore = Arc::new(Semaphore::new(settings.max_workers));

    while let Ok(event) = receiver.recv_async().await {
        if let Commit(commit) = event {
            events_handler(repository, commit, Arc::clone(&semaphore)).await;
        }
    }
}
