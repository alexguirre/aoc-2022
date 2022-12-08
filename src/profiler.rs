use std::time::Instant;

pub struct Profiler {
    start: Instant
}

impl Profiler {
    fn new() -> Self {
        Self { start: Instant::now() }
    }
}

impl Drop for Profiler {
    fn drop(&mut self) {
        println!("Took {} μs", self.start.elapsed().as_micros())
    }
}

pub fn profile() -> Profiler {
    Profiler::new()
}