use std::sync::{Arc, Mutex};
use tc_audio::MockTtsProvider;
use tc_db::Database;
use tc_ranking::RankingConfig;
use tc_timeline::{ProjectModel, TimelineHistory};

pub struct AppState {
    pub db: Arc<Database>,
    pub active_project: Mutex<Option<ProjectModel>>,
    pub history: Mutex<TimelineHistory>,
    pub ranking_config: Mutex<RankingConfig>,
    pub tts_provider: MockTtsProvider,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            active_project: Mutex::new(None),
            history: Mutex::new(TimelineHistory::new()),
            ranking_config: Mutex::new(RankingConfig::default()),
            tts_provider: MockTtsProvider::new(),
        }
    }

    pub fn in_memory() -> Self {
        let db = Database::open_in_memory().expect("Failed to create in-memory db");
        Self::new(db)
    }
}
