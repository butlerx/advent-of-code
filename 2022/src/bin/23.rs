use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/23.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Point = (i64, i64);
type Elves = HashSet<Point>;

fn parse_input(input: &str) -> Elves {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect()
}

fn bounds(elves: &Elves) -> (Point, Point) {
    (
        (
            elves.iter().map(|e| e.0).min().unwrap(),
            elves.iter().map(|e| e.0).max().unwrap(),
        ),
        (
            elves.iter().map(|e| e.1).min().unwrap(),
            elves.iter().map(|e| e.1).max().unwrap(),
        ),
    )
}

fn move_elves(round: usize, elves: &Elves) -> Option<Elves> {
    let proposals = elves.iter().fold(HashMap::new(), |mut prop, e| {
        if let Some(p) = check_and_propose(round, *e, elves) {
            *prop.entry(p).or_insert(0) += 1;
        }
        prop
    });
    if !proposals.is_empty() {
        Some(
            elves
                .iter()
                .map(|&e| match check_and_propose(round, e, elves) {
                    Some(p) if *proposals.get(&p).unwrap() == 1 => p,
                    _ => e,
                })
                .collect(),
        )
    } else {
        None
    }
}

fn check_and_propose(round: usize, pos: Point, elves: &Elves) -> Option<Point> {
    if (-1..=1).any(|y| {
        (-1..=1).any(|x| !(x == 0 && y == 0) && elves.get(&(pos.0 + x, pos.1 + y)).is_some())
    }) {
        (round..=round + 3).find_map(|r| {
            let dir = r % 4;
            let diff = if dir == 0 || dir == 2 { -1 } else { 1 };
            if dir < 2 && !(-1..=1).any(|x| elves.get(&(pos.0 + x, pos.1 + diff)).is_some()) {
                Some((pos.0, pos.1 + diff))
            } else if dir >= 2 && !(-1..=1).any(|y| elves.get(&(pos.0 + diff, pos.1 + y)).is_some())
            {
                Some((pos.0 + diff, pos.1))
            } else {
                None
            }
        })
    } else {
        None
    }
}

fn part_1(input: &str) -> i64 {
    let mut elves = parse_input(input);
    for r in 0..10 {
        if let Some(nelves) = move_elves(r, &elves) {
            elves = nelves;
        }
    }
    let ((x_min, x_max), (y_min, y_max)) = bounds(&elves);
    (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as i64
}

fn part_2(input: &str) -> i64 {
    let mut elves = parse_input(input);
    let mut r = 0;
    loop {
        if let Some(nelves) = move_elves(r, &elves) {
            elves = nelves;
        } else {
            break r as i64 + 1;
        }
        r += 1;
    }
}

#[cfg(test)]
mod day_23_tests {
    use super::*;
    static INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 110);
        assert_eq!(part_1(INPUT_TXT), 3812);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 20);
        assert_eq!(part_2(INPUT_TXT), 1003);
    }
}
