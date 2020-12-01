use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// Loads the Content of a file in to String
pub fn read(name: String) -> io::Result<String> {
    let mut s = String::new();
    File::open(Path::new("input").join(name))?.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::read;

    #[test]
    fn test_input() {
        let results = read("test.txt".to_string()).unwrap();
        assert!(results.trim() == "test".to_string());
    }
}
