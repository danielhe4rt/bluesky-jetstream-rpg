use crate::events::{AppBskyEventRecord, CreateEventPayload};
use atrium_api::app::bsky::feed::post::RecordEmbedRefs;
use atrium_api::record::KnownRecord;
use atrium_api::types::Union::Refs;
use std::collections::HashMap;

pub struct NewEventDTO {
    pub user_did: String,
    pub event_id: String,
    pub event_type: String,
    pub posted_at: u64,
    pub context: HashMap<String, String>,
}

impl From<&CreateEventPayload> for NewEventDTO {
    fn from(payload: &CreateEventPayload) -> Self {
        let mut context = HashMap::new();
        match payload.commit_data.record.clone() {
            KnownRecord::AppBskyFeedPost(post) => {
                let mut has_image = false;
                let mut image_has_alt_text = false;
                let embed = post.embed.clone();

                if let Some(embed) = embed {
                    if let Refs(RecordEmbedRefs::AppBskyEmbedImagesMain(embed_image)) = embed {
                        has_image = true;
                        image_has_alt_text = !embed_image
                            .images
                            .iter()
                            .find(|image| image.alt.is_empty())
                            .is_some();
                    }
                }

                context.insert("text".to_string(), post.text.clone());
                context.insert("length".to_string(), post.text.len().to_string());
                context.insert("has_image".to_string(), has_image.to_string());
                context.insert(
                    "image_has_alt_text".to_string(),
                    image_has_alt_text.to_string(),
                );

                NewEventDTO {
                    user_did: payload.event_info.did.to_string(),
                    posted_at: payload.event_info.time_us,
                    event_id: payload.commit_data.info.rkey.clone(),
                    event_type: AppBskyEventRecord::Post.to_string(),
                    context,
                }
            }
            _ => NewEventDTO {
                user_did: payload.event_info.did.to_string(),
                posted_at: payload.event_info.time_us,
                event_id: payload.commit_data.info.rkey.clone(),
                event_type: AppBskyEventRecord::Post.to_string(),
                context,
            },
        }
    }
}
