use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (_, cards) = l.trim().split_once(':').unwrap();
            let (winning_cards, got_cards) = cards.split_once('|').unwrap();
            let winning: HashSet<u32> = winning_cards
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let got: HashSet<u32> = got_cards
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            winning.intersection(&got).count()
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|p| (1..=p).fold(0, |acc, n| if n == 1 { n } else { acc * 2 }))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let games = parse_input(input);
    let mut card_amounts = vec![1u32; games.len()];
    for (i, matches) in games.into_iter().enumerate() {
        for index in (i + 1)..=(i + matches) {
            card_amounts[index] += card_amounts[i];
        }
    }
    card_amounts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13);
        assert_eq!(part_1(INPUT_TXT), 19135);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 30);
        assert_eq!(part_2(INPUT_TXT), 5_704_953);
    }
}
