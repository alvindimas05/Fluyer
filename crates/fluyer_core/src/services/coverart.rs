use super::musicbrainz::MusicBrainz;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct CoverArtService {
    cache_dir: PathBuf,
    /// Cache paths with an online fetch running. Every grid row of one album asks
    /// at once on a cold cache; only the first fetches, the rest reload on the
    /// cover-loaded event. ponytail: failures aren't remembered, so a missing cover
    /// is retried next launch; add a negative cache if that gets noisy.
    in_flight: Mutex<HashSet<PathBuf>>,
}

/// Removes the in-flight mark when the fetch ends, including on error or cancel.
struct InFlight<'a>(&'a Mutex<HashSet<PathBuf>>, PathBuf);

impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        self.0.lock().unwrap().remove(&self.1);
    }
}

impl CoverArtService {
    pub fn new(cache_dir: &Path) -> Self {
        let art_cache = cache_dir.join("covers");
        let _ = std::fs::create_dir_all(&art_cache);
        Self {
            cache_dir: art_cache,
            in_flight: Mutex::new(HashSet::new()),
        }
    }

    pub fn get_cached(
        &self,
        artist: &str,
        album: Option<&str>,
        title: Option<&str>,
    ) -> Option<Vec<u8>> {
        self.get_cached_with_fallback(artist, album, title, None)
    }

    pub fn get_cached_with_fallback(
        &self,
        artist: &str,
        album: Option<&str>,
        title: Option<&str>,
        fallback: Option<&str>,
    ) -> Option<Vec<u8>> {
        let path = self.cache_path(artist, album, title, fallback);
        std::fs::read(path).ok()
    }

    pub fn save_to_cache(
        &self,
        artist: &str,
        album: Option<&str>,
        title: Option<&str>,
        fallback: Option<&str>,
        bytes: &[u8],
    ) -> std::io::Result<()> {
        let path = self.cache_path(artist, album, title, fallback);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(path, bytes)
    }

    pub async fn fetch_and_cache(
        &self,
        artist: &str,
        album: Option<&str>,
        title: Option<&str>,
    ) -> Result<Option<Vec<u8>>, String> {
        if let Some(cached) = self.get_cached(artist, album, title) {
            return Ok(Some(cached));
        }
        let Some(_guard) = self.begin_fetch(self.cache_path(artist, album, title, None)) else {
            return Ok(None); // same cover already being fetched
        };

        let url = MusicBrainz::get_cover_art_url(artist, album, title).await?;
        let Some(image_url) = url else {
            return Ok(None);
        };

        let res = reqwest::get(&image_url).await.map_err(|e| e.to_string())?;
        let bytes = res.bytes().await.map_err(|e| e.to_string())?.to_vec();

        let _ = self.save_to_cache(artist, album, title, None, &bytes);

        Ok(Some(bytes))
    }

    fn begin_fetch(&self, key: PathBuf) -> Option<InFlight<'_>> {
        self.in_flight
            .lock()
            .unwrap()
            .insert(key.clone())
            .then(|| InFlight(&self.in_flight, key))
    }

    fn cache_path(
        &self,
        artist: &str,
        album: Option<&str>,
        title: Option<&str>,
        fallback: Option<&str>,
    ) -> PathBuf {
        let (folder, name) = if let Some(alb) = album.filter(|s| !s.trim().is_empty()) {
            (
                "album",
                if artist.trim().is_empty() {
                    alb.to_string()
                } else {
                    format!("{} - {}", artist, alb)
                },
            )
        } else if let Some(tit) = title.filter(|s| !s.trim().is_empty()) {
            (
                "track",
                if artist.trim().is_empty() {
                    tit.to_string()
                } else {
                    format!("{} - {}", artist, tit)
                },
            )
        } else if !artist.trim().is_empty() {
            ("misc", artist.to_string())
        } else if let Some(fb) = fallback {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            fb.hash(&mut hasher);
            ("fallback", format!("{:016x}", hasher.finish()))
        } else {
            ("misc", "unknown".to_string())
        };

        let sanitized: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == ' ' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        self.cache_dir
            .join(folder)
            .join(format!("{}.jpg", sanitized))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_fetch_of_same_cover_is_skipped_until_first_ends() {
        let svc = CoverArtService::new(&std::env::temp_dir().join("fluyer-coverart-test"));
        let key = svc.cache_path("Dabin", Some("Between Broken"), None, None);
        let first = svc.begin_fetch(key.clone());
        assert!(first.is_some());
        assert!(svc.begin_fetch(key.clone()).is_none());
        drop(first);
        assert!(svc.begin_fetch(key).is_some());
    }
}
