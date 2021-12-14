use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../../input/22.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn find_score(deck: VecDeque<i64>) -> i64 {
    deck.iter().enumerate().fold(0, |total, (pos, card)| {
        total + (*card * (deck.len() - pos) as i64)
    })
}

fn recursive_combat(
    mut player_1: VecDeque<i64>,
    mut player_2: VecDeque<i64>,
    score: bool,
) -> (usize, i64) {
    let mut previous_round = HashSet::new();
    loop {
        if !previous_round.insert((player_1.clone(), player_2.clone())) || player_2.is_empty() {
            break (1, if score { find_score(player_1) } else { 0 });
        } else if player_1.is_empty() {
            break (2, find_score(player_2));
        }

        let (c_1, c_2) = (player_1.pop_front().unwrap(), player_2.pop_front().unwrap());
        if c_1 <= player_1.len() as i64 && c_2 <= player_2.len() as i64 {
            match recursive_combat(
                player_1.iter().take(c_1 as usize).cloned().collect(),
                player_2.iter().take(c_2 as usize).cloned().collect(),
                false,
            ) {
                (1, _) => player_1.extend([c_1, c_2].iter()),
                (2, _) => player_2.extend([c_2, c_1].iter()),
                _ => unreachable!(),
            }
        } else if c_1 > c_2 {
            player_1.extend([c_1, c_2].iter());
        } else {
            player_2.extend([c_2, c_1].iter());
        }
    }
}

fn find_winner(mut player_1: VecDeque<i64>, mut player_2: VecDeque<i64>) -> i64 {
    loop {
        if player_1.is_empty() {
            break find_score(player_2);
        } else if player_2.is_empty() {
            break find_score(player_1);
        }
        let (c_1, c_2) = (player_1.pop_front().unwrap(), player_2.pop_front().unwrap());
        if c_1 > c_2 {
            player_1.extend([c_1, c_2].iter());
        } else {
            player_2.extend([c_2, c_1].iter());
        }
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    let (player_1, player_2): (VecDeque<i64>, VecDeque<i64>) = input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .skip(1)
                .map(|line| line.parse::<i64>().unwrap())
                .collect()
        })
        .collect_tuple()
        .unwrap();
    if part_two {
        let (_, score) = recursive_combat(player_1, player_2, true);
        score
    } else {
        find_winner(player_1, player_2)
    }
}

#[cfg(test)]
mod day_22_tests {
    use super::*;
    static INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 306);
        assert_eq!(run(include_str!("../../input/22.txt"), false), 32366);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 291);
        assert_eq!(run(include_str!("../../input/22.txt"), true), 30891);
    }
}
