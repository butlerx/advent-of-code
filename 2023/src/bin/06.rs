static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

//  Original Solution bruteforced it
#[allow(dead_code)]
fn possible_wins_brute_force(time: usize, dist: usize) -> usize {
    (1..time)
        .map(|held| held * (time - held))
        .filter(|possible_dist| possible_dist > &dist)
        .count()
}

// Maths solution
fn possible_wins(time: f64, dist: f64) -> u64 {
    let d = (time * time - 4.0 * (dist + 1.0)).sqrt();
    let low = ((time - d) / 2.0).max(0.0).ceil() as u64;
    let high = ((time + d) / 2.0).floor() as u64;
    (high - low) + 1
}

fn part_1(input: &str) -> u64 {
    let nums: Vec<_> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    nums[0]
        .iter()
        .zip(nums[1].iter())
        .map(|(time, dist)| possible_wins(*time, *dist))
        .product()
}

fn part_2(input: &str) -> u64 {
    let nums: Vec<_> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<f64>()
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
