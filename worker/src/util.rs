use core_extensions::measure_time::measure;

pub fn measure_execution_time<R, F: FnOnce() -> R>(label: &str, func: F) -> R {
    let (dur, out) = measure(func);
    println!(
        "measure_execution_time, {}, {}",
        label,
        (dur.as_micros() as f64) / 1000.0
    );
    out
}
