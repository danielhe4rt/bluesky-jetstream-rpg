mod character_repository;
mod event_repository;

use std::sync::Arc;
use scylla::CachingSession;
use crate::repositories::character_repository::CharacterRepository;
use crate::repositories::event_repository::EventRepository;

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