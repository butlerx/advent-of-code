use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("🌟 --- Day 6 Results --- 🌟");
    println!("📌 Part 1: {}", part_1(INPUT_TXT));
    println!("📌 Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> (Self, Position, Direction) {
        let cols = input.lines().next().unwrap().len();
        let cells: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
        let rows = cells.len() / cols;

        let start_pos = cells
            .iter()
            .position(|&c| c == '^')
            .map(|i| Position {
                row: i / cols,
                col: i % cols,
            })
            .expect("no start position found");

        (Self { cells, rows, cols }, start_pos, Direction::North)
    }

    fn get(&self, pos: Position) -> char {
        self.cells[pos.row * self.cols + pos.col]
    }

    fn set(&mut self, pos: Position, value: char) {
        self.cells[pos.row * self.cols + pos.col] = value;
    }

    fn next_position(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::North if pos.row == 0 => None,
            Direction::North => Some(Position {
                row: pos.row - 1,
                col: pos.col,
            }),
            Direction::East if pos.col == self.cols - 1 => None,
            Direction::East => Some(Position {
                row: pos.row,
                col: pos.col + 1,
            }),
            Direction::South if pos.row == self.rows - 1 => None,
            Direction::South => Some(Position {
                row: pos.row + 1,
                col: pos.col,
            }),
            Direction::West if pos.col == 0 => None,
            Direction::West => Some(Position {
                row: pos.row,
                col: pos.col - 1,
            }),
        }
    }

    fn simulate_path(&self, start: Position, mut dir: Direction) -> HashSet<Position> {
        let mut visited = HashSet::with_capacity(self.rows * self.cols);
        let mut current = start;
        visited.insert(current);

        loop {
            let Some(next) = self.next_position(current, dir) else {
                break visited;
            };

            if self.get(next) == '#' {
                dir = dir.turn_right();
            } else {
                current = next;
                visited.insert(current);
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    let (grid, start, dir) = Grid::new(input);
    grid.simulate_path(start, dir).len()
}

fn part_2(input: &str) -> usize {
    let (grid, start, start_dir) = Grid::new(input);

    let mut test_grid = grid.clone();
    let mut visited = HashSet::with_capacity(grid.rows * grid.cols);

    grid.simulate_path(start, start_dir)
        .into_iter()
        .filter(|&pos| pos != start)
        .filter(|&pos| {
            visited.clear();
            test_grid.set(pos, '#');

            let mut current = start;
            let mut dir = start_dir;

            loop {
                if !visited.insert((current, dir)) {
                    test_grid.set(pos, '.');
                    break true;
                }

                let Some(next) = test_grid.next_position(current, dir) else {
                    test_grid.set(pos, '.');
                    break false;
                };

                if test_grid.get(next) == '#' {
                    dir = dir.turn_right();
                } else {
                    current = next;
                }
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 41);
        assert_eq!(part_1(INPUT_TXT), 5409);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 6);
        assert_eq!(part_2(INPUT_TXT), 2022);
    }
}
