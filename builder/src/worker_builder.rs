use crate::worker::*;

// Generic worker builder
pub struct WorkerBuilder<W: Workload> {
    workload: W,
    mem_size: u128,
    keep_alive: bool,
}

// Implement builder methods that apply for all workloads
impl<W: Workload> WorkerBuilder<W> {
    pub fn mem_size(&mut self, mem_size: u128) -> &mut Self {
        self.mem_size = mem_size;
        self
    }
    pub fn keep_alive(&mut self, keep_alive: bool) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }
}

impl std::default::Default for WorkerBuilder<NoWorkload> {
    fn default() -> WorkerBuilder<NoWorkload> {
        WorkerBuilder {
            workload: NoWorkload,
            mem_size: 128 * 1024,
            keep_alive: false,
        }
    }
}

impl WorkerBuilder<NoWorkload> {
    // Return a worker builder from no workload, to one with a String workload
    pub fn workload<W: Workload>(&self, workload: W) -> WorkerBuilder<W> {
        WorkerBuilder {
            workload,
            mem_size: self.mem_size,
            keep_alive: self.keep_alive,
        }
    }
}

// Generic fn for building worker from builder
impl<W: Workload + Clone> WorkerBuilder<W> {
    pub fn build(self) -> Worker<W> {
        Worker {
            workload: self.workload,
            mem_size: self.mem_size,
            keep_alive: self.keep_alive,
        }
    }
}
