use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/01.txt");
    println!("Part 1: {}", run(input, 2));
    println!("Part 2: {}", run(input, 3));
}

fn parse_expense(raw_expense: &str) -> Vec<i64> {
    raw_expense
        .split_whitespace()
        .map(|line| line.trim().parse::<i64>().expect("parse error"))
        .collect()
}

fn find_2020_multiple(expenses: Vec<i64>, depth: usize) -> i64 {
    let goal = 2020;
    if depth == 0 {
        0
    } else {
        expenses
            .into_iter()
            .combinations(depth)
            .filter(|combination| combination.iter().sum::<i64>() == goal)
            .map(|combination| combination.iter().product::<i64>())
            .next()
            .unwrap()
    }
}

fn run(input: &str, depth: usize) -> i64 {
    let expenses = parse_expense(input);
    find_2020_multiple(expenses, depth)
}

#[cfg(test)]
mod day_1_tests {
    use super::*;

    #[test]
    fn test_parse_expense() {
        let results = vec![1721, 979, 366, 299, 675, 1456];
        assert!(parse_expense("1721\n979\n366\n299\n675\n1456") == results);
    }

    #[test]
    fn test_find_2020_multiple() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_2020_multiple(input, 2);
        assert!(514_579 == result);
    }

    #[test]
    fn test_find_2020_multple_depth() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_2020_multiple(input, 3);
        assert!(241_861_950 == result);
    }
    #[test]
    fn test_part_1() {
        let results = run(include_str!("../../input/01.txt"), 2);
        println!("{results}");
        assert!(results == 864_864);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../../input/01.txt"), 3);
        println!("{results}");
        assert!(results == 281_473_080);
    }
}
