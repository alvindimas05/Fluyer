use crate::lyric::{cache, types::*};
use std::fs;

/// Normalize string for comparison (lowercase, no punctuation, normalized whitespace)
fn normalize_string(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let m = s1_chars.len();
    let n = s2_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut matrix = vec![vec![0usize; m + 1]; n + 1];

    for i in 0..=m {
        matrix[0][i] = i;
    }
    for j in 0..=n {
        matrix[j][0] = j;
    }

    for j in 1..=n {
        for i in 1..=m {
            let indicator = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[j][i] = (matrix[j][i - 1] + 1)
                .min(matrix[j - 1][i] + 1)
                .min(matrix[j - 1][i - 1] + indicator);
        }
    }

    matrix[n][m]
}

/// Calculate string similarity (0.0 to 1.0)
fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    let longer = if s1.len() > s2.len() { s1 } else { s2 };
    let shorter = if s1.len() > s2.len() { s2 } else { s1 };

    if longer.is_empty() {
        return 1.0;
    }

    let distance = levenshtein_distance(longer, shorter);
    (longer.len() - distance) as f64 / longer.len() as f64
}

/// Score a lyric result against the query (ultra-strict mode with duration check)
fn score_result(result: &LrcLibResult, query: &LyricQuery) -> f64 {
    // First check: duration must be within ±3 seconds or reject immediately
    if let (Some(result_duration), Some(query_duration)) = (result.duration, query.duration) {
        let duration_diff = (result_duration - (query_duration as f64 / 1000.0)).abs();
        if duration_diff > 3.0 {
            return 0.0; // Reject immediately if duration is off by more than 3 seconds
        }
    }

    let mut score = 0.0;

    // Title matching (weight: 0.4)
    let title_similarity = calculate_similarity(
        &normalize_string(&result.name),
        &normalize_string(&query.title),
    );
    score += title_similarity * 0.4;

    // Artist matching (weight: 0.3)
    // Get first artist if multiple (split by separator)
    let primary_artist = query.artist.split(" • ").next().unwrap_or(&query.artist);
    let artist_similarity = calculate_similarity(
        &normalize_string(&result.artist_name),
        &normalize_string(primary_artist),
    );
    score += artist_similarity * 0.3;

    // Duration matching (weight: 0.3) - only for fine-tuning since we already filtered
    if let (Some(result_duration), Some(query_duration)) = (result.duration, query.duration) {
        let duration_diff = (result_duration - (query_duration as f64 / 1000.0)).abs();
        let duration_similarity = 1.0 - duration_diff / 3.0; // Perfect match = 1, 3s diff = 0
        score += duration_similarity * 0.3;
    }

    score
}

/// Request lyrics from LrcLib API and cache the result
pub async fn request_lyrics(query: LyricQuery) -> Option<String> {
    let client = reqwest::Client::new();

    // Build search queries - try multiple approaches
    let primary_artist = query.artist.split(" • ").next().unwrap_or(&query.artist);
    let search_queries = vec![
        format!("{} {}", query.title, primary_artist),
        query.title.clone(),
    ];

    let mut all_results: Vec<LrcLibResult> = Vec::new();

    for search_query in search_queries {
        let url = format!(
            "https://lrclib.net/api/search?q={}",
            urlencoding::encode(&search_query)
        );

        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(results) = response.json::<Vec<LrcLibResult>>().await {
                    all_results.extend(results);
                    // If we got good results with the primary query, don't try others
                    if all_results.len() >= 3 {
                        break;
                    }
                }
            }
            Err(e) => {
                crate::warn!("LrcLib request failed: {}", e);
            }
        }
    }

    if all_results.is_empty() {
        return None;
    }

    // Remove duplicates based on name + artist
    let mut unique_results: Vec<LrcLibResult> = Vec::new();
    for result in all_results {
        let exists = unique_results.iter().any(|r| {
            r.name.to_lowercase() == result.name.to_lowercase()
                && r.artist_name.to_lowercase() == result.artist_name.to_lowercase()
        });
        if !exists {
            unique_results.push(result);
        }
    }

    // Score and sort results
    let mut scored_results: Vec<(LrcLibResult, f64)> = unique_results
        .into_iter()
        .filter(|r| r.synced_lyrics.is_some()) // Must have synced lyrics
        .map(|r| {
            let score = score_result(&r, &query);
            (r, score)
        })
        .filter(|(_, score)| *score > 0.4) // Minimum similarity threshold
        .collect();

    scored_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Get the best match
    let best_result = scored_results.into_iter().next()?;
    let lyrics = best_result.0.synced_lyrics?;

    crate::info!("Found lyrics with confidence score: {:.2}", best_result.1);

    // Save to cache
    let cache_key = generate_cache_key(&query);
    let cache_path = format!("{}/{}", cache::get_cache_directory(), cache_key);

    if let Err(e) = fs::write(&cache_path, &lyrics) {
        crate::warn!("Failed to cache lyrics: {}", e);
    }

    Some(lyrics)
}

/// Generate a cache key from the query
pub fn generate_cache_key(query: &LyricQuery) -> String {
    let primary_artist = query.artist.split(" • ").next().unwrap_or(&query.artist);
    // Sanitize for filesystem
    let key = format!("{} - {}", primary_artist, query.title);
    key.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
