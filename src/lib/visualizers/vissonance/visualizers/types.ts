export interface Visualizer {
    initialize: () => void,
    make: () => Promise<void>,
    destroy: () => void,
    render: () => void
}