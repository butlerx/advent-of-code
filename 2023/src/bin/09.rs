static INPUT_TXT: &str = include_str!("../../input/09.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[must_use]
pub fn predict_next(seq: &[i32]) -> i32 {
    if seq.iter().any(|v| *v != 0) {
        let last = seq.len() - 1;
        let nums = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        seq[last] + predict_next(&nums)
    } else {
        0
    }
}

fn part_1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|l| {
            predict_next(
                &l.split_whitespace()
                    .map(|s| s.parse::<i32>().expect("Invalid number"))
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut nums = l
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Invalid number"))
                .collect::<Vec<_>>();
            nums.reverse();
            predict_next(&nums)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 114);
        assert_eq!(part_1(INPUT_TXT), 1_731_106_378);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2);
        assert_eq!(part_2(INPUT_TXT), 1087);
    }
}
