use crate::worker::*;

/// Builder for creating a [Worker] with a specific [Workload].
pub struct WorkerBuilder<W: Workload> {
    workload: W,
    mem_size: u128,
    keep_alive: bool,
}

impl<W: Workload> WorkerBuilder<W> {
    /// Creates a new [WorkerBuilder] with a specific [Workload].
    pub fn new(workload: W) -> WorkerBuilder<W> {
        WorkerBuilder {
            workload,
            mem_size: 128 * 1024,
            keep_alive: false,
        }
    }

    /// Sets the memory size of the [Worker].
    pub fn mem_size(mut self, mem_size: u128) -> Self {
        self.mem_size = mem_size;
        self
    }

    /// Sets whether the [Worker] should keep running after the workload is done.
    pub fn keep_alive(mut self, keep_alive: bool) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    /// Builds the [Worker] with the specified configuration.
    pub fn build(self) -> Worker<W> {
        Worker {
            workload: self.workload,
            mem_size: self.mem_size,
            keep_alive: self.keep_alive,
        }
    }
}
