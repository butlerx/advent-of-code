//! Systems Check - Mission 2
//!
//! <https://systemscheck.dev/missions/2>

use aoc_shared::{Grid, Point};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

const CARDINALS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

const ALL_NEIGHBORS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
];

fn bfs(grid: &Grid<char>, start: Point, target: Point) -> usize {
    let mut visited = Grid::new(Point::from((grid.width - 1, grid.height - 1)), false);
    visited.set(start, true);

    let mut frontier = VecDeque::from([(start, 0)]);

    loop {
        let (pos, dist) = frontier.pop_front().expect("No path found");
        if pos == target {
            return dist;
        }
        let neighbors: Vec<_> = CARDINALS
            .iter()
            .map(|dir| pos + *dir)
            .filter(|&next| grid.get(next).is_some_and(|c| c != '#'))
            .filter(|&next| visited.get(next) == Some(false))
            .collect();

        for next in neighbors {
            visited.set(next, true);
            frontier.push_back((next, dist + 1));
        }
    }
}

fn dijkstra(grid: &Grid<char>, start: Point, target: Point) -> usize {
    let bounds = Point::from((grid.width - 1, grid.height - 1));
    let mut dist_grid = Grid::new(bounds, usize::MAX);
    dist_grid.set(start, 0);

    let cost_grid: Grid<usize> = grid
        .iter()
        .fold(Grid::new(bounds, 1), |mut costs, (pos, _)| {
            if ALL_NEIGHBORS
                .iter()
                .any(|d| grid.get(pos + *d) == Some('#'))
            {
                costs.set(pos, 2);
            }
            costs
        });

    let mut heap = BinaryHeap::from([Reverse((0, start))]);

    loop {
        let Reverse((cost, pos)) = heap.pop().expect("No path found");
        if pos == target {
            return cost;
        }
        if cost > dist_grid.get(pos).unwrap_or(usize::MAX) {
            continue;
        }
        for next in CARDINALS
            .iter()
            .map(|dir| pos + *dir)
            .filter(|&next| grid.get(next).is_some_and(|c| c != '#'))
        {
            let new_cost = cost + cost_grid.get(next).unwrap_or(1);
            if new_cost < dist_grid.get(next).unwrap_or(usize::MAX) {
                dist_grid.set(next, new_cost);
                heap.push(Reverse((new_cost, next)));
            }
        }
    }
}

#[must_use]
/// Part 1: Find the shortest path from the top-left corner to the target '*' in the grid.
///
/// # Panics
///
/// Panics if the target is not found in the grid.
pub fn part_1(input: &str) -> usize {
    let grid: Grid<char> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let target = grid.find_position('*').expect("No target found");
    bfs(&grid, Point::new(0, 0), target)
}

#[must_use]
/// Part 2: Find the shortest path from the top-left corner to the target '*' in the grid,
/// considering the cost of moving through cells adjacent to walls.
///
/// # Panics
///
/// panics if the target is not found in the grid.
pub fn part_2(input: &str) -> usize {
    let grid: Grid<char> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let target = grid.find_position('*').expect("No target found");
    dijkstra(&grid, Point::new(0, 0), target)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...........#####
..........######
.........##.#..#
...##.#..###.#.#
...##...#####..#
...####.########
.#...#....#.####
...###.#######..
#.####.##.#..###
#####..#####.##.
##..#.....#.####
.##..##...##.###
##.####...##.##.
.##.##...######.
##.###.....#....
#####.#........*";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 32);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 50);
    }
}
