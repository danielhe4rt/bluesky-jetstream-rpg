mod fetch_user_profile;

use crate::http::fetch_user_profile::handle;
use crate::repositories::DatabaseRepository;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::sync::Arc;

struct AppState {
    repository: Arc<DatabaseRepository>,
}

pub async fn start_http(repository: &Arc<DatabaseRepository>) -> std::io::Result<()> {
    let repository = Arc::clone(repository);

    let app_state = Data::new(AppState { repository });
    HttpServer::new(move || App::new().app_data(app_state.clone()).service(handle))
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .workers(1)
        .run()
        .await
}
