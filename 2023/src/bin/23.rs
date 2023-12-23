use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

static INPUT_TXT: &str = include_str!("../../input/23.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn find_start_positions(grid: &[Vec<char>]) -> VecDeque<(usize, usize, HashSet<(usize, usize)>)> {
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
                let directions = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
                for (dx, dy) in directions {
                    let new_x = x.wrapping_add(*dx as usize);
                    let new_y = y.wrapping_add(*dy as usize);
                    if new_x < grid[0].len() && new_y < grid.len() && grid[new_y][new_x] != '#' {
                        queue.push_back((new_x, new_y, visited.clone()));
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

fn part_2(input: &str) -> usize {
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
            _ => {
                let directions = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
                for (dx, dy) in directions {
                    let new_x = x.wrapping_add(*dx as usize);
                    let new_y = y.wrapping_add(*dy as usize);
                    if new_x < grid[0].len() && new_y < grid.len() && grid[new_y][new_x] != '#' {
                        queue.push_back((new_x, new_y, visited.clone()));
                    }
                }
            }
        }
    }

    max_path - 1
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
