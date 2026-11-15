#![warn(clippy::perf)]
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/21.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Position = (usize, usize);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Position) {
    let mut start = (0, 0);

    let garden = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            if let Some(x) = line.find('S') {
                start = (x, y);
            }
            line.replace('S', ".").chars().collect::<Vec<_>>()
        })
        .collect();
    (garden, start)
}

fn get_graph(garden: &[Vec<char>]) -> HashMap<Position, Vec<Position>> {
    let width = garden[0].len();
    let height = garden.len();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|&(x, y)| garden[y][x] == '.')
        .map(|(x, y)| {
            let neighbors = vec![
                (
                    isize::try_from(x).expect("cast within range") - 1,
                    isize::try_from(y).expect("cast within range"),
                ),
                (
                    isize::try_from(x).expect("cast within range") + 1,
                    isize::try_from(y).expect("cast within range"),
                ),
                (
                    isize::try_from(x).expect("cast within range"),
                    isize::try_from(y).expect("cast within range") - 1,
                ),
                (
                    isize::try_from(x).expect("cast within range"),
                    isize::try_from(y).expect("cast within range") + 1,
                ),
            ]
            .into_iter()
            .filter(|&(x, y)| {
                x >= 0
                    && y >= 0
                    && x < isize::try_from(width).expect("cast within range")
                    && y < isize::try_from(height).expect("cast within range")
            })
            .filter(|&(x, y)| garden[y.unsigned_abs()][x.unsigned_abs()] == '.')
            .map(|(x, y)| (x.unsigned_abs(), y.unsigned_abs()))
            .collect();
            ((x, y), neighbors)
        })
        .collect()
}

fn get_destination_count(
    graph: &HashMap<Position, Vec<Position>>,
    start: Position,
    steps: usize,
) -> usize {
    let mut position_set = HashSet::new();
    position_set.insert(start);

    for _ in 0..steps {
        let mut new_set = HashSet::new();
        for &position in &position_set {
            if let Some(neighbors) = graph.get(&position) {
                new_set.extend(neighbors);
            }
        }
        position_set = new_set;
    }
    position_set.len()
}

fn take_steps(input: &str, steps: usize) -> usize {
    let (garden, start) = parse_input(input);
    let graph = get_graph(&garden);
    get_destination_count(&graph, start, steps)
}

fn part_1(input: &str) -> usize {
    take_steps(input, 64)
}

/// One step of the exact-step reachable set on the infinite, tiled garden.
fn step_infinite(
    garden: &[Vec<char>],
    positions: &HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    let width = isize::try_from(garden[0].len()).expect("cast within range");
    let height = isize::try_from(garden.len()).expect("cast within range");
    positions
        .iter()
        .flat_map(|&(x, y)| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)])
        .filter(|&(x, y)| {
            let col = usize::try_from(x.rem_euclid(width)).expect("cast within range");
            let row = usize::try_from(y.rem_euclid(height)).expect("cast within range");
            garden[row][col] == '.'
        })
        .collect()
}

/// Extrapolates the count at tile `n` from the samples in `counts` (one per whole tile),
/// once the last five samples show the per-tile count is quadratic.
fn extrapolate_tiles(counts: &[usize], tile: usize) -> Option<usize> {
    let samples = counts
        .iter()
        .map(|&c| i128::try_from(c).expect("count fits in i128"))
        .collect::<Vec<_>>();
    let last = samples.len().checked_sub(1)?;
    if last < 4 {
        return None;
    }
    let second_diff = |i: usize| samples[i + 2] - 2 * samples[i + 1] + samples[i];
    let curvature = second_diff(last - 2);
    if second_diff(last - 4) != curvature || second_diff(last - 3) != curvature {
        return None;
    }
    let base = last - 4;
    let offset = i128::try_from(tile - base).expect("index fits in i128");
    let value = samples[base]
        + offset * (samples[base + 1] - samples[base])
        + offset * (offset - 1) / 2 * curvature;
    usize::try_from(value).ok()
}

/// Counts the garden plots reachable in exactly `steps` steps on the tiled grid.
///
/// Each whole tile adds a quadratic number of plots once the pattern settles, so the
/// counts at `steps % width + k * width` are sampled until they do, then extrapolated.
fn get_big_destination_count(garden: &[Vec<char>], start: Position, steps: usize) -> usize {
    let width = garden[0].len();
    let tiles = steps / width;
    let mut positions = HashSet::from([(
        isize::try_from(start.0).expect("cast within range"),
        isize::try_from(start.1).expect("cast within range"),
    )]);
    for _ in 0..steps % width {
        positions = step_infinite(garden, &positions);
    }

    let mut counts = Vec::new();
    loop {
        counts.push(positions.len());
        if tiles < counts.len() {
            return counts[tiles];
        }
        if let Some(value) = extrapolate_tiles(&counts, tiles) {
            return value;
        }
        for _ in 0..width {
            positions = step_infinite(garden, &positions);
        }
    }
}

fn take_big_steps(input: &str, steps: usize) -> usize {
    let (garden, start) = parse_input(input);
    get_big_destination_count(&garden, start, steps)
}

fn part_2(input: &str) -> usize {
    take_big_steps(input, 26_501_365)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part_1() {
        assert_eq!(take_steps(INPUT, 6), 16);
        assert_eq!(part_1(INPUT_TXT), 3709);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(take_big_steps(INPUT, 6), 16);
        assert_eq!(take_big_steps(INPUT, 10), 50);
        assert_eq!(take_big_steps(INPUT, 50), 1594);
        assert_eq!(take_big_steps(INPUT, 100), 6536);
        assert_eq!(take_big_steps(INPUT, 500), 167_004);
        assert_eq!(take_big_steps(INPUT, 1000), 668_697);
        assert_eq!(take_big_steps(INPUT, 5000), 16_733_044);
        assert_eq!(part_2(INPUT_TXT), 617_361_073_602_319);
    }
}
