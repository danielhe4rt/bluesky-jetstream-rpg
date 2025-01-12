use atrium_api::app::bsky::feed::get_author_feed;
use atrium_api::client::AtpServiceClient;
use atrium_api::types::string::AtIdentifier;
use atrium_xrpc_client::reqwest::ReqwestClient;
use paris::info;
use std::str::FromStr;

async fn base_crawler() {
    // TODO: maybe use CDC to get the initial state of the database and then listen for changes
    let client = AtpServiceClient::new(ReqwestClient::new("https://public.api.bsky.app"));

    let feed = client
        .service
        .app
        .bsky
        .feed
        .get_author_feed(
            get_author_feed::ParametersData {
                actor: AtIdentifier::from_str("danielhe4rt.dev")
                    .expect("Failed to create AtIdentifier"),
                cursor: None,
                limit: None,
                filter: None,
                include_pins: None,
            }
            .into(),
        )
        .await
        .expect("Failed to get feed");
    info!("Feed: {:?}", feed);

    feed.feed.iter().for_each(|feed| {
        println!("Feed: {:?}", feed.data);
    });
}
