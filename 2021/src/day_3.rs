pub fn run(input: &str, part_two: bool) -> i64 {
    let bit_length = input.lines().next().unwrap().len();
    let nums: Vec<i64> = input
        .lines()
        .map(|num| i64::from_str_radix(num, 2).unwrap())
        .collect();
    if part_two {
        let o2_rating = get_rating(&nums, 1, bit_length);
        let c02_rating = get_rating(&nums, 0, bit_length);
        o2_rating * c02_rating
    } else {
        let gamma_rate: i64 = (0..bit_length)
            .map(|i| most_common_bit(&nums, i) << i)
            .sum();
        let epsilon_rate: i64 = !gamma_rate & ((1 << bit_length) - 1);
        gamma_rate * epsilon_rate
    }
}

fn most_common_bit(nums: &[i64], bit: usize) -> i64 {
    let mut c = [0, 0];
    for &x in nums {
        c[(x as usize >> bit) & 1] += 1
    }
    (c[1] >= c[0]) as i64
}

fn get_rating(nums: &[i64], most_common: i64, bit_length: usize) -> i64 {
    let mut nums = nums.to_vec();
    for i in (0..bit_length).rev() {
        let retain_bit = most_common_bit(&nums, i) ^ most_common;
        nums.retain(|x| (x >> i) & 1 == retain_bit);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 198);
        assert!(run(include_str!("../input/day_3.txt"), false) == 3847100);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 230);
        assert!(run(include_str!("../input/day_3.txt"), true) == 4105235);
    }
}
