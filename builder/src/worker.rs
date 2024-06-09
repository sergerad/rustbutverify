#[allow(dead_code)]
pub struct Worker<W> {
    pub workload: W,
    pub mem_size: u128,
    pub keep_alive: bool,
}

// Workload trait is used as a bound in our builder
pub trait Workload {
    fn work(&self);
}
