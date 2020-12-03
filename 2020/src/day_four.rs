use std::io::Error;

pub fn run(_input: String) -> Result<i64, Error> {
    Ok(0)
}

pub fn part_1(input: String) -> Result<i64, Error> {
    run(input)
}

pub fn part_2(input: String) -> Result<i64, Error> {
    run(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert!(part_1(INPUT.to_string()).unwrap() == 0);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT.to_string()).unwrap() == 0);
    }

    #[test]
    fn test_run() {
        assert!(run(INPUT.to_string()).unwrap() == 0);
    }
}
