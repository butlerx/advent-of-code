use std::io::Error;

pub fn part_1(input: &str) -> Result<i64, Error> {
    Ok(0)
}

pub fn part_2(input: &str) -> Result<i64, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert!(part_1(INPUT).unwrap() == 0);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT).unwrap() == 0);
    }
}
