#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{time_execution, time_execution_us, Grid, Point};
static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("🌟 --- Day 4 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_input(input: &str) -> Grid<bool> {
    let cells = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Grid::from(cells)
}

#[inline]
fn count_neighbors(grid: &Grid<bool>, point: Point) -> usize {
    point
        .neighbours_all_directions()
        .iter()
        .filter(|&neighbor| grid.in_bounds(*neighbor) && grid.get(*neighbor) == Some(true))
        .count()
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    grid.iter()
        .filter(|&(_, &value)| value)
        .filter(|(point, _)| count_neighbors(&grid, *point) < 4)
        .count()
}

fn part_2(input: &str) -> usize {
    let mut grid = parse_input(input);
    std::iter::from_fn(|| {
        let removable_points: Vec<Point> = grid
            .iter()
            .filter(|&(_, &value)| value)
            .filter(|(point, _)| count_neighbors(&grid, *point) < 4)
            .map(|(point, _)| point)
            .collect();
        if removable_points.is_empty() {
            None
        } else {
            for &point in &removable_points {
                grid.set(point, false);
            }
            Some(removable_points.len())
        }
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 13);
        assert_eq!(part_1(INPUT_TXT), 1491);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 43);
        assert_eq!(part_2(INPUT_TXT), 8722);
    }
}
