static INPUT_TXT: &str = include_str!("../../input/02.txt");

fn main() {
    println!("🌟 --- Day 2 Results --- 🌟");
    println!("📌 Part 1: {}", part_1(INPUT_TXT));
    println!("📌 Part 2: {}", part_2(INPUT_TXT));
}

fn is_safe(arr: &[i32]) -> bool {
    let pairs: Vec<_> = arr.windows(2).map(|w| (w[0], w[1])).collect();

    (pairs.iter().all(|(a, b)| a < b) || pairs.iter().all(|(a, b)| a > b))
        && pairs.iter().all(|(a, b)| {
            let c = (a - b).abs();
            (1..=3).contains(&c)
        })
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().expect("invalid number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> usize {
    let nums = parse_input(input);
    nums.iter().filter(|x| is_safe(x)).count()
}

fn part_2(input: &str) -> usize {
    let nums = parse_input(input);
    nums.into_iter()
        .filter(|line| {
            line.iter().enumerate().any(|(n, _)| {
                let mut nums = line.clone();
                nums.remove(n);
                is_safe(&nums)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 2);
        assert_eq!(part_1(INPUT_TXT), 421);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4);
        assert_eq!(part_2(INPUT_TXT), 476);
    }
}
