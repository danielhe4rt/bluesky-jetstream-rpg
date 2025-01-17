use std::default;

use crate::leveling::get_base_level_from_bsky_profile;
use crate::models::udts::leveling::Leveling;
use atrium_api::app::bsky::actor::defs::ProfileViewDetailed;
use charybdis::macros::{charybdis_model, charybdis_udt_model};
use charybdis::types::Text;
use rust_bert::pipelines::sentiment::{Sentiment, SentimentModel, SentimentPolarity};
use serde::Serialize;

#[derive(Default, Serialize)]
#[charybdis_model(
    table_name = characters,
    partition_keys = [user_did],
    clustering_keys = []
)]
pub struct Character {
    pub user_did: Text,           // profile_did
    pub name: Text,               // handle
    pub leveling_state: Leveling, // udt leveling state
    pub alignment: UserAlignment,
}

impl From<ProfileViewDetailed> for Character {
    fn from(response: ProfileViewDetailed) -> Self {
        let level_response = get_base_level_from_bsky_profile(&response);

        Self {
            user_did: response.did.clone().to_string(),
            name: response.handle.clone().to_string(),
            leveling_state: Leveling::from(level_response),
            alignment: UserAlignment::new(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[charybdis_udt_model(type_name = moralAxis)]
pub struct MoralAxis {
    good: i32,
    neutral: i32,
    evil: i32,
}

#[derive(Debug, Default, Serialize)]
#[charybdis_udt_model(type_name = ethicalAxis)]
pub struct EthicalAxis {
    lawful: i32,
    neutral: i32,
    chaotic: i32,
}

#[derive(Debug, Default, Serialize)]
#[charybdis_udt_model(type_name = userAlignment)]
pub struct UserAlignment {
    moral: MoralAxis,
    ethical: EthicalAxis,
    current_align: String,
}

impl UserAlignment {
    pub fn new() -> Self {
        Self {
            moral: MoralAxis::default(),
            ethical: EthicalAxis::default(),
            current_align: Alignment::TrueNeutral.to_string(),
        }
    }

    pub fn update_alignment_from_text(&mut self, text: &str) {
        let sentiment_classifier = SentimentModel::new(Default::default()).unwrap();
        let result = sentiment_classifier.predict(vec![text]);

        match *result.first().unwrap() {
            Sentiment {
                polarity: SentimentPolarity::Positive,
                score,
            } => {
                let points = (score * 100.0) as i32;

                self.moral.good += points;
                self.moral.evil = (self.moral.evil - (points / 4)).max(0);

                // If score is very high, reduce neutral slightly
                if score > 0.7 {
                    self.moral.neutral = (self.moral.neutral - (points / 6)).max(0);
                }
            }
            Sentiment {
                polarity: SentimentPolarity::Negative,
                score,
            } => {
                let points = (score * 100.0) as i32;

                self.moral.evil += points;
                self.moral.good = (self.moral.good - (points / 4)).max(0);

                // If score is very high, reduce neutral slightly
                if score > 0.7 {
                    self.moral.neutral = (self.moral.neutral - (points / 6)).max(0);
                }
            }
        }

        // TODO: update ethical from text

        self.update_current_alignment();
    }

    fn update_current_alignment(&mut self) {
        let moral_leaning =
            if self.moral.good > self.moral.evil && self.moral.good > self.moral.neutral {
                Moral::Good
            } else if self.moral.evil > self.moral.good && self.moral.evil > self.moral.neutral {
                Moral::Evil
            } else {
                Moral::Neutral
            };

        let ethical_leaning = if self.ethical.lawful > self.ethical.chaotic
            && self.ethical.lawful > self.ethical.neutral
        {
            Ethical::Lawful
        } else if self.ethical.chaotic > self.ethical.lawful
            && self.ethical.chaotic > self.ethical.neutral
        {
            Ethical::Chaotic
        } else {
            Ethical::Neutral
        };

        self.current_align = match (ethical_leaning, moral_leaning) {
            (Ethical::Chaotic, Moral::Evil) => Alignment::ChaoticEvil.to_string(),
            (Ethical::Chaotic, Moral::Neutral) => Alignment::ChaoticNeutral.to_string(),
            (Ethical::Chaotic, Moral::Good) => Alignment::ChaoticGood.to_string(),
            (Ethical::Neutral, Moral::Evil) => Alignment::NeutralEvil.to_string(),
            (Ethical::Neutral, Moral::Neutral) => Alignment::TrueNeutral.to_string(),
            (Ethical::Neutral, Moral::Good) => Alignment::NeutralGood.to_string(),
            (Ethical::Lawful, Moral::Evil) => Alignment::LawfulEvil.to_string(),
            (Ethical::Lawful, Moral::Neutral) => Alignment::LawfulNeutral.to_string(),
            (Ethical::Lawful, Moral::Good) => Alignment::LawfulGood.to_string(),
        };
    }
}

#[derive(Debug, Serialize)]

pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

impl Alignment {
    pub fn to_string(self) -> String {
        match self {
            Alignment::LawfulGood => "lawful good".to_string(),
            Alignment::NeutralGood => "neutral good".to_string(),
            Alignment::ChaoticGood => "chaotic good".to_string(),
            Alignment::LawfulNeutral => "lawful neutral".to_string(),
            Alignment::TrueNeutral => "true neutral".to_string(),
            Alignment::ChaoticNeutral => "chaotic neutral".to_string(),
            Alignment::LawfulEvil => "lawful evil".to_string(),
            Alignment::NeutralEvil => "neutral evil".to_string(),
            Alignment::ChaoticEvil => "chaotic evil".to_string(),
        }
    }
}

impl From<String> for Alignment {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Alignment {
    fn from(value: &str) -> Self {
        match value {
            "lawful good" => Alignment::LawfulGood,
            "neutral good" => Alignment::NeutralGood,
            "chaotic good" => Alignment::ChaoticGood,
            "lawful neutral" => Alignment::LawfulNeutral,
            "true neutral" => Alignment::TrueNeutral,
            "chaotic neutral" => Alignment::ChaoticNeutral,
            "lawful evil" => Alignment::LawfulEvil,
            "neutral evil" => Alignment::NeutralEvil,
            "chaotic evil" => Alignment::ChaoticEvil,
            _ => panic!("Unknown Alignment: {}", value),
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self::TrueNeutral
    }
}

enum Moral {
    Evil,
    Neutral,
    Good,
}

enum Ethical {
    Chaotic,
    Neutral,
    Lawful,
}
