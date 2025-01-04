use atrium_api::app::bsky::feed::post::RecordEmbedRefs;
use atrium_api::record::KnownRecord;
use atrium_api::types::Union::Refs;



pub struct LikePostEvent {
    text: String,
    length: u32,
    has_image: bool,
    image_has_alt_text: bool,
}

impl LikePostEvent {
    pub fn new() -> Self {
        LikePostEvent {
            text: "".to_string(),
            length: 0,
            has_image: false,
            image_has_alt_text: false,
        }
    }
}

impl  LikePostEvent {
    pub fn handle(&mut self, record: KnownRecord) {
        let record = if let KnownRecord::AppBskyFeedPost(record) = record {
            record
        } else {
            return;
        };

        self.text = record.text.clone();
        self.length = record.text.len() as u32;


        let embed = record.embed.clone();
        if let Some(embed) = embed {
            if let Refs(RecordEmbedRefs::AppBskyEmbedImagesMain(embed_image)) = embed {
                self.has_image = true;
                self.image_has_alt_text = !embed_image.images.iter().find(|image| image.alt.is_empty()).is_some();
            }
        }
    }

    pub fn calculate_exp(&self) -> i32 {
        10
    }
}