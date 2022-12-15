use itertools::Itertools;
use std::collections::VecDeque;

static INPUT_TXT: &str = include_str!("../../input/12.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Point = (usize, usize);

fn find_path(grid: &[Vec<u8>], start: Point, goal: Point) -> Option<usize> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = vec![(start, 0)].into_iter().collect::<VecDeque<_>>();
    while let Some(((x, y), len)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(len);
        }
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            let Some(&next_square) = grid.get(nx).and_then(|row| row.get(ny)) else { continue };
            if grid[x][y] + 1 >= next_square && !visited[nx][ny] {
                visited[nx][ny] = true;
                queue.push_back(((nx, ny), len + 1));
            }
        }
    }
    None
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Point, Point) {
    let mut grid = input
        .trim()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'S')
        .unwrap();
    let goal = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'E')
        .unwrap();
    grid[start.0][start.1] = b'a';
    grid[goal.0][goal.1] = b'z';
    (grid, start, goal)
}
fn part_1(input: &str) -> usize {
    let (grid, start, goal) = parse_input(input);
    find_path(&grid, start, goal).unwrap()
}

fn part_2(input: &str) -> usize {
    let (grid, _, goal) = parse_input(input);
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| grid[x][y] == b'a')
        .filter_map(|start| find_path(&grid, start, goal))
        .min()
        .unwrap()
}

#[cfg(test)]
mod day_12_tests {
    use super::*;
    static INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 31);
        assert_eq!(part_1(INPUT_TXT), 490);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 29);
        assert_eq!(part_2(INPUT_TXT), 488);
    }
}
