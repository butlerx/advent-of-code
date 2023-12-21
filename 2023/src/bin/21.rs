use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/21.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct BigPosition {
    position: Position,
    x: isize,
    y: isize,
}

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
                (x as isize - 1, y as isize),
                (x as isize + 1, y as isize),
                (x as isize, y as isize - 1),
                (x as isize, y as isize + 1),
            ]
            .into_iter()
            .filter(|&(x, y)| x >= 0 && y >= 0 && x < width as isize && y < height as isize)
            .filter(|&(x, y)| garden[y as usize][x as usize] == '.')
            .map(|(x, y)| (x as usize, y as usize))
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

fn get_big_positions_by_grid(
    big_positions: &HashSet<BigPosition>,
) -> HashMap<(isize, isize), usize> {
    big_positions
        .iter()
        .fold(HashMap::new(), |mut counts, &BigPosition { x, y, .. }| {
            *counts.entry((x, y)).or_insert(0) += 1;
            counts
        })
}

fn get_big_destination_count(
    graph: &HashMap<Position, Vec<Position>>,
    width: usize,
    height: usize,
    start: Position,
    steps: usize,
) -> usize {
    let big_start = BigPosition {
        position: start,
        x: 0,
        y: 0,
    };
    let big_positions =
        get_big_destinations(graph, width, height, big_start, steps % width + width * 2);
    let counts = get_big_positions_by_grid(&big_positions);

    let tip = counts.get(&(-2, 0)).unwrap_or(&0)
        + counts.get(&(2, 0)).unwrap_or(&0)
        + counts.get(&(0, -2)).unwrap_or(&0)
        + counts.get(&(0, 2)).unwrap_or(&0);
    let edge1 = counts.get(&(-2, -1)).unwrap_or(&0)
        + counts.get(&(-2, 1)).unwrap_or(&0)
        + counts.get(&(2, -1)).unwrap_or(&0)
        + counts.get(&(2, 1)).unwrap_or(&0);
    let edge2 = counts.get(&(-1, -1)).unwrap_or(&0)
        + counts.get(&(-1, 1)).unwrap_or(&0)
        + counts.get(&(1, -1)).unwrap_or(&0)
        + counts.get(&(1, 1)).unwrap_or(&0);
    let center1 = counts.get(&(0, 1)).unwrap_or(&0);
    let center2 = counts.get(&(0, 0)).unwrap_or(&0);

    let num = steps / width;
    tip + edge1 * num + edge2 * (num - 1) + center1 * num * num + center2 * (num - 1) * (num - 1)
}

fn get_big_destinations(
    graph: &HashMap<Position, Vec<Position>>,
    width: usize,
    height: usize,
    start: BigPosition,
    steps: usize,
) -> HashSet<BigPosition> {
    let mut position_set = HashSet::new();
    position_set.insert(start);

    for _ in 0..steps {
        let mut new_set = HashSet::new();
        for &BigPosition { position, x, y } in &position_set {
            if let Some(neighbors) = graph.get(&position) {
                new_set.extend(neighbors.iter().map(|&p| BigPosition { position: p, x, y }));
                if position.0 == 0 {
                    new_set.insert(BigPosition {
                        position: (width - 1, position.1),
                        x: x - 1,
                        y,
                    });
                }
                if position.0 == width - 1 {
                    new_set.insert(BigPosition {
                        position: (0, position.1),
                        x: x + 1,
                        y,
                    });
                }
                if position.1 == 0 {
                    new_set.insert(BigPosition {
                        position: (position.0, height - 1),
                        x,
                        y: y - 1,
                    });
                }
                if position.1 == height - 1 {
                    new_set.insert(BigPosition {
                        position: (position.0, 0),
                        x,
                        y: y + 1,
                    });
                }
            }
        }
        position_set = new_set;
    }
    position_set
}

fn take_big_steps(input: &str, steps: usize) -> usize {
    let (garden, start) = parse_input(input);
    let width = garden[0].len();
    let height = garden.len();
    let graph = get_graph(&garden);
    get_big_destination_count(&graph, width, height, start, steps)
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
