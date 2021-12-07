fn main() {
    let input = include_str!("../../input/19.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

pub fn part_1(_input: &str) -> i64 {
    0
}

pub fn part_2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod day_19_tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 0);
        assert_eq!(part_1(include_str!("../../input/19.txt")), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
        assert_eq!(part_2(include_str!("../../input/19.txt")), 0);
    }
}
