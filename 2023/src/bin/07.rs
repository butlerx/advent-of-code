#![warn(clippy::pedantic, clippy::perf)]
use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/07.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<HashMap<char, usize>> for HandType {
    fn from(cards: HashMap<char, usize>) -> Self {
        let mut counts = cards.values().collect::<Vec<_>>();
        counts.sort();
        match counts.as_slice() {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

impl HandType {
    fn new(cards: &str, jacks_wild: bool) -> Self {
        let mut freq: HashMap<char, usize> = HashMap::new();
        if jacks_wild {
            freq.insert('J', 0);
        }

        let mut max_char = '0';
        let mut max_count = 0;

        for c in cards.chars() {
            let count = freq.entry(c).or_insert(0);
            *count += 1;
            if *count >= max_count && (c != 'J' || !jacks_wild) {
                max_char = c;
                max_count = *count;
            }
        }

        if jacks_wild {
            let add = *freq.get(&'J').unwrap();
            let count = freq.entry(max_char).or_insert(0);
            *count += add;
            freq.remove(&'J');
        }

        Self::from(freq)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    kind: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.kind, &other.kind) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind)
            | (HandType::FourOfAKind, HandType::FourOfAKind)
            | (HandType::FullHouse, HandType::FullHouse)
            | (HandType::ThreeOfAKind, HandType::ThreeOfAKind)
            | (HandType::TwoPair, HandType::TwoPair)
            | (HandType::OnePair, HandType::OnePair)
            | (HandType::HighCard, HandType::HighCard) => self.cards.cmp(&other.cards),
            (HandType::FiveOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandType::FiveOfAKind) => std::cmp::Ordering::Less,
            (HandType::FourOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandType::FourOfAKind) => std::cmp::Ordering::Less,
            (HandType::FullHouse, _) => std::cmp::Ordering::Greater,
            (_, HandType::FullHouse) => std::cmp::Ordering::Less,
            (HandType::ThreeOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandType::ThreeOfAKind) => std::cmp::Ordering::Less,
            (HandType::TwoPair, _) => std::cmp::Ordering::Greater,
            (_, HandType::TwoPair) => std::cmp::Ordering::Less,
            (HandType::OnePair, _) => std::cmp::Ordering::Greater,
            (_, HandType::OnePair) => std::cmp::Ordering::Less,
        }
    }
}

impl Hand {
    fn new(line: &str, jacks_wild: bool) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let hand_type = HandType::new(cards, jacks_wild);
        let cards = cards
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if jacks_wild {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => c.to_digit(10).expect("Not a valid card"),
            })
            .collect();

        Self {
            cards,
            bid,
            kind: hand_type,
        }
    }
}

fn play_cards(input: &str, jacks_wild: bool) -> u32 {
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| Hand::new(line, jacks_wild))
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| u32::try_from(i + 1).expect("unable to convert usize to u32") * hand.bid)
        .sum()
}

fn part_1(input: &str) -> u32 {
    play_cards(input, false)
}

fn part_2(input: &str) -> u32 {
    play_cards(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    static COMMUNITY_CARDS: &str = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 6440);
        assert_eq!(part_1(COMMUNITY_CARDS), 1343);
        assert_eq!(part_1(INPUT_TXT), 250_946_742);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 5905);
        assert_eq!(part_1(COMMUNITY_CARDS), 1343);
        assert_eq!(part_2(INPUT_TXT), 251_824_095);
    }
}
