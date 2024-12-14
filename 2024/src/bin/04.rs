#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;
static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("🌟 --- Day 4 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

const DIRECTIONS: [Point; 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn parse_input(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] == 'X')
        .map(|(row, col)| {
            DIRECTIONS
                .iter()
                .filter(|(dx, dy)| {
                    WORD.iter().enumerate().all(|(i, item)| {
                        let i = i32::try_from(i).expect("number too large");
                        let new_row =
                            (i32::try_from(row).expect("number too large") + dx * i) as usize;
                        let new_col =
                            (i32::try_from(col).expect("number too large") + dy * i) as usize;
                        new_row < rows && new_col < cols && grid[new_row][new_col] == *item
                    })
                })
                .count()
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let grid = parse_input(input);

    (1..grid.len() - 1)
        .flat_map(|r| (1..grid[0].len() - 1).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] == 'A')
        .map(|(r, c)| {
            u32::from(
                [
                    (
                        (r - 1, c - 1, 'M', r + 1, c + 1, 'S'),
                        (r - 1, c + 1, 'M', r + 1, c - 1, 'S'),
                    ),
                    (
                        (r - 1, c - 1, 'M', r + 1, c + 1, 'S'),
                        (r + 1, c - 1, 'M', r - 1, c + 1, 'S'),
                    ),
                    (
                        (r + 1, c + 1, 'M', r - 1, c - 1, 'S'),
                        (r + 1, c - 1, 'M', r - 1, c + 1, 'S'),
                    ),
                    (
                        (r + 1, c + 1, 'M', r - 1, c - 1, 'S'),
                        (r - 1, c + 1, 'M', r + 1, c - 1, 'S'),
                    ),
                ]
                .iter()
                .any(|&(p1, p2)| check_pattern(&grid, p1) && check_pattern(&grid, p2)),
            )
        })
        .sum()
}

fn check_pattern(
    grid: &Grid,
    (r1, c1, m, r2, c2, s): (usize, usize, char, usize, usize, char),
) -> bool {
    grid[r1][c1] == m && grid[r2][c2] == s
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
