pub mod character_repository;
pub mod event_repository;

use crate::repositories::character_repository::CharacterRepository;
use crate::repositories::event_repository::EventRepository;
use scylla::CachingSession;
use std::sync::Arc;

pub struct DatabaseRepository {
    pub character: CharacterRepository,
    pub event: EventRepository,
}

impl DatabaseRepository {
    pub fn new(connection: Arc<CachingSession>) -> Self {
        Self {
            character: CharacterRepository::new(Arc::clone(&connection)),
            event: EventRepository::new(Arc::clone(&connection)),
        }
    }
}
