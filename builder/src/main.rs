#[allow(dead_code)]
struct Worker<W> {
    workload: W,
    mem_size: u128,
    keep_alive: bool,
}

// Workload trait is used as a bound in our builder
trait Workload {
    fn work(&self);
}

// NoWorkload allows empty builders to be created
struct NoWorkload;
impl Workload for NoWorkload {
    fn work(&self) {}
}

// Generic worker builder
struct WorkerBuilder<W: Workload> {
    workload: W,
    mem_size: u128,
    keep_alive: bool,
}

// Implement builder methods that apply for all workloads
impl<W: Workload> WorkerBuilder<W> {
    fn mem_size(&mut self, mem_size: u128) -> &mut Self {
        self.mem_size = mem_size;
        self
    }
    fn keep_alive(&mut self, keep_alive: bool) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }
}

impl WorkerBuilder<NoWorkload> {
    fn new() -> WorkerBuilder<NoWorkload> {
        WorkerBuilder {
            workload: NoWorkload,
            mem_size: 128 * 1024,
            keep_alive: false,
        }
    }
    // Return a worker builder from no workload, to one with a String workload
    fn workload<W: Workload>(&self, workload: W) -> WorkerBuilder<W> {
        WorkerBuilder {
            workload,
            mem_size: self.mem_size,
            keep_alive: self.keep_alive,
        }
    }
}

// A workload that prints a string
#[derive(Clone, Debug)]
struct HelloWorkload<'a>(&'a str);
impl Workload for HelloWorkload<'_> {
    fn work(&self) {
        println!("{}", self.0);
    }
}
impl<'a> From<&'a str> for HelloWorkload<'a> {
    fn from(s: &'a str) -> Self {
        HelloWorkload(s)
    }
}

// Generic fn for building worker from builder
impl<W: Workload + Clone> WorkerBuilder<W> {
    fn build(&mut self) -> Worker<W> {
        Worker {
            workload: self.workload.clone(),
            mem_size: self.mem_size,
            keep_alive: self.keep_alive,
        }
    }
}

// Rust has no variadic argument list for functions (e.g. println!() is a marcro, not a func that takes variadic arguments).
// Rust has no default values for function arguments.
fn main() {
    // Create a builder, then worker (try doing it all in one line to see compiler complain)
    let mut builder = WorkerBuilder::new()
        .mem_size(256 * 1024)
        .keep_alive(true)
        .workload(HelloWorkload("hello world"));
    let hello_worker = builder.build();

    // Do work (could add a fn from worker to avoid using workload directly)
    hello_worker.workload.work();
}
