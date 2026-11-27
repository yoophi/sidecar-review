use std::sync::Mutex;

use crate::domain::{Block, Document};

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
    pub fn push(&self, session: String, goal: String, blocks: Vec<Block>) -> usize {
        let mut repo = self.repo.lock().unwrap();
        let mut doc = repo.take(&session).unwrap_or_else(|| Document::new(session));
        doc.append(goal, blocks);
        let count = doc.blocks.len();
        repo.save(doc);
        self.notifier.changed(&repo.all());
        count
    }

    pub fn documents(&self) -> Vec<Document> {
        self.repo.lock().unwrap().all()
    }
}
