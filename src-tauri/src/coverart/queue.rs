use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::coverart::types::{CoverArtRequest, CoverArtRequestStatus};

lazy_static! {
    static ref COVER_ART_QUEUE: Mutex<Vec<CoverArtRequest>> = Mutex::new(vec![]);
}

pub fn get_queue(album: String) -> Option<CoverArtRequest> {
    let index = COVER_ART_QUEUE
        .lock()
        .unwrap()
        .iter()
        .position(|e| e.name == album);
    if index.is_none() {
        return None;
    }
    Some(
        COVER_ART_QUEUE
            .lock()
            .unwrap()
            .get(index.unwrap())
            .unwrap()
            .clone(),
    )
}

pub fn set_status(name: String, status: CoverArtRequestStatus) {
    remove(name.clone());
    COVER_ART_QUEUE
        .lock()
        .unwrap()
        .push(CoverArtRequest { name, status });
}

pub fn remove(name: String) {
    COVER_ART_QUEUE.lock().unwrap().retain(|c| c.name == name);
}
