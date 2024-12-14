use std::time::Instant;

pub fn time_execution<F, T>(f: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed().as_millis();
    (result, duration)
}
