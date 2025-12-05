#![warn(clippy::pedantic, clippy::perf)]
use aoc_shared::{time_execution, Grid, Point};

static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("🌟 --- Day 4 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

const DIRECTIONS: [Point; 8] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: -1, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: 1 },
];

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn parse_input(input: &str) -> Grid<char> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);

    grid.iter()
        .filter(|(_, &c)| c == 'X')
        .map(|(pos, _)| {
            DIRECTIONS
                .iter()
                .filter(|&&delta| {
                    WORD.iter().enumerate().all(|(i, item)| {
                        let i = i64::try_from(i).expect("number too large");
                        let next_pos = pos + Point::new(delta.x * i, delta.y * i);
                        grid.get(next_pos) == Some(*item)
                    })
                })
                .count()
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let grid = parse_input(input);

    grid.iter()
        .filter(|(_, &c)| c == 'A')
        .filter(|(pos, _)| {
            pos.x > 0
                && pos.y > 0
                && pos.x < i64::try_from(grid.height - 1).expect("number too large")
                && pos.y < i64::try_from(grid.width - 1).expect("number too large")
        })
        .map(|(pos, _)| {
            u32::from(
                [
                    (
                        (Point::new(-1, -1), 'M', Point::new(1, 1), 'S'),
                        (Point::new(-1, 1), 'M', Point::new(1, -1), 'S'),
                    ),
                    (
                        (Point::new(-1, -1), 'M', Point::new(1, 1), 'S'),
                        (Point::new(1, -1), 'M', Point::new(-1, 1), 'S'),
                    ),
                    (
                        (Point::new(1, 1), 'M', Point::new(-1, -1), 'S'),
                        (Point::new(1, -1), 'M', Point::new(-1, 1), 'S'),
                    ),
                    (
                        (Point::new(1, 1), 'M', Point::new(-1, -1), 'S'),
                        (Point::new(-1, 1), 'M', Point::new(1, -1), 'S'),
                    ),
                ]
                .iter()
                .any(|&(p1, p2)| check_pattern(&grid, pos, p1) && check_pattern(&grid, pos, p2)),
            )
        })
        .sum()
}

fn check_pattern(
    grid: &Grid<char>,
    center: Point,
    (offset1, m, offset2, s): (Point, char, Point, char),
) -> bool {
    let pos1 = center + offset1;
    let pos2 = center + offset2;
    grid.get(pos1) == Some(m) && grid.get(pos2) == Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 18);
        assert_eq!(part_1(INPUT_TXT), 2454);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 9);
        assert_eq!(part_2(INPUT_TXT), 1858);
    }
}
