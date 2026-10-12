use fluyer_core::audio::player::MusicPlayerSync;
use fluyer_core::events::EventSink;
use fluyer_core::metadata::MusicMetadata;

#[derive(Debug, Clone)]
pub enum AppEvent {
    PlayerSync(MusicPlayerSync),
    TrackChanged(Option<MusicMetadata>, usize),
    ScanProgress(usize, usize),
    Toast(String),
    TrackCoverLoaded(usize),
    AlbumCoverLoaded(usize),
    LyricsLoaded(String),
}

pub struct EventBridge(pub async_channel::Sender<AppEvent>);

impl EventSink for EventBridge {
    fn on_player_sync(&self, state: MusicPlayerSync) {
        let _ = self.0.send_blocking(AppEvent::PlayerSync(state));
    }

    fn on_track_changed(&self, track: Option<MusicMetadata>, index: usize) {
        let _ = self.0.send_blocking(AppEvent::TrackChanged(track, index));
    }

    fn on_scan_progress(&self, current: usize, total: usize) {
        let _ = self.0.send_blocking(AppEvent::ScanProgress(current, total));
    }

    fn on_toast(&self, message: &str) {
        let _ = self.0.send_blocking(AppEvent::Toast(message.to_string()));
    }

    fn on_track_cover_loaded(&self, index: usize) {
        let _ = self.0.send_blocking(AppEvent::TrackCoverLoaded(index));
    }

    fn on_album_cover_loaded(&self, index: usize) {
        let _ = self.0.send_blocking(AppEvent::AlbumCoverLoaded(index));
    }

    fn on_lyrics_loaded(&self, lyrics: &str) {
        let _ = self.0.send_blocking(AppEvent::LyricsLoaded(lyrics.to_string()));
    }
}
