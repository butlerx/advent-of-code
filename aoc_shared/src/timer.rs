use std::time::Instant;

/// Measures the execution time of a closure in milliseconds.
///
/// Returns a tuple of the closure's result and the elapsed time in milliseconds.
///
/// # Examples
///
/// ```
/// use aoc_shared::time_execution;
/// let (result, ms) = time_execution(|| 2 + 2);
/// assert_eq!(result, 4);
/// ```
pub fn time_execution<F, T>(f: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed().as_millis();
    (result, duration)
}

/// Measures the execution time of a closure in microseconds.
///
/// Returns a tuple of the closure's result and the elapsed time in microseconds.
///
/// # Examples
///
/// ```
/// use aoc_shared::time_execution_us;
/// let (result, us) = time_execution_us(|| 2 + 2);
/// assert_eq!(result, 4);
/// ```
pub fn time_execution_us<F, T>(f: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed().as_micros();
    (result, duration)
}
