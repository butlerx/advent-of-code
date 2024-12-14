#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;

static INPUT_TXT: &str = include_str!("../../input/07.txt");

fn main() {
    println!("🌟 --- Day 7 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn calibration(
    nums: &Vec<usize>,
    idx: usize,
    current: usize,
    results: &mut Vec<usize>,
    ans: usize,
    part_2: bool,
) -> bool {
    if current > ans {
        return false;
    }
    if idx >= nums.len() {
        results.push(current);
        return current == ans;
    }

    if part_2 {
        let mut combined = current;
        let mut temp = nums[idx];
        while temp > 0 {
            combined *= 10;
            temp /= 10;
        }
        combined += nums[idx];
        if calibration(nums, idx + 1, combined, results, ans, part_2) {
            return true;
        }
    }
    calibration(nums, idx + 1, current * nums[idx], results, ans, part_2)
        || calibration(nums, idx + 1, current + nums[idx], results, ans, part_2)
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, Vec<usize>)> + '_ {
    input.trim().lines().map(|l| {
        let (a, nums_str) = l.split_once(": ").expect("no : found");
        let ans = a.parse::<usize>().expect("not a number");
        let nums = nums_str
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("not a number"))
            .collect::<Vec<usize>>();
        (ans, nums)
    })
}

fn part_1(input: &str) -> usize {
    parse_input(input)
        .filter(|(ans, nums)| {
            let mut results = Vec::new();
            calibration(nums, 1, nums[0], &mut results, *ans, false)
        })
        .map(|(ans, _)| ans)
        .sum()
}

fn part_2(input: &str) -> usize {
    parse_input(input)
        .filter(|(ans, nums)| {
            let mut results = Vec::new();
            calibration(nums, 1, nums[0], &mut results, *ans, true)
        })
        .map(|(ans, _)| ans)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 3749);
        assert_eq!(part_1(INPUT_TXT), 303_876_485_655);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 11387);
        assert_eq!(part_2(INPUT_TXT), 146_111_650_210_682);
    }
}
