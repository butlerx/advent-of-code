static INPUT_TXT: &str = include_str!("../../input/07.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn part_1(_input: &str) -> i64 {
    0
}

fn part_2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod day_7_tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 0);
        assert_eq!(part_1(INPUT_TXT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
        assert_eq!(part_2(INPUT_TXT), 0);
    }
}
