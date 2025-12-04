#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_shared::{time_execution, Grid, Point};
use std::collections::VecDeque;

static INPUT_TXT: &str = include_str!("../../input/20.txt");

fn main() {
    println!("🌟 --- Day 20 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

struct SearchConfig {
    max_distance: i64,
    distance_check: fn(i64) -> bool,
    distance_add: fn(i64) -> i64,
}

fn count_paths(
    input: &Grid<char>,
    config: &SearchConfig,
    target: i64,
    exact_distance: bool,
) -> i64 {
    let start_position = input.find_position('S').expect("No start Position");
    let end_position = input.find_position('E').expect("No end Position");
    let start_distance = bfs(input, start_position);
    let end_distance = bfs(input, end_position);
    let orig_distance = end_distance.get(start_position).unwrap();

    input
        .iter()
        .filter(|&(p, c)| *c != '#' && start_distance.get(p).unwrap() != i64::MAX)
        .map(|(mid_point, _)| mid_point)
        .map(|mid_point| {
            i64::try_from(
                (-config.max_distance..=config.max_distance)
                    .flat_map(|r_offset| {
                        (-config.max_distance..=config.max_distance)
                            .map(move |c_offset| Point::new(c_offset, r_offset))
                    })
                    .map(|offset| {
                        let end_point = mid_point + offset;
                        let manhattan = offset.manhattan_distance(Point::new(0, 0));
                        (end_point, manhattan)
                    })
                    .filter(|&(_, manhattan)| (config.distance_check)(manhattan))
                    .filter(|&(e, _)| input.in_bounds(e))
                    .filter(|&(e, _)| input.get(e) != Some('#'))
                    .filter(|&(e, _)| end_distance.get(e).unwrap() != i64::MAX)
                    .filter_map(|(end_point, manhattan)| {
                        let start_dist = start_distance.get(mid_point)?;
                        let end_dist = end_distance.get(end_point)?;
                        Some(start_dist + end_dist + (config.distance_add)(manhattan))
                    })
                    .filter(|&new_distance| {
                        if exact_distance {
                            new_distance + target == orig_distance
                        } else {
                            new_distance + target <= orig_distance
                        }
                    })
                    .count(),
            )
            .expect("number too large")
        })
        .sum()
}

fn bfs(input: &Grid<char>, start: Point) -> Grid<i64> {
    let dimensions = Point::from((input.width, input.height));
    let mut distance = Grid::new(dimensions, i64::MAX);
    let mut queue = VecDeque::from([(start, 0i64)]);

    while let Some((position, cost)) = queue.pop_front() {
        if distance.get(position).unwrap() != i64::MAX {
            continue;
        }

        distance.set(position, cost);
        let next_steps = position
            .neighbours()
            .into_iter()
            .filter(|&adj| input.in_bounds(adj))
            .filter(|&adj| distance.get(adj) == Some(i64::MAX))
            .filter(|&adj| input.get(adj) != Some('#'))
            .map(|adj| (adj, cost + 1));
        queue.extend(next_steps);
    }
    distance
}

fn part_1(input: &str) -> i64 {
    let grid = Grid::from(
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    );
    let config = SearchConfig {
        max_distance: 2,
        distance_check: |d| d == 2,
        distance_add: |_| 2,
    };
    count_paths(&grid, &config, 100, false)
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::from(
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    );
    let config = SearchConfig {
        max_distance: 20,
        distance_check: |d| d <= 20,
        distance_add: |d| d,
    };
    count_paths(&grid, &config, 100, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_TXT), 1452);
    }

    #[test]
    fn test_part2_specific_savings() {
        let grid = Grid::from(
            INPUT
                .lines()
                .map(|line| line.chars().collect())
                .collect::<Vec<_>>(),
        );

        let config = SearchConfig {
            max_distance: 20,
            distance_check: |d| d <= 20,
            distance_add: |d| d,
        };

        let savings = vec![
            (50, 32), // 32 cheats saving 50 picoseconds
            (52, 31), // 31 cheats saving 52 picoseconds
            (54, 29), // 29 cheats saving 54 picoseconds
            (56, 39), // 39 cheats saving 56 picoseconds
            (58, 25), // 25 cheats saving 58 picoseconds
            (60, 23), // 23 cheats saving 60 picoseconds
            (62, 20), // 20 cheats saving 62 picoseconds
            (64, 19), // 19 cheats saving 64 picoseconds
            (66, 12), // 12 cheats saving 66 picoseconds
            (68, 14), // 14 cheats saving 68 picoseconds
            (70, 12), // 12 cheats saving 70 picoseconds
            (72, 22), // 22 cheats saving 72 picoseconds
            (74, 4),  // 4 cheats saving 74 picoseconds
            (76, 3),  // 3 cheats saving 76 picoseconds
        ];

        for (saving, expected_count) in savings {
            let count = count_paths(&grid, &config, saving, true);
            assert_eq!(
                count, expected_count,
                "Expected {expected_count} cheats saving exactly {saving} picoseconds"
            );
        }
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_TXT), 999_556);
    }
}
