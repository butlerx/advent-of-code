fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(_input: &str) -> i64 {
    0
}

fn part_2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod day_1_tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 0);
        assert_eq!(part_1(include_str!("../../input/01.txt")), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
        assert_eq!(part_2(include_str!("../../input/01.txt")), 0);
    }
}