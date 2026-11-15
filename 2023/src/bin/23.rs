#![warn(clippy::pedantic, clippy::perf)]
use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
};

static INPUT_TXT: &str = include_str!("../../input/23.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

type StartPositions = VecDeque<(usize, usize, HashSet<(usize, usize)>)>;

fn find_start_positions(grid: &[Vec<char>]) -> StartPositions {
    grid[0]
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c == '.')
        .map(|(x, _)| (x, 0, HashSet::new()))
        .collect()
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut queue = find_start_positions(&grid);
    let mut max_path = 0;
    while !queue.is_empty() {
        let (x, y, mut visited) = queue.pop_front().unwrap();
        if !visited.insert((x, y)) {
            continue;
        }
        if y == grid.len() - 1 {
            max_path = max(max_path, visited.len());
            continue;
        }
        match grid[y][x] {
            '#' => {}
            '.' => {
                let directions = &[(0i32, -1i32), (0, 1), (-1, 0), (1, 0)];
                for (dx, dy) in directions {
                    let new_x = i32::try_from(x).expect("couldnt convert to i32") + dx;
                    let new_y = i32::try_from(y).expect("couldnt convert to i32") + dy;
                    if new_x >= 0 && new_y >= 0 {
                        let new_x = new_x.unsigned_abs() as usize;
                        let new_y = new_y.unsigned_abs() as usize;
                        if new_x < grid[0].len() && new_y < grid.len() && grid[new_y][new_x] != '#'
                        {
                            queue.push_back((new_x, new_y, visited.clone()));
                        }
                    }
                }
            }
            '<' => {
                if x > 0 && grid[y][x - 1] != '#' {
                    queue.push_back((x - 1, y, visited));
                }
            }
            '>' => {
                if x < grid[0].len() - 1 && grid[y][x + 1] != '#' {
                    queue.push_back((x + 1, y, visited));
                }
            }
            '^' => {
                if y > 0 && grid[y - 1][x] != '#' {
                    queue.push_back((x, y - 1, visited));
                }
            }
            'v' => {
                if y < grid.len() - 1 && grid[y + 1][x] != '#' {
                    queue.push_back((x, y + 1, visited));
                }
            }
            _ => unreachable!(),
        }
    }

    max_path - 1
}

fn open_neighbours(grid: &[Vec<char>], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    DIRECTIONS
        .iter()
        .filter_map(|&(dx, dy)| {
            let nx = x.checked_add_signed(dx)?;
            let ny = y.checked_add_signed(dy)?;
            let cell = *grid.get(ny)?.get(nx)?;
            (cell != '#').then_some((nx, ny))
        })
        .collect()
}

/// Longest path length (in steps) from `node` to `end` that does not revisit a junction.
fn longest_path(
    edges: &[Vec<(usize, usize)>],
    node: usize,
    end: usize,
    visited: u64,
) -> Option<usize> {
    if node == end {
        return Some(0);
    }
    edges[node]
        .iter()
        .filter(|&&(next, _)| visited & (1_u64 << next) == 0)
        .filter_map(|&(next, len)| {
            longest_path(edges, next, end, visited | (1_u64 << next)).map(|rest| rest + len)
        })
        .max()
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let start = (
        grid[0]
            .iter()
            .position(|&c| c == '.')
            .expect("open start on top row"),
        0,
    );
    let end = (
        grid[height - 1]
            .iter()
            .position(|&c| c == '.')
            .expect("open end on bottom row"),
        height - 1,
    );

    // Junctions (more than two open neighbours, or the start and end) are the graph nodes;
    // the corridors between them become weighted edges.
    let is_junction = |pos| pos == start || pos == end || open_neighbours(&grid, pos).len() > 2;
    let junctions = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|&(x, y)| grid[y][x] != '#' && is_junction((x, y)))
        .collect::<Vec<_>>();
    let index = junctions
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect::<HashMap<_, _>>();

    let mut edges = vec![Vec::new(); junctions.len()];
    for (from_idx, &from) in junctions.iter().enumerate() {
        for first in open_neighbours(&grid, from) {
            let (mut prev, mut cur, mut len) = (from, first, 1);
            while !is_junction(cur) {
                let Some(next) = open_neighbours(&grid, cur).into_iter().find(|&p| p != prev)
                else {
                    break;
                };
                prev = cur;
                cur = next;
                len += 1;
            }
            if let Some(&to_idx) = index.get(&cur) {
                edges[from_idx].push((to_idx, len));
            }
        }
    }

    let start_idx = index[&start];
    let end_idx = index[&end];
    longest_path(&edges, start_idx, end_idx, 1_u64 << start_idx).expect("end is reachable")
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 94);
        assert_eq!(part_1(INPUT_TXT), 2246);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 154);
        assert_eq!(part_2(INPUT_TXT), 6622);
    }
}
