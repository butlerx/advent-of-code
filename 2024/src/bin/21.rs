#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::{time_execution, Grid, Point};
use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/21.txt");

fn main() {
    println!("🌟 --- Day 21 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Clone, Debug)]
struct State {
    pad: Grid<char>,
    moves: Vec<Point>,
    move_to_dir: HashMap<Point, char>,
}

impl Default for State {
    fn default() -> Self {
        let pad = Grid::from(vec![
            vec!['7', '4', '1', ' '],
            vec!['8', '5', '2', '0'],
            vec!['9', '6', '3', 'A'],
        ]);
        State::from(pad)
    }
}

impl From<Grid<char>> for State {
    fn from(pad: Grid<char>) -> Self {
        let moves = vec![
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ];
        let move_to_dir = moves.iter().copied().zip("^v<>".chars()).collect();
        State {
            pad,
            moves,
            move_to_dir,
        }
    }
}

impl State {
    fn get_all_encodings(&self, code: &str, start: Point) -> Vec<String> {
        let bounds = Point::new(
            i64::try_from(self.pad.width).unwrap() - 1,
            i64::try_from(self.pad.height).unwrap() - 1,
        );

        get_permutations(&self.moves)
            .into_iter()
            .map(|perm| self.encode_path(code, start, bounds, &perm))
            .collect()
    }

    fn encode_path(&self, code: &str, start: Point, bounds: Point, moves: &[Point]) -> String {
        let mut current = start;
        code.chars()
            .filter_map(|x| {
                self.pad.iter().find_map(|(pos, &c)| {
                    if c == x {
                        Some(self.find_path(&mut current, pos, bounds, moves))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }

    fn find_path(
        &self,
        current: &mut Point,
        target: Point,
        bounds: Point,
        moves: &[Point],
    ) -> Vec<char> {
        let mut path = Vec::new();
        loop {
            if moves
                .iter()
                .find_map(|&delta| {
                    let next = *current + delta;
                    if next.in_bounds(bounds.x, bounds.y)
                        && self.pad.get(next).map_or(false, |c| c != ' ')
                        && current.manhattan_distance(target) > next.manhattan_distance(target)
                    {
                        path.push(*self.move_to_dir.get(&delta).unwrap());
                        *current = next;
                        Some(true)
                    } else {
                        None
                    }
                })
                .is_none()
            {
                path.push('A');
                break;
            }
        }
        path
    }
}

fn get_permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    match items.len() {
        0 => vec![vec![]],
        n => (0..n)
            .flat_map(|i| {
                let mut items = items.to_vec();
                let item = items.remove(i);
                get_permutations(&items).into_iter().map(move |mut perm| {
                    perm.insert(0, item.clone());
                    perm
                })
            })
            .collect(),
    }
}

fn dp(code: &str, depth: i64, cache: &mut HashMap<(String, i64), i64>) -> i64 {
    let cache_key = (code.to_string(), depth);

    if let Some(&result) = cache.get(&cache_key) {
        return result;
    }

    let result = if depth == 0 {
        i64::try_from(code.len()).expect("number too large")
    } else {
        let state = State::from(Grid::from(vec![
            vec![' ', '<'],
            vec!['^', 'v'],
            vec!['A', '>'],
        ]));

        code[..code.len() - 1]
            .split('A')
            .map(|chunk| {
                let chunk = format!("{chunk}A");
                state
                    .get_all_encodings(&chunk, Point::new(0, 2))
                    .iter()
                    .map(|code2| dp(code2, depth - 1, cache))
                    .min()
                    .unwrap_or(0)
            })
            .sum()
    };

    cache.insert(cache_key, result);
    result
}

fn solve(input: &str, depth: i64) -> i64 {
    let state = State::default();
    let mut cache = HashMap::new();

    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let value = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<i64>()
                .expect("Invalid number");

            let min_encoding = state
                .get_all_encodings(line, Point::new(3, 2))
                .iter()
                .map(|code| dp(code, depth, &mut cache))
                .min()
                .unwrap_or(0);

            min_encoding * value
        })
        .sum()
}

fn part_1(input: &str) -> i64 {
    solve(input, 2)
}

fn part_2(input: &str) -> i64 {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 126_384);
        assert_eq!(part_1(INPUT_TXT), 238_078);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 154_115_708_116_294);
        assert_eq!(part_2(INPUT_TXT), 293_919_502_998_014);
    }
}
