use std::{cmp::Ordering, collections::HashMap};
static INPUT_TXT: &str = include_str!("../../input/05.txt");

fn main() {
    println!("🌟 --- Day 5 Results --- 🌟");
    println!("📌 Part 1: {}", part_1(INPUT_TXT));
    println!("📌 Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (orders, updates) = input.trim().split_once("\n\n").expect("invalid input");

    let orders_map = orders
        .lines()
        .fold(HashMap::new(), |mut acc: HashMap<u32, Vec<u32>>, l| {
            let (k, v) = l.split_once('|').expect("invalid line");
            acc.entry(k.parse::<u32>().expect("invalid key"))
                .or_default()
                .push(v.parse::<u32>().expect("invalid value"));

            acc
        });

    let updates = updates
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().expect("invalid number in list"))
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();
    (orders_map, updates)
}

fn check_order(orders: &HashMap<u32, Vec<u32>>, line: &[u32]) -> bool {
    line.iter().enumerate().all(|(i, n)| {
        let Some(must_go_befores) = orders.get(n) else {
            return true;
        };
        line[0..i].iter().all(|c| !must_go_befores.contains(c))
    })
}

fn part_1(input: &str) -> u32 {
    let (orders, updates) = parse_input(input);
    updates
        .iter()
        .filter(|line| check_order(&orders, line))
        .map(|line| line.get(line.len() / 2).expect("line length is not odd"))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let (orders, mut updates) = parse_input(input);
    updates
        .iter_mut()
        .filter(|line| !check_order(&orders, line))
        .map(|line| {
            line.sort_by(|a, b| match orders.get(a) {
                Some(must_go_befores) if must_go_befores.contains(b) => Ordering::Less,
                _ => Ordering::Greater,
            });
            line.get(line.len() / 2).expect("line length is not odd")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 143);
        assert_eq!(part_1(INPUT_TXT), 7198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 123);
        assert_eq!(part_2(INPUT_TXT), 4230);
    }
}
