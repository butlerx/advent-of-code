pub fn part_1(_input: &str) -> i64 {
    0
}

pub fn part_2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert!(part_1(INPUT) == 0);
        let results = part_1(include_str!("../input/day_10.txt"));
        println!("{}", results);
        assert!(results == 0);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT) == 0);
        let results = part_2(include_str!("../input/day_10.txt"));
        println!("{}", results);
        assert!(results == 0);
    }
}
