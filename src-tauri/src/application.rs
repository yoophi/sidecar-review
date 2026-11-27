use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::{Block, Document, Origin};

// outbound ports
pub trait DocumentRepository: Send {
    fn take(&mut self, session: &str) -> Option<Document>;
    /// Saved documents come back last from `all`, so the UI can follow the latest activity.
    fn save(&mut self, doc: Document);
    fn all(&self) -> Vec<Document>;
}

pub trait DocumentNotifier: Send + Sync {
    fn changed(&self, docs: &[Document]);
}

// inbound port: the use cases the adapters call
pub struct ReviewService {
    repo: Mutex<Box<dyn DocumentRepository>>,
    notifier: Box<dyn DocumentNotifier>,
}

impl ReviewService {
    pub fn new(repo: Box<dyn DocumentRepository>, notifier: Box<dyn DocumentNotifier>) -> Self {
        Self { repo: Mutex::new(repo), notifier }
    }

    /// Appends to the session's document and returns its block count.
    pub fn push(&self, session: String, goal: String, blocks: Vec<Block>, origin: Origin) -> usize {
        let mut repo = self.repo.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64);
        let mut doc = repo.take(&session).unwrap_or_else(|| Document::new(session, now));
        doc.append(goal, blocks, origin, now);
        let count = doc.blocks.len();
        repo.save(doc);
        self.notifier.changed(&repo.all());
        count
    }

    pub fn documents(&self) -> Vec<Document> {
        self.repo.lock().unwrap().all()
    }
}
