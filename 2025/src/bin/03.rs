#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use aoc_shared::time_execution_us;
static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("🌟 --- Day 3 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution_us(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} us");
}

fn max_joltage(bank: &str, digits: usize) -> usize {
    let bytes = bank.as_bytes();
    let len = bytes.len();

    (0..digits)
        .rev()
        .scan(0, |left, i| {
            let slice = &bytes[*left..len - i];

            let (high_pos, highest) = slice
                .iter()
                .enumerate()
                .try_fold((0, 0u8), |best, (idx, &byte)| {
                    let result = if byte > best.1 { (idx, byte) } else { best };
                    if byte == b'9' {
                        Err(result)
                    } else {
                        Ok(result)
                    }
                })
                .unwrap_or_else(|result| result);

            *left += high_pos + 1;
            Some((highest - b'0') as usize)
        })
        .fold(0, |acc, digit| acc * 10 + digit)
}

fn part_1(input: &str) -> usize {
    input.trim().lines().map(|line| max_joltage(line, 2)).sum()
}

fn part_2(input: &str) -> usize {
    input.trim().lines().map(|line| max_joltage(line, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 357);
        assert_eq!(part_1(INPUT_TXT), 17613);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 3_121_910_778_619);
        assert_eq!(part_2(INPUT_TXT), 175_304_218_462_560);
    }
}
