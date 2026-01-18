/**
 * Store for preserving scroll positions across page navigation.
 * Scroll offsets are stored in memory (session-only, not persisted).
 */
const scrollStore = $state({
    musicList: 0,
    albumList: 0
});

export default scrollStore;
