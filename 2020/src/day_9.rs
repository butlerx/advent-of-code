use itertools::Itertools;

fn find_not_sum(nums: &Vec<i64>, preable: usize) -> i64 {
    nums[(preable..nums.len())
        .position(|i| {
            !&nums[i - preable..i]
                .iter()
                .permutations(2)
                .map(|comb| comb.iter().map(|j| *j).sum())
                .collect::<Vec<i64>>()
                .contains(&nums[i])
        })
        .unwrap()
        + preable]
}

fn find_range(nums: &Vec<i64>, target: i64) -> (i64, i64) {
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

pub fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
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
mod tests {
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
        let nums = INPUT
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect();
        assert!(find_range(&nums, find_not_sum(&nums, 5)) == (15, 47))
    }

    #[test]
    fn test_find_not_sum() {
        assert!(
            find_not_sum(
                &INPUT
                    .lines()
                    .map(|line| line.parse::<i64>().unwrap())
                    .collect(),
                5
            ) == 127
        );
    }

    #[test]
    fn test_part_1() {
        let results = run(include_str!("../input/day_9.txt"), false);
        println!("{}", results);
        assert!(results == 69316178);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../input/day_9.txt"), true);
        println!("{}", results);
        assert!(results == 9351526);
    }
}
