static INPUT_TXT: &str = include_str!("../../input/20.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn mix(nums: &[i64], iterations: usize) -> Vec<usize> {
    (0..iterations).fold((0..nums.len()).collect(), |res, _| {
        nums.iter().enumerate().fold(res, |mut res, (i, &x)| {
            let pos = res.iter().position(|&y| y == i).unwrap();
            res.remove(pos);
            res.insert((pos as i64 + x).rem_euclid(res.len() as i64) as usize, i);
            res
        })
    })
}

fn decrypt(nums: Vec<i64>, iterations: usize) -> i64 {
    let orig_zero_pos = nums.iter().position(|&i| i == 0).unwrap();
    let ans = mix(&nums, iterations);
    let zero_pos = ans.iter().position(|&i| i == orig_zero_pos).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[ans[(zero_pos + i) % ans.len()]])
        .sum()
}

fn part_1(input: &str) -> i64 {
    let nums = input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    decrypt(nums, 1)
}

fn part_2(input: &str) -> i64 {
    let nums = input
        .trim()
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811_589_153)
        .collect::<Vec<_>>();
    decrypt(nums, 10)
}

#[cfg(test)]
mod day_20_tests {
    use super::*;
    static INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 3);
        assert_eq!(part_1(INPUT_TXT), 4267);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1_623_178_306);
        assert_eq!(part_2(INPUT_TXT), 6_871_725_358_451);
    }
}
