use crate::events::create::CreateEventHandler;
use crate::events::dto::NewEventDTO;

pub struct LikePostEvent {}

impl LikePostEvent {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl CreateEventHandler for LikePostEvent {
    fn calculate_exp(&self, _: &NewEventDTO) -> i32 {
        10
    }
}
