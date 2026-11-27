use std::path::PathBuf;
use std::{fs, io};

use crate::application::DocumentRepository;
use crate::domain::Document;

// shortcut: rewrites one JSON file per save and never prunes; move to SQLite when documents get large or need deleting
pub struct FileRepo {
    path: PathBuf,
    docs: Vec<Document>,
}

impl FileRepo {
    /// A missing file is an empty repo. An unreadable one is an error, so it is never overwritten.
    pub fn open(path: PathBuf) -> io::Result<Self> {
        let docs = match fs::read(&path) {
            Ok(bytes) => serde_json::from_slice(&bytes)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => vec![],
            Err(e) => return Err(e),
        };
        Ok(Self { path, docs })
    }

    fn persist(&self) -> io::Result<()> {
        // write-then-rename: a crash mid-write cannot truncate the saved documents
        let tmp = self.path.with_extension("tmp");
        fs::write(&tmp, serde_json::to_vec(&self.docs)?)?;
        fs::rename(tmp, &self.path)
    }
}

impl DocumentRepository for FileRepo {
    fn take(&mut self, session: &str) -> Option<Document> {
        let i = self.docs.iter().position(|d| d.session == session)?;
        Some(self.docs.remove(i))
    }

    fn save(&mut self, doc: Document) {
        self.docs.push(doc);
        if let Err(e) = self.persist() {
            eprintln!("sidecar: could not save documents to {}: {e}", self.path.display());
        }
    }

    fn all(&self) -> Vec<Document> {
        self.docs.clone()
    }
}

#[cfg(test)]
pub fn temp(name: &str) -> FileRepo {
    let path = std::env::temp_dir().join(format!("sidecar-test-{}-{name}.json", std::process::id()));
    let _ = fs::remove_file(&path);
    FileRepo::open(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn survives_reopen() {
        let mut repo = temp("reopen");
        repo.save(Document::new("s1".into(), 0));
        repo.save(Document::new("s2".into(), 0));
        let path = repo.path.clone();

        let mut reopened = FileRepo::open(path.clone()).unwrap();
        assert_eq!(reopened.all().len(), 2);
        assert_eq!(reopened.take("s1").unwrap().session, "s1");

        fs::write(&path, "not json").unwrap();
        assert!(FileRepo::open(path).is_err());
    }
}
