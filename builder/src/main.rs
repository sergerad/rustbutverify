#[allow(dead_code)]
struct Worker<W> {
    workload: W,
    mem_size: u128,
    keep_alive: bool,
}

// NoWorkload allows empty builders to be created
struct NoWorkload;

// So that we can do Worker::builder()
impl Worker<NoWorkload> {
    fn builder() -> WorkerBuilder<NoWorkload> {
        WorkerBuilder {
            workload: NoWorkload,
            mem_size: 128 * 1024,
            keep_alive: false,
        }
    }
}

impl Worker<HelloWorkload<'_>> {
    fn work(&self) {
        println!("{}", self.workload.0);
    }
}

// Generic worker builder
struct WorkerBuilder<W> {
    workload: W,
    mem_size: u128,
    keep_alive: bool,
}

// Implement builder methods that apply for all workloads
impl<W> WorkerBuilder<W> {
    fn mem_size(&mut self, mem_size: u128) -> &mut Self {
        self.mem_size = mem_size;
        self
    }
    fn keep_alive(&mut self, keep_alive: bool) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }
}

#[derive(Clone, Debug)]
struct HelloWorkload<'a>(&'a str);

impl<'a> From<&'a str> for HelloWorkload<'a> {
    fn from(s: &'a str) -> Self {
        HelloWorkload(s)
    }
}

impl<'a> WorkerBuilder<NoWorkload> {
    // TODO: for generic workloads!
    // Return a worker builder from no workload, to one with a String workload
    fn workload(&self, workload: impl Into<HelloWorkload<'a>>) -> WorkerBuilder<HelloWorkload<'a>> {
        WorkerBuilder {
            workload: workload.into(),
            mem_size: self.mem_size,
            keep_alive: self.keep_alive,
        }
    }
}

impl<'a> WorkerBuilder<HelloWorkload<'a>> {
    fn build(&mut self) -> Worker<HelloWorkload> {
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
    // Must assign builder before worker, try the following to check:
    //let mut worker = Worker::builder()
    //    .mem_size(256 * 1024)
    //    .keep_alive(true)
    //    .workload("hello world")
    //    .build();
    //worker.work();

    // Create a builder, then worker
    let mut builder = Worker::builder()
        .mem_size(256 * 1024)
        .keep_alive(true)
        .workload("hello world");
    let hello_worker = builder.build();

    // Do work
    hello_worker.work();
}
