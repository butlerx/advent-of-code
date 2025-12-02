#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/02.txt");

fn main() {
    println!("🌟 --- Day 2 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_range(section: &str) -> impl Iterator<Item = usize> {
    let (start_str, end_str) = section.split_once('-').expect("Invalid range");
    let start = start_str.parse::<usize>().expect("Invalid start number");
    let end = end_str.parse::<usize>().expect("Invalid end number");
    start..=end
}

fn count_digits(n: usize) -> usize {
    std::iter::successors(Some(n), |&x| (x >= 10).then_some(x / 10)).count()
}

fn has_repeated_half(n: usize) -> bool {
    let digits = count_digits(n);
    if digits.is_multiple_of(2) {
        let half_digits = u32::try_from(digits / 2).expect("Failed to convert to u32");
        let divisor = 10_usize.pow(half_digits);
        n / divisor == n % divisor
    } else {
        false
    }
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|section| {
            parse_range(section)
                .filter(|&n| has_repeated_half(n))
                .sum::<usize>()
        })
        .sum()
}

fn has_repeating_pattern(n: usize) -> bool {
    let digits = count_digits(n);

    (1..=(digits / 2))
        .filter(|&pattern_len| digits.is_multiple_of(pattern_len))
        .any(|pattern_len| {
            let pattern_len = u32::try_from(pattern_len).expect("Failed to convert to u32");
            let divisor = 10_usize.pow(pattern_len);
            let pattern = n % divisor;

            let mut remaining = n;
            let mut matches = true;

            while remaining > 0 {
                if remaining % divisor != pattern {
                    matches = false;
                    break;
                }
                remaining /= divisor;
            }
            matches
        })
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|section| {
            parse_range(section)
                .filter(|&n| has_repeating_pattern(n))
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 1_227_775_554);
        assert_eq!(part_1(INPUT_TXT), 21_139_440_284);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 4_174_379_265);
        assert_eq!(part_2(INPUT_TXT), 38_731_915_928);
    }
}
