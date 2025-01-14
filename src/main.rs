//! A very basic example of how to listen for create/delete events on a specific DID and NSID.

mod events;
mod http;
mod jetstream;
mod leveling;
mod models;
mod repositories;

use paris::Logger;
use scylla::{CachingSession, SessionBuilder};

use crate::http::start_http;
use crate::jetstream::start_jetstream;
use actix_web::rt::signal;
use scylla::transport::session::{CurrentDeserializationApi, GenericSession};
use std::sync::Arc;
use tokio::task::JoinSet;

#[actix_web::main]
async fn main() {
    Logger::new();
    env_logger::init();

    let session = start_scylla_session().await;
    let caching_session = Arc::new(CachingSession::from(session, 50));

    let repository = Arc::new(repositories::DatabaseRepository::new(Arc::clone(
        &caching_session,
    )));

    let mut join = JoinSet::new();
    let jetstream_repository = Arc::clone(&repository);
    join.spawn(async move {
        start_jetstream(&jetstream_repository).await;
    });

    join.spawn(async move {
        let _ = start_http(&repository).await;
    });

    // Listen for Ctrl+C (SIGINT) or termination signals (SIGTERM)
    let signal_handle = tokio::spawn(async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("Ctrl+C pressed. Shutting down...");
    });

    // Wait for tasks or signals
    tokio::select! {
        _ = signal_handle => {
            println!("Received shutdown signal.");
        }
        _ = async {
            while let Some(task) = join.join_next().await {
                if let Err(e) = task {
                    eprintln!("Task failed: {:?}", e);
                }
            }
        } => {
            println!("All tasks completed or terminated.");
        }
    }

    println!("Connection to Jetstream lost. Application shutting down.");
}

async fn start_scylla_session() -> GenericSession<CurrentDeserializationApi> {
    let session = SessionBuilder::new()
        .known_nodes(vec![
            "localhost:19042",
            "localhost:19043",
            "localhost:19044",
        ])
        .build()
        .await
        .expect("Failed to create Scylla session");

    session
        .use_keyspace("bsky_rpg", false)
        .await
        .expect("Failed to use keyspace");
    session
}
