class PlaylistMoveQueue {
    private queue: Array<() => Promise<void>> = [];
    private isProcessing = false;

    async add(operation: () => Promise<void>): Promise<void> {
        return new Promise((resolve, reject) => {
            this.queue.push(async () => {
                try {
                    await operation();
                    resolve();
                } catch (error) {
                    reject(error);
                }
            });

            // Start processing if not already running
            if (!this.isProcessing) {
                this.processQueue();
            }
        });
    }

    private async processQueue(): Promise<void> {
        if (this.isProcessing || this.queue.length === 0) return;

        this.isProcessing = true;

        while (this.queue.length > 0) {
            const operation = this.queue.shift();
            if (operation) {
                try {
                    await operation();
                } catch (error) {
                    console.error('Queue operation failed:', error);
                }
            }
        }

        this.isProcessing = false;
    }

    getQueueLength(): number {
        return this.queue.length;
    }
}

export const playlistMoveQueue = new PlaylistMoveQueue();