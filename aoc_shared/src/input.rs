/// # Panics
///
/// Panics if the input file cannot be read.
#[must_use]
pub fn read_input(manifest_dir: &str, day: u32) -> String {
    let path = format!("{manifest_dir}/input/{day:02}.txt");
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {path}"))
}
