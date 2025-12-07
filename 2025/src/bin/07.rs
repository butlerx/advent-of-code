#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{time_execution, time_execution_us, Grid, Point};
use std::collections::HashMap;
static INPUT_TXT: &str = include_str!("../../input/07.txt");

fn main() {
    println!("🌟 --- Day 7 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_input(input: &str) -> Grid<char> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start_pos(grid: &Grid<char>) -> Point {
    grid.iter()
        .find_map(|(pos, &c)| (c == 'S').then_some(pos))
        .expect("No start pos found")
}

fn part_1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let start_pos = find_start_pos(&grid);

    let height = grid.height.try_into().expect("number too large");
    let width = grid.width.try_into().expect("number too large");
    (start_pos.y + 1..=height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .fold(0, |split, (x, y)| {
            if let Some('|' | 'S') = grid.get((x, y - 1).into()) {
                let current_pos = (x, y).into();
                match grid.get(current_pos) {
                    Some('^') => {
                        grid.set((x - 1, y).into(), '|');
                        grid.set((x + 1, y).into(), '|');
                        split + 1
                    }
                    Some('.') => {
                        grid.set(current_pos, '|');
                        split
                    }
                    _ => split,
                }
            } else {
                split
            }
        })
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let start_pos = find_start_pos(&grid);
    let height = grid.height.try_into().expect("number too large");
    let mut memo = HashMap::new();
    count_paths_memo(&grid, start_pos, height, 0, &mut memo)
}

fn count_paths_memo(
    grid: &Grid<char>,
    pos: Point,
    height: i64,
    x_offset: i64,
    memo: &mut HashMap<(Point, i64), usize>,
) -> usize {
    let next_y = pos.y + 1;

    if next_y >= height {
        return 1;
    }

    let key = (pos, x_offset);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let current_x = pos.x + x_offset;
    let next_pos = (current_x, next_y).into();
    let result = match grid.get(next_pos) {
        Some('^') => {
            let left = count_paths_memo(grid, pos, height, x_offset - 1, memo);
            let right = count_paths_memo(grid, pos, height, x_offset + 1, memo);
            left + right
        }
        Some('.') => count_paths_memo(grid, next_pos, height, 0, memo),
        _ => 0,
    };

    memo.insert(key, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 21);
        assert_eq!(part_1(INPUT_TXT), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 40);
        assert_eq!(part_2(INPUT_TXT), 108_924_003_331_749);
    }
}
