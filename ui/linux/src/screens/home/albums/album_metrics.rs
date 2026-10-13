//! Card sizing shared by the carousel and album grid. Port of macOS `AlbumMetrics`.
//! ponytail: GTK widths are already logical px, so only the 1x rule rows apply.

pub const CARD_GAP: f64 = 12.0;
pub const EDGE: f64 = 16.0;
pub const LABEL_HEIGHT: f64 = 52.0;

/// (min window width, fraction of available width per card)
const RULES: [(f64, f64); 6] = [
    (1536.0, 0.125),    // 8 items
    (1440.0, 0.142857), // 7
    (1280.0, 0.16667),  // 6
    (1024.0, 0.2),      // 5
    (768.0, 0.25),      // 4
    (640.0, 0.33334),   // 3
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlbumMetrics {
    pub item_width: f64,
    pub cover_size: f64,
    pub columns: u32,
}

impl AlbumMetrics {
    pub fn new(width: f64) -> Self {
        let available = (width - EDGE * 2.0 + CARD_GAP).max(1.0);
        let ratio = RULES
            .iter()
            .find(|(min, _)| width >= *min)
            .map_or(0.5, |(_, r)| *r);
        let item_width = ratio * available;
        Self {
            item_width,
            cover_size: (item_width - CARD_GAP).max(16.0),
            columns: ((available / item_width).round() as u32).max(1),
        }
    }

    pub fn height(&self) -> f64 {
        self.cover_size + LABEL_HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_fill_width_exactly() {
        for w in [500.0, 800.0, 1100.0, 1600.0] {
            let m = AlbumMetrics::new(w);
            let n = m.columns as f64;
            let used = n * m.cover_size + (n - 1.0) * CARD_GAP + EDGE * 2.0;
            assert!((used - w).abs() < 1.0, "w={w} used={used}");
        }
        assert_eq!(AlbumMetrics::new(1100.0).columns, 5);
        assert_eq!(AlbumMetrics::new(500.0).columns, 2);
    }
}
