fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|num| num.trim().parse::<i64>().unwrap())
                .sum()
        })
        .collect()
}

fn part_1(input: &str) -> i64 {
    *parse_input(input).iter().max().unwrap()
}

fn part_2(input: &str) -> i64 {
    let mut nums = parse_input(input);
    nums.sort();
    nums.iter().rev().take(3).sum()
}

#[cfg(test)]
mod day_1_tests {
    use super::*;
    static INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 24000);
        assert_eq!(part_1(include_str!("../../input/01.txt")), 66186);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 45000);
        assert_eq!(part_2(include_str!("../../input/01.txt")), 196804);
    }
}
