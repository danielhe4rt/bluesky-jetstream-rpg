// services.rs
use harper_core::linting::{Lint, LintGroup, LintGroupConfig, Linter, SpellCheck};
use harper_core::parsers::PlainEnglish;
use harper_core::{remove_overlaps, Dictionary, Document, FstDictionary, Lrc};
use rust_bert::pipelines::sentiment::{Sentiment, SentimentModel};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task;

// Wrapper structs for our services
pub struct SentimentService {
    model: Arc<RwLock<SentimentModel>>,
}

impl SentimentService {
    pub async fn new() -> Self {
        // Initialize the SentimentModel in a blocking thread
        let model = task::spawn_blocking(|| SentimentModel::new(Default::default()))
            .await
            .expect("Failed to initialize SentimentModel")
            .expect("Error during SentimentModel initialization");

        Self {
            model: Arc::new(RwLock::new(model)),
        }
    }

    pub async fn analyze(&self, text: &str) -> Vec<Sentiment> {
        let model = self.model.read().await;
        model.predict(vec![text])
    }
}

pub struct SpellCheckService {
    lint_group: Arc<RwLock<LintGroup<Arc<FstDictionary>>>>,
    dict: Arc<FstDictionary>,
    parser: Box<PlainEnglish>,
}

impl SpellCheckService {
    pub async fn new() -> Self {
        // Initialize the SpellCheck instance in a blocking thread
        let lint_group = tokio::task::spawn_blocking(move || {
            LintGroup::new(LintGroupConfig::default(), FstDictionary::curated())
        })
        .await
        .expect("Failed to initialize SpellCheck");

        Self {
            lint_group: Arc::new(RwLock::new(lint_group)),
            dict: FstDictionary::curated(),
            parser: Box::new(PlainEnglish),
        }
    }

    pub async fn check(&mut self, text: String) -> Vec<Lint> {
        let mut checker = self.lint_group.write().await;
        let mut lints = checker.lint(&Document::new_from_vec(
            Lrc::new(text.chars().collect()),
            &self.parser,
            &self.dict,
        ));

        remove_overlaps(&mut lints);

        lints
    }
}

pub struct NLPServices {
    pub sentiment_model: Arc<Mutex<SentimentModel>>,
    pub spellcheck: Arc<Mutex<SpellCheckService>>,
}

impl NLPServices {
    pub async fn new() -> Self {
        let sentiment_model = tokio::task::spawn_blocking(|| {
            SentimentModel::new(Default::default()).expect("Failed to initialize SentimentModel")
        })
        .await
        .expect("Blocking task failed");

        let spellcheck = SpellCheckService::new().await;

        Self {
            sentiment_model: Arc::new(Mutex::new(sentiment_model)),
            spellcheck: Arc::new(Mutex::new(spellcheck)),
        }
    }
}

use once_cell::sync::OnceCell;

pub static NLP_SERVICES: OnceCell<NLPServices> = OnceCell::new();

pub async fn initialize_nlp_services() -> Result<(), NLPServices> {
    NLP_SERVICES.set(NLPServices::new().await)
}
