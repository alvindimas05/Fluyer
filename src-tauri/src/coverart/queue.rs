use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::watch;

use crate::coverart::types::{CoverArtRequest, CoverArtRequestStatus};

lazy_static::lazy_static! {
    static ref COVER_ART_QUEUE: Mutex<HashMap<String, CoverArtEntry>> = Mutex::new(HashMap::new());
}

struct CoverArtEntry {
    request: CoverArtRequest,
    sender: watch::Sender<CoverArtRequestStatus>,
    receiver: watch::Receiver<CoverArtRequestStatus>,
}

pub fn get_queue(name: &str) -> Option<CoverArtRequest> {
    let queue = COVER_ART_QUEUE.lock().unwrap();
    queue.get(name).map(|entry| entry.request.clone())
}

/// Get a receiver to wait for status changes
pub fn get_receiver(name: &str) -> Option<watch::Receiver<CoverArtRequestStatus>> {
    let queue = COVER_ART_QUEUE.lock().unwrap();
    queue.get(name).map(|entry| entry.receiver.clone())
}

/// Set status and notify all waiters
pub fn set_status(name: String, status: CoverArtRequestStatus) {
    let mut queue = COVER_ART_QUEUE.lock().unwrap();

    if let Some(entry) = queue.get_mut(&name) {
        // Update existing entry and notify waiters
        entry.request.status = status;
        let _ = entry.sender.send(status);
    } else {
        // Create new entry with watch channel
        let (sender, receiver) = watch::channel(status);
        queue.insert(
            name.clone(),
            CoverArtEntry {
                request: CoverArtRequest { name, status },
                sender,
                receiver,
            },
        );
    }
}

pub fn remove(name: &str) {
    COVER_ART_QUEUE.lock().unwrap().remove(name);
}

/// Wait for a pending request to complete (become Loaded or Failed)
pub async fn wait_for_result(name: &str) -> CoverArtRequestStatus {
    let receiver = {
        let queue = COVER_ART_QUEUE.lock().unwrap();
        queue.get(name).map(|entry| entry.receiver.clone())
    };

    if let Some(mut rx) = receiver {
        // Wait until status changes from Pending
        loop {
            let status = *rx.borrow();
            if status != CoverArtRequestStatus::Pending {
                return status;
            }
            // Wait for the next status update
            if rx.changed().await.is_err() {
                // Sender dropped, treat as failed
                return CoverArtRequestStatus::Failed;
            }
        }
    }

    CoverArtRequestStatus::Failed
}
