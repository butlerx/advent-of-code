use std::io::Error;

pub fn run(input: String) -> Result<i64, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "".to_string();
        let results = run(input).unwrap();
        assert!(results == 0);
    }
}
