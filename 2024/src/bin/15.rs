#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::{Grid, Point, time_execution};
use std::collections::HashSet;
static INPUT_TXT: &str = include_str!("../../input/15.txt");

fn main() {
    println!("🌟 --- Day 15 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const UP: Point = Point { x: 0, y: -1 };
    const DOWN: Point = Point { x: 0, y: 1 };
    const LEFT: Point = Point { x: -1, y: 0 };
    const RIGHT: Point = Point { x: 1, y: 0 };

    fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    fn vector(self) -> Point {
        match self {
            Self::Up => Self::UP,
            Self::Down => Self::DOWN,
            Self::Left => Self::LEFT,
            Self::Right => Self::RIGHT,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    let (map_str, path_str) = input.trim().split_once("\n\n").expect("invalid input");
    let map = map_str.lines().map(|line| line.chars().collect()).collect();
    let path = path_str
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect();
    (map, path)
}

fn find_robot(grid: &Grid<char>) -> Point {
    grid.iter()
        .find(|(_, &c)| c == '@')
        .map(|(p, _)| p)
        .expect("No robot found")
}

type MoveResult = (bool, Vec<(Point, char)>);

fn move_expanded_vertical(grid: &Grid<char>, start: Point, d: Point) -> MoveResult {
    let mut row = vec![start];
    let mut to_move = Vec::new();

    let empty = loop {
        if row.iter().all(|&cur| grid.get(cur + d) == Some('.')) {
            break true;
        }
        if row.iter().any(|&cur| grid.get(cur + d) == Some('#')) {
            break false;
        }

        let hit: HashSet<_> = row
            .iter()
            .map(|&p| p + d)
            .flat_map(|next| match grid.get(next) {
                Some('[') => vec![(next, '['), (next + Point::from((1_i64, 0)), ']')],
                Some(']') => vec![(next, ']'), (next + Point::from((-1_i64, 0)), '[')],
                _ => vec![],
            })
            .collect();

        if hit.is_empty() {
            break false;
        }

        to_move.extend(hit.clone());
        row = hit.into_iter().map(|(p, _)| p).collect();
    };
    (empty, to_move)
}

fn move_block(grid: &Grid<char>, start: Point, d: Point) -> MoveResult {
    let mut pos = start + d;
    let mut to_move = Vec::new();

    while let Some(c) = grid.get(pos) {
        match c {
            '.' => return (true, to_move),
            'O' | '[' | ']' => {
                if let Some(val) = grid.get(pos) {
                    to_move.push((pos, val));
                }
                pos = pos + d;
            }
            _ => return (false, to_move),
        }
    }
    (false, to_move)
}

fn move_objects(grid: &mut Grid<char>, robot: Point, d: Direction, part_2: bool) -> Point {
    let move_vector = d.vector();
    let (empty, to_move) = if part_2 && d.is_vertical() {
        move_expanded_vertical(grid, robot, move_vector)
    } else {
        move_block(grid, robot, move_vector)
    };

    if !empty {
        return robot;
    }

    let target_positions: HashSet<_> = to_move.iter().map(|(p, _)| *p + move_vector).collect();
    for (p, val) in &to_move {
        if !target_positions.contains(p) {
            grid.set(*p, '.');
        }
        grid.set(*p + move_vector, *val);
    }

    robot + move_vector
}

fn sum_gps(mut grid: Grid<char>, moves: Vec<Direction>, part_2: bool) -> i64 {
    let mut robot = find_robot(&grid);
    grid.set(robot, '.');

    for direction in moves {
        robot = move_objects(&mut grid, robot, direction, part_2);
    }

    grid.iter()
        .filter(|&(_, &c)| c == 'O' || c == '[')
        .map(|(pos, _)| pos.y * 100 + pos.x)
        .sum()
}

fn part_1(input: &str) -> i64 {
    let (grid_data, moves) = parse_input(input);
    let grid = Grid::from(grid_data);
    sum_gps(grid, moves, false)
}

fn expand_grid(row: &[char]) -> Vec<char> {
    row.iter()
        .flat_map(|&c| match c {
            '#' | '.' => vec![c, c],
            'O' => vec!['[', ']'],
            '@' => vec!['@', '.'],
            _ => panic!("Invalid character"),
        })
        .collect()
}

fn part_2(input: &str) -> i64 {
    let (grid_data, moves) = parse_input(input);
    let grid = Grid::from(
        grid_data
            .iter()
            .map(|row| expand_grid(row))
            .collect::<Vec<_>>(),
    );

    sum_gps(grid, moves, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    static INPUT_2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 10092);
        assert_eq!(part_1(INPUT_2), 2028);
        assert_eq!(part_1(INPUT_TXT), 1_471_826);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 9021);
        assert_eq!(part_2(INPUT_2), 1751);
        assert_eq!(part_2(INPUT_TXT), 1_457_703);
    }
}
