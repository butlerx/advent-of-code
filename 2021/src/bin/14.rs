use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/14.txt");
    println!("Part 1: {}", run(input, 10));
    println!("Part 2: {}", run(input, 40));
}

type PairCount = HashMap<(char, char), i64>;
type CharCount = HashMap<char, i64>;
type Rules = HashMap<(char, char), char>;

fn parse_input(input: &str) -> ((PairCount, CharCount), Rules) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let pair_count = template
        .chars()
        .tuple_windows()
        .fold(HashMap::new(), |mut counts, window| {
            *counts.entry(window).or_default() += 1;
            counts
        });
    let char_count = template.chars().fold(HashMap::new(), |mut counts, c| {
        *counts.entry(c).or_default() += 1;
        counts
    });
    let pairs = rules
        .lines()
        .map(|line| {
            let (l, n) = line.split_once(" -> ").unwrap();
            let mut letters = l.chars().tuple_windows();
            (letters.next().unwrap(), n.parse::<char>().unwrap())
        })
        .collect();
    ((pair_count, char_count), pairs)
}

fn run(input: &str, steps: usize) -> i64 {
    let (counts, rules) = parse_input(input);
    let (_, results) = (0..steps).fold(counts, |(pair_count, mut char_count), _| {
        let mut new_pair_count = pair_count.clone();

        for ((left, right), count) in pair_count {
            let middle = *rules.get(&(left, right)).unwrap();
            *new_pair_count.entry((left, right)).or_default() -= count;
            *new_pair_count.entry((left, middle)).or_default() += count;
            *new_pair_count.entry((middle, right)).or_default() += count;
            *char_count.entry(middle).or_default() += count;
        }

        (new_pair_count, char_count)
    });

    results.values().max().unwrap() - results.values().min().unwrap()
}

#[cfg(test)]
mod day_14_tests {
    use super::*;
    static INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, 10), 1588);
        assert_eq!(run(include_str!("../../input/14.txt"), 10), 2068);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, 40), 2_188_189_693_529);
        assert_eq!(
            run(include_str!("../../input/14.txt"), 40),
            2_158_894_777_814
        );
    }
}
