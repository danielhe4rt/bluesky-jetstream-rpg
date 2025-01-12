use crate::events::create::CreateEventHandler;
use crate::events::dto::NewEventDTO;

pub struct CreatePostEvent {}

impl CreatePostEvent {
    pub fn new() -> Self {
        CreatePostEvent {}
    }
}

#[async_trait::async_trait]
impl CreateEventHandler for CreatePostEvent {
    fn calculate_exp(&self, dto: &NewEventDTO) -> i32 {
        let mut exp = 30;

        if dto
            .context
            .get("has_image")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            exp += 100;
        }

        if dto
            .context
            .get("image_has_alt_text")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            exp += 50;
        }

        exp
    }
}
