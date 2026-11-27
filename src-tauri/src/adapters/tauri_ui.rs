use std::sync::Arc;

use tauri::{AppHandle, Emitter};

use crate::application::{DocumentNotifier, ReviewService};
use crate::domain::Document;

// outbound: push every change to the webview
pub struct TauriNotifier(pub AppHandle);

impl DocumentNotifier for TauriNotifier {
    fn changed(&self, docs: &[Document]) {
        let _ = self.0.emit("documents", docs);
    }
}

// inbound: the webview asks for the current state on load
#[tauri::command]
pub fn documents(service: tauri::State<Arc<ReviewService>>) -> Vec<Document> {
    service.documents()
}
