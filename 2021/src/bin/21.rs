use std::{cmp::max, collections::HashMap, mem::swap};

type Cache = HashMap<(usize, usize, usize, usize), (usize, usize)>;

fn main() {
    let input = include_str!("../../input/21.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let (mut player_1, mut player_2) = parse_input(input);
    let (mut rolls, mut scores, mut dice) = (0, (0, 0), 1);

    loop {
        dice = (0..3).fold(dice, |dice, _| {
            player_1 += dice;
            (dice + 1) % 100
        });
        rolls += 3;
        while player_1 > 10 {
            player_1 -= 10
        }
        scores.0 += player_1;
        if scores.0 >= 1000 {
            break rolls * scores.1;
        }
        swap(&mut player_1, &mut player_2);
        swap(&mut scores.0, &mut scores.1);
    }
}

fn part_2(input: &str) -> usize {
    let (player_1, player_2) = parse_input(input);
    let wins = play_game(&mut HashMap::new(), player_1, player_2, 0, 0);
    max(wins.0, wins.1)
}

fn play_game(
    cache: &mut Cache,
    player_1: usize,
    player_2: usize,
    score_1: usize,
    score_2: usize,
) -> (usize, usize) {
    if score_2 >= 21 {
        (0, 1)
    } else if let Some(&score) = cache.get(&(player_1, player_2, score_1, score_2)) {
        score
    } else {
        let score = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .iter()
            .fold((0, 0), |score, (roll, times)| {
                let n = player_1 + roll;
                let player_1 = if n > 10 { n - 10 } else { n };
                let wins = play_game(cache, player_2, player_1, score_2, score_1 + player_1);
                (score.0 + wins.1 * times, score.1 + wins.0 * times)
            });
        cache.insert((player_1, player_2, score_1, score_2), score);
        score
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let positions: Vec<usize> = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    (positions[0], positions[1])
}

#[cfg(test)]
mod day_21_tests {
    use super::*;
    static INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 739785);
        assert_eq!(part_1(include_str!("../../input/21.txt")), 893700);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 444356092776315);
        assert_eq!(part_2(include_str!("../../input/21.txt")), 568867175661958);
    }
}
