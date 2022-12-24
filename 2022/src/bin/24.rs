use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
};
static INPUT_TXT: &str = include_str!("../../input/24.txt");
const DIRECTIONS: [Point; 5] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 0 },
];

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn walk(blizzards: Vec<(Point, usize)>, width: i64, height: i64, trips: usize) -> i64 {
    let mut bps = (0..=height)
        .map(|_| {
            (0..=width)
                .map(|_| HashSet::<(i64, i64)>::new())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let dir_periods = [(width - 1), (width - 1), (height - 1), (height - 1)];

    for (start, dir) in blizzards {
        let mut current_pos = start;
        let dir_vec = DIRECTIONS[dir];
        let dir_period = dir_periods[dir];
        for i in 0.. {
            bps[current_pos.y as usize][current_pos.x as usize].insert((i, dir_period));
            current_pos += dir_vec;
            if current_pos.x <= 0 {
                current_pos.x = width - 1;
            } else if current_pos.x >= width {
                current_pos.x = 1;
            } else if current_pos.y <= 0 {
                current_pos.y = height - 1;
            } else if current_pos.y >= height {
                current_pos.y = 1;
            }

            if current_pos == start {
                break;
            }
        }
    }
    let mut start = Point::new(1, 0);
    let mut end = Point::new(width - 1, height);
    let mut minimum_steps = 0;
    for _ in 0..trips {
        let mut current_queue = [start].into_iter().collect::<HashSet<_>>();
        let mut next_queue = HashSet::new();
        'searching: loop {
            for current_position in current_queue.drain() {
                if current_position == end {
                    break 'searching;
                }

                if ((current_position.x <= 0
                    || current_position.x >= width
                    || current_position.y <= 0
                    || current_position.y >= height)
                    && current_position != start)
                    || bps[current_position.y as usize][current_position.x as usize]
                        .iter()
                        .any(|(offset, period)| (minimum_steps % period) == *offset)
                {
                    continue;
                }

                next_queue.extend(
                    DIRECTIONS
                        .into_iter()
                        .map(|dir| current_position + dir)
                        .collect::<HashSet<_>>(),
                );
            }

            std::mem::swap(&mut current_queue, &mut next_queue);
            minimum_steps += 1;
        }

        std::mem::swap(&mut start, &mut end);
    }
    minimum_steps
}

fn parse_input(input: &str) -> (Vec<(Point, usize)>, i64, i64) {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let width = (lines[0].len() - 1) as i64;
    let height = (lines.len() - 1) as i64;
    let blizzards = lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, b)| b != '.' && b != '#')
                .map(move |(x, b)| {
                    let dir = match b {
                        '>' => 0,
                        '<' => 1,
                        'v' => 2,
                        '^' => 3,
                        _ => unreachable!(),
                    };

                    (Point::new(x as i64, y as i64), dir)
                })
        })
        .collect::<Vec<_>>();
    (blizzards, width, height)
}

fn part_1(input: &str) -> i64 {
    let (blizzards, width, height) = parse_input(input);
    walk(blizzards, width, height, 1)
}

fn part_2(input: &str) -> i64 {
    let (blizzards, width, height) = parse_input(input);
    walk(blizzards, width, height, 3)
}

#[cfg(test)]
mod day_24_tests {
    use super::*;
    static INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 18);
        assert_eq!(part_1(INPUT_TXT), 257);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 54);
        assert_eq!(part_2(INPUT_TXT), 828);
    }
}
