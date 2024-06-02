use builder::hello_workload::HelloWorkload;
use builder::worker::Workload;
use builder::worker_builder::WorkerBuilder;

// Rust has no variadic argument list for functions (e.g. println!() is a marcro, not a func that takes variadic arguments).
// Rust has no default values for function arguments.
fn main() {
    // Create a builder, then worker (try doing it all in one line to see compiler complain)
    let mut builder = WorkerBuilder::default()
        .mem_size(256 * 1024)
        .keep_alive(true)
        .workload(HelloWorkload("hello world"));
    let hello_worker = builder.build();

    // Do work (could add a fn from worker to avoid using workload directly)
    hello_worker.workload.work();
}
