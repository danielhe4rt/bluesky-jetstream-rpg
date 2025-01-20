use paris::Logger;

#[derive(Debug)]
pub struct AppSettings {
    pub bsky_topics: Vec<String>,
    pub bsky_dids: Option<Vec<String>>,
    pub max_workers: usize,
}

impl AppSettings {
    pub fn new() -> Self {
        Logger::new();
        env_logger::init();
        dotenvy::from_filename(".env").expect("Failed to load .env file");
        
        let bsky_topics = dotenvy::var("BSKY_TOPICS")
            .unwrap_or("app.bsky.feed.post".to_string())
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let bsky_dids = dotenvy::var("BSKY_DIDS")
            .unwrap();
        
        let max_workers = dotenvy::var("MAX_WORKERS")
            .unwrap_or("5".to_string())
            .parse::<usize>()
            .expect("Failed to parse WORKERS");

        let bsky_dids: Option<Vec<String>> = if !bsky_dids.is_empty() {
            let dids = bsky_dids
                .split(',')
                .map(|s| s.to_string())
                .collect();

            Some(dids)
        } else {
            None
        };


        Self {
            bsky_topics,
            bsky_dids,
            max_workers
        }
    }
}