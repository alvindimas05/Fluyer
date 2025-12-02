import type { MusicData } from "$lib/features/music/types";
import axios from "axios";
import {MusicConfig} from "$lib/constants/MusicConfig";

interface LrcLibResult {
	name: string;
	artistName: string;
	albumName: string;
	duration: number;
	syncedLyrics: string | null;
	plainLyrics: string | null;
}

const LrcLib = {
	// Helper function to normalize strings for comparison
	normalizeString: (str: string): string => {
		return str
			.toLowerCase()
			.replace(/[^\w\s]/g, "") // Remove punctuation
			.replace(/\s+/g, " ") // Normalize whitespace
			.trim();
	},

	// Helper function to calculate string similarity (simple Levenshtein-based)
	calculateSimilarity: (str1: string, str2: string): number => {
		const longer = str1.length > str2.length ? str1 : str2;
		const shorter = str1.length > str2.length ? str2 : str1;

		if (longer.length === 0) return 1.0;

		const editDistance = LrcLib.levenshteinDistance(longer, shorter);
		return (longer.length - editDistance) / longer.length;
	},

	levenshteinDistance: (str1: string, str2: string): number => {
		const matrix = Array(str2.length + 1)
			.fill(null)
			.map(() => Array(str1.length + 1).fill(null));

		for (let i = 0; i <= str1.length; i++) matrix[0][i] = i;
		for (let j = 0; j <= str2.length; j++) matrix[j][0] = j;

		for (let j = 1; j <= str2.length; j++) {
			for (let i = 1; i <= str1.length; i++) {
				const indicator = str1[i - 1] === str2[j - 1] ? 0 : 1;
				matrix[j][i] = Math.min(
					matrix[j][i - 1] + 1, // deletion
					matrix[j - 1][i] + 1, // insertion
					matrix[j - 1][i - 1] + indicator, // substitution
				);
			}
		}

		return matrix[str2.length][str1.length];
	},

	// Option 1: Very strict duration matching (±5 seconds max)
	scoreResult: (result: LrcLibResult, music: MusicData): number => {
		let score = 0;

		// Title matching (weight: 0.4)
		const titleSimilarity = LrcLib.calculateSimilarity(
			LrcLib.normalizeString(result.name),
			LrcLib.normalizeString(music.title || ""),
		);
		score += titleSimilarity * 0.4;

		// Artist matching (weight: 0.3)
		if (music.artist && result.artistName) {
			const artistSimilarity = LrcLib.calculateSimilarity(
				LrcLib.normalizeString(result.artistName),
				LrcLib.normalizeString(
					music.artist.split(MusicConfig.artistSeparator)[0],
				),
			);
			score += artistSimilarity * 0.3;
		}

		// STRICT Duration matching (weight: 0.3)
		if (music.duration && result.duration) {
			const durationDiff = Math.abs(
				result.duration - Math.floor(music.duration / 1000),
			);

			// Only allow ±5 seconds tolerance
			if (durationDiff <= 5) {
				const durationSimilarity = 1 - durationDiff / 5; // Linear falloff within 5s
				score += durationSimilarity * 0.3;
			} else {
				// Heavily penalize anything beyond 5 seconds
				score *= 0.1; // Reduce total score by 90%
			}
		}

		return score;
	},

	// Option 2: Ultra-strict - exact duration match required
	scoreResultUltraStrict: (result: LrcLibResult, music: MusicData): number => {
		// First check: duration must be within ±3 seconds or reject immediately
		if (music.duration && result.duration) {
			const durationDiff = Math.abs(
				result.duration - Math.floor(music.duration / 1000),
			);
			if (durationDiff > 3) {
				return 0; // Reject immediately if duration is off by more than 3 seconds
			}
		}

		let score = 0;

		// Title matching (weight: 0.4)
		const titleSimilarity = LrcLib.calculateSimilarity(
			LrcLib.normalizeString(result.name),
			LrcLib.normalizeString(music.title || ""),
		);
		score += titleSimilarity * 0.4;

		// Artist matching (weight: 0.3)
		if (music.artist && result.artistName) {
			const artistSimilarity = LrcLib.calculateSimilarity(
				LrcLib.normalizeString(result.artistName),
				LrcLib.normalizeString(
					music.artist.split(MusicConfig.artistSeparator)[0],
				),
			);
			score += artistSimilarity * 0.3;
		}

		// Duration matching (weight: 0.3) - only for fine-tuning since we already filtered
		if (music.duration && result.duration) {
			const durationDiff = Math.abs(
				result.duration - Math.floor(music.duration / 1000),
			);
			const durationSimilarity = 1 - durationDiff / 3; // Perfect match = 1, 3s diff = 0
			score += durationSimilarity * 0.3;
		}

		return score;
	},

	// Option 3: Exponential penalty for duration differences
	scoreResultExponentialPenalty: (
		result: LrcLibResult,
		music: MusicData,
	): number => {
		let score = 0;

		// Title matching (weight: 0.4)
		const titleSimilarity = LrcLib.calculateSimilarity(
			LrcLib.normalizeString(result.name),
			LrcLib.normalizeString(music.title || ""),
		);
		score += titleSimilarity * 0.4;

		// Artist matching (weight: 0.3)
		if (music.artist && result.artistName) {
			const artistSimilarity = LrcLib.calculateSimilarity(
				LrcLib.normalizeString(result.artistName),
				LrcLib.normalizeString(
					music.artist.split(MusicConfig.artistSeparator)[0],
				),
			);
			score += artistSimilarity * 0.3;
		}

		// EXPONENTIAL Duration penalty (weight: 0.3)
		if (music.duration && result.duration) {
			const durationDiff = Math.abs(
				result.duration - Math.floor(music.duration / 1000),
			);

			// Exponential decay: perfect match = 1, each second reduces score exponentially
			const durationSimilarity = Math.exp(-durationDiff / 3); // e^(-diff/3)
			score += durationSimilarity * 0.3;

			// Additional penalty: if off by more than 10 seconds, multiply total score by penalty
			if (durationDiff > 10) {
				score *= 0.2; // 80% penalty to total score
			}
		}

		return score;
	},

	getLyrics: async (music: MusicData): Promise<string | null> => {
		try {
			if (!music.title) return null;

			// Prepare search queries - try multiple approaches
			const searchQueries = [
				// Primary query: title + primary artist
				`${music.title} ${music.artist ? music.artist.split(MusicConfig.artistSeparator)[0] : ""}`.trim(),
				// Fallback: just title
				music.title,
				// If artist has multiple parts, try with full artist name
				...(music.artist && music.artist.includes(MusicConfig.artistSeparator)
					? [`${music.title} ${music.artist}`]
					: []),
			];

			let bestResults: LrcLibResult[] = [];

			// Try each search query
			for (const query of searchQueries) {
				try {
					const url = new URL("https://lrclib.net/api/search");
					url.searchParams.append("q", query);

					const res = await axios.get<LrcLibResult[]>(url.toString());

					if (res.data.length > 0) {
						bestResults = [...bestResults, ...res.data];
						// If we got good results with the primary query, don't try others
						if (query === searchQueries[0] && res.data.length >= 3) break;
					}
				} catch (queryErr) {
					console.warn(`Query "${query}" failed:`, queryErr);
					continue;
				}
			}

			if (bestResults.length === 0) return null;

			// Remove duplicates based on name + artist
			const uniqueResults = bestResults.filter(
				(result, index, self) =>
					index ===
					self.findIndex(
						(r) =>
							r.name.toLowerCase() === result.name.toLowerCase() &&
							r.artistName.toLowerCase() === result.artistName.toLowerCase(),
					),
			);

			// Score and sort results
			const scoredResults = uniqueResults
				.map((result) => ({
					result,
					score: LrcLib.scoreResultUltraStrict(result, music),
				}))
				.filter(
					({ score, result }) =>
						score > 0.4 && // Minimum similarity threshold
						result.syncedLyrics, // Must have synced lyrics
				)
				.sort((a, b) => b.score - a.score);

			// Return the best match
			if (scoredResults.length > 0) {
				console.log(
					`Found lyrics with confidence score: ${scoredResults[0].score.toFixed(2)}`,
				);
				return scoredResults[0].result.syncedLyrics;
			}

			return null;
		} catch (err) {
			console.error("Error fetching lyrics:", err);
			return null;
		}
	},

	// Alternative method for exact search when you have precise metadata
    getExactLyrics: async (
		title: string,
		artist: string,
		album?: string,
		duration?: number,
	): Promise<string | null> => {
		try {
			const url = new URL("https://lrclib.net/api/get");
			url.searchParams.append("artist_name", artist);
			url.searchParams.append("track_name", title);
			if (album) url.searchParams.append("album_name", album);
			if (duration)
				url.searchParams.append("duration", Math.floor(duration).toString());

			const res = await axios.get<LrcLibResult>(url.toString());
			return res.data?.syncedLyrics || null;
		} catch (err) {
			// Fallback to search if exact match fails
			return null;
		}
	},
};

export default LrcLib;
