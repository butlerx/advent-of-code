use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/09.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn find_not_sum(nums: &[i64], preable: usize) -> i64 {
    nums[(preable..nums.len())
        .position(|i| {
            !&nums[i - preable..i]
                .iter()
                .permutations(2)
                .map(|comb| comb.iter().copied().sum())
                .any(|x: i64| x == nums[i])
        })
        .unwrap()
        + preable]
}

fn find_range(nums: &[i64], target: i64) -> (i64, i64) {
    let (mut sum, mut low, mut high) = (0, 0, 0);
    loop {
        sum = if low >= nums.len() {
            break (0, 0);
        } else if sum == target && low + 1 < high {
            let min = nums[low..high].iter().min().unwrap();
            let max = nums[low..high].iter().max().unwrap();
            break (*min, *max);
        } else if sum < target && high < nums.len() {
            high += 1;
            sum + nums[high - 1]
        } else {
            low += 1;
            sum - nums[low - 1]
        }
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    let nums: Vec<i64> = input
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect();
    let num = find_not_sum(&nums, 25);
    if part_two {
        let (low, high) = find_range(&nums, num);
        low + high
    } else {
        num
    }
}

#[cfg(test)]
mod day_9_tests {
    use super::*;
    static INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_find_range() {
        let nums: Vec<i64> = INPUT
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect();
        assert_eq!(find_range(&nums, find_not_sum(&nums, 5)), (15, 47));
    }

    #[test]
    fn test_find_not_sum() {
        let nums: Vec<i64> = INPUT
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect();
        assert_eq!(find_not_sum(&nums, 5), 127);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(run(include_str!("../../input/09.txt"), false), 69_316_178);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(include_str!("../../input/09.txt"), true), 9_351_526);
    }
}
