use std::time::Instant;

pub fn measure_execution_time<R, F: FnOnce() -> R>(label: &str, func: F) -> R {
    let start = Instant::now();
    let out = func();
    println!(
        "measure_execution_time, {}, {}",
        label,
        start.elapsed().as_micros()
    );
    out
}
