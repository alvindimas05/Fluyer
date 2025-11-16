use crate::playlist::playlist::Playlist;

pub fn playlist_get_all() -> Vec<Playlist> {
    Playlist::get_all()
}