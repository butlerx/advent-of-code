use itertools::Itertools;

static COMMA: &str = ",";

fn main() {
    let input = include_str!("../../input/07.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn run(input: &str, exponential: bool) -> usize {
    let mut nums: Vec<i64> = input
        .split(COMMA)
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect();
    nums.sort_unstable();
    let counts: Vec<(i64, usize)> = nums
        .iter()
        .dedup_with_count()
        .map(|(a, b)| (*b, a))
        .collect();
    (counts.first().unwrap().0..=counts.last().unwrap().0)
        .map(|candidate| {
            counts
                .iter()
                .map(|(pos, count)| cost((pos - candidate).abs() as usize, exponential) * count)
                .sum()
        })
        .min()
        .unwrap()
}

fn cost(num: usize, exponential: bool) -> usize {
    if exponential {
        (num * (num + 1)) / 2
    } else {
        num
    }
}

#[cfg(test)]
mod day_7_tests {
    use super::*;
    static INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 37);
        assert_eq!(run(include_str!("../../input/07.txt"), false), 325528);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 168);
        assert_eq!(run(include_str!("../../input/07.txt"), true), 85015836);
    }
}
