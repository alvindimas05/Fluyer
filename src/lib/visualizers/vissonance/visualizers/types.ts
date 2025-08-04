export interface Visualizer {
    initialize: () => void,
    make: () => void,
    destroy: () => void,
    render: () => void
}