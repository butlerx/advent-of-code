static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn possible_wins(time: usize, dist: usize) -> usize {
    (1..time)
        .map(|held| held * (time - held))
        .filter(|possible_dist| possible_dist > &dist)
        .count()
}

fn part_1(input: &str) -> usize {
    let nums: Vec<_> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    nums[0]
        .iter()
        .zip(nums[1].iter())
        .map(|(time, dist)| possible_wins(*time, *dist))
        .product()
}

fn part_2(input: &str) -> usize {
    let nums: Vec<_> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect();
    possible_wins(nums[0], nums[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 288);
        assert_eq!(part_1(INPUT_TXT), 781_200);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 71503);
        assert_eq!(part_2(INPUT_TXT), 49_240_091);
    }
}
