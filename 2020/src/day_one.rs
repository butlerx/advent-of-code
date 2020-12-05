use itertools::Itertools;
use std::io::Error;

fn parse_expense(raw_expense: &str) -> Vec<i64> {
    raw_expense
        .split_whitespace()
        .map(|line| line.trim().parse::<i64>().expect("parse error"))
        .collect()
}

fn find_2020_multiple(expenses: Vec<i64>, depth: usize) -> Result<i64, Error> {
    let goal = 2020;
    if depth == 0 {
        Ok(0)
    } else {
        Ok(expenses
            .into_iter()
            .combinations(depth)
            .filter(|combination| combination.iter().sum::<i64>() == goal)
            .map(|combination| combination.iter().product::<i64>())
            .next()
            .unwrap())
    }
}

pub fn run(input: &str, depth: usize) -> Result<i64, Error> {
    let expenses = parse_expense(input);
    find_2020_multiple(expenses, depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expense() {
        let results = vec![1721, 979, 366, 299, 675, 1456];
        assert!(parse_expense("1721\n979\n366\n299\n675\n1456") == results);
    }

    #[test]
    fn test_find_2020_multiple() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_2020_multiple(input, 2).unwrap();
        assert!(514579 == result);
    }

    #[test]
    fn test_find_2020_multple_depth() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_2020_multiple(input, 3).unwrap();
        assert!(241861950 == result);
    }
    #[test]
    fn test_part_1() {
        let results = run(include_str!("../input/day_one.txt"), 2).unwrap();
        println!("{}", results);
        assert!(results == 864864);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../input/day_one.txt"), 3).unwrap();
        println!("{}", results);
        assert!(results == 281473080);
    }
}
