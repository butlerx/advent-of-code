#![warn(clippy::pedantic, clippy::perf)]
static INPUT_TXT: &str = include_str!("../../input/01.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn first_num(line: &str) -> u32 {
    line.chars()
        .find(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0)
}

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_num_words(mut line: &str) -> usize {
    loop {
        for (num, prefix) in NUMBERS.iter().enumerate() {
            if line.starts_with(prefix) || line.starts_with(&format!("{num}")) {
                return num;
            }
        }
        line = &line[1..];
    }
}

fn last_num_words(mut line: &str) -> usize {
    loop {
        for (num, suffix) in NUMBERS.iter().enumerate() {
            if line.ends_with(suffix) || line.ends_with(&format!("{num}")) {
                return num;
            }
        }
        line = &line[..line.len() - 1];
    }
}

fn part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| first_num(line) * 10 + first_num(&line.chars().rev().collect::<String>()))
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| first_num_words(line) * 10 + last_num_words(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 142);
        assert_eq!(part_1(INPUT_TXT), 53080);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_2), 281);
        assert_eq!(part_2(INPUT_TXT), 53268);
    }
}
