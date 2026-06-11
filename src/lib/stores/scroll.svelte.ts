/**
 * Store for preserving scroll positions across page navigation.
 * Scroll offsets are stored in memory (session-only, not persisted).
 */
const scrollStore = $state({
	tracks: 0,
	albums: 0
});

export default scrollStore;
