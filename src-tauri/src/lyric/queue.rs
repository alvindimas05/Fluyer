use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::watch;

use crate::lyric::types::{LyricRequest, LyricRequestStatus};

lazy_static::lazy_static! {
    static ref LYRIC_QUEUE: Mutex<HashMap<String, LyricEntry>> = Mutex::new(HashMap::new());
}

struct LyricEntry {
    request: LyricRequest,
    sender: watch::Sender<LyricRequestStatus>,
    receiver: watch::Receiver<LyricRequestStatus>,
}

pub fn get_queue(name: &str) -> Option<LyricRequest> {
    let queue = LYRIC_QUEUE.lock().unwrap();
    queue.get(name).map(|entry| entry.request.clone())
}

/// Get a receiver to wait for status changes
pub fn get_receiver(name: &str) -> Option<watch::Receiver<LyricRequestStatus>> {
    let queue = LYRIC_QUEUE.lock().unwrap();
    queue.get(name).map(|entry| entry.receiver.clone())
}

/// Set status and notify all waiters
pub fn set_status(name: String, status: LyricRequestStatus) {
    let mut queue = LYRIC_QUEUE.lock().unwrap();

    if let Some(entry) = queue.get_mut(&name) {
        // Update existing entry and notify waiters
        entry.request.status = status;
        let _ = entry.sender.send(status);
    } else {
        // Create new entry with watch channel
        let (sender, receiver) = watch::channel(status);
        queue.insert(
            name.clone(),
            LyricEntry {
                request: LyricRequest { name, status },
                sender,
                receiver,
            },
        );
    }
}

pub fn remove(name: &str) {
    LYRIC_QUEUE.lock().unwrap().remove(name);
}

/// Wait for a pending request to complete (become Loaded or Failed)
pub async fn wait_for_result(name: &str) -> LyricRequestStatus {
    let receiver = {
        let queue = LYRIC_QUEUE.lock().unwrap();
        queue.get(name).map(|entry| entry.receiver.clone())
    };

    if let Some(mut rx) = receiver {
        // Wait until status changes from Pending
        loop {
            let status = *rx.borrow();
            if status != LyricRequestStatus::Pending {
                return status;
            }
            // Wait for the next status update
            if rx.changed().await.is_err() {
                // Sender dropped, treat as failed
                return LyricRequestStatus::Failed;
            }
        }
    }

    LyricRequestStatus::Failed
}
