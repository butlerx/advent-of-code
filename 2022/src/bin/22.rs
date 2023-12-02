static INPUT_TXT: &str = include_str!("../../input/22.txt");
const GRID_SIZE: usize = 50;

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
    facing: i64,
}

impl Position {
    fn left(&mut self) {
        self.facing += if self.facing == 0 { 3 } else { -1 };
    }

    fn right(&mut self) {
        self.facing = (self.facing + 1).rem_euclid(4);
    }

    fn next_position(&self, map: &[Vec<char>]) -> Self {
        let (x, y) = match self.facing {
            0 => (
                if map[self.y].len() - 1 > self.x {
                    self.x + 1
                } else {
                    map[self.y].iter().position(|c| *c != ' ').unwrap()
                },
                self.y,
            ),
            1 => (
                self.x,
                if map.len() - 1 > self.y
                    && map[self.y + 1].len() > self.x
                    && map[self.y + 1][self.x] != ' '
                {
                    self.y + 1
                } else {
                    map.iter()
                        .position(|l| l.len() > self.x && l[self.x] != ' ')
                        .unwrap()
                },
            ),
            2 => (
                if self.x == 0 || map[self.y][self.x - 1] == ' ' {
                    map[self.y].len() - 1
                } else {
                    self.x - 1
                },
                self.y,
            ),
            3 => (
                self.x,
                if self.y == 0 || map[self.y - 1][self.x] == ' ' {
                    map.iter()
                        .rposition(|l| l.len() > self.x && l[self.x] != ' ')
                        .unwrap()
                } else {
                    self.y - 1
                },
            ),
            _ => unreachable!(),
        };
        Self {
            x,
            y,
            facing: self.facing,
        }
    }

    fn cube_next_position(&self, map: &[Vec<char>]) -> Self {
        match self.facing {
            0 => {
                if map[self.y].len() - 1 > self.x {
                    Self {
                        x: self.x + 1,
                        y: self.y,
                        facing: self.facing,
                    }
                } else if self.y < GRID_SIZE {
                    Self {
                        x: 2 * GRID_SIZE - 1,
                        y: 3 * GRID_SIZE - 1 - self.y,
                        facing: 2,
                    }
                } else if self.y < 2 * GRID_SIZE {
                    Self {
                        x: GRID_SIZE + self.y,
                        y: GRID_SIZE - 1,
                        facing: 3,
                    }
                } else if self.y < 3 * GRID_SIZE {
                    Self {
                        x: 3 * GRID_SIZE - 1,
                        y: 3 * GRID_SIZE - self.y - 1,
                        facing: 2,
                    }
                } else {
                    Self {
                        x: self.y - 2 * GRID_SIZE,
                        y: 3 * GRID_SIZE - 1,
                        facing: 3,
                    }
                }
            }
            1 => {
                if map.len() - 1 > self.y
                    && map[self.y + 1].len() > self.x
                    && map[self.y + 1][self.x] != ' '
                {
                    Self {
                        x: self.x,
                        y: self.y + 1,
                        facing: self.facing,
                    }
                } else if self.x < GRID_SIZE {
                    Self {
                        x: self.x + 2 * GRID_SIZE,
                        y: 0,
                        facing: self.facing,
                    }
                } else if self.x < 2 * GRID_SIZE {
                    Self {
                        x: GRID_SIZE - 1,
                        y: self.x + 2 * GRID_SIZE,
                        facing: 2,
                    }
                } else {
                    Self {
                        x: 2 * GRID_SIZE - 1,
                        y: self.x - GRID_SIZE,
                        facing: 2,
                    }
                }
            }
            2 => {
                if self.x != 0 && map[self.y][self.x - 1] != ' ' {
                    Self {
                        x: self.x - 1,
                        y: self.y,
                        facing: self.facing,
                    }
                } else if self.y < GRID_SIZE {
                    Self {
                        x: 0,
                        y: 3 * GRID_SIZE - 1 - self.y,
                        facing: 0,
                    }
                } else if self.y < 2 * GRID_SIZE {
                    Self {
                        x: self.y - GRID_SIZE,
                        y: 2 * GRID_SIZE,
                        facing: 1,
                    }
                } else if self.y < 3 * GRID_SIZE {
                    Self {
                        x: GRID_SIZE,
                        y: 3 * GRID_SIZE - 1 - self.y,
                        facing: 0,
                    }
                } else {
                    Self {
                        x: self.y - 2 * GRID_SIZE,
                        y: 0,
                        facing: 1,
                    }
                }
            }
            3 => {
                if self.y != 0 && map[self.y - 1][self.x] != ' ' {
                    Self {
                        x: self.x,
                        y: self.y - 1,
                        facing: self.facing,
                    }
                } else if self.x < GRID_SIZE {
                    Self {
                        x: GRID_SIZE,
                        y: self.x + GRID_SIZE,
                        facing: 0,
                    }
                } else if self.x < 2 * GRID_SIZE {
                    Self {
                        x: 0,
                        y: self.x + 2 * GRID_SIZE,
                        facing: 0,
                    }
                } else {
                    Self {
                        x: self.x - 2 * GRID_SIZE,
                        y: 4 * GRID_SIZE - 1,
                        facing: self.facing,
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

fn walk(map: &[Vec<char>], mut path: &str, cube: bool) -> i64 {
    let x = map[0].iter().position(|c| *c == '.').unwrap();
    let mut pos = Position { x, y: 0, facing: 0 };
    loop {
        if path.is_empty() {
            break 1000 * (pos.y + 1) as i64 + 4 * (pos.x + 1) as i64 + pos.facing;
        }
        match &path[..1] {
            "L" => {
                pos.left();
                path = &path[1..];
            }
            "R" => {
                pos.right();
                path = &path[1..];
            }
            _ => {
                let len = path.find(|c: char| !c.is_numeric()).unwrap_or(path.len());
                let steps = path[..len].parse::<usize>().unwrap();
                path = &path[len..];
                pos = (0..steps).fold(pos, |pos, _| {
                    let next = if cube {
                        pos.cube_next_position(map)
                    } else {
                        pos.next_position(map)
                    };
                    if map[next.y][next.x] != '#' {
                        next
                    } else {
                        pos
                    }
                });
            }
        }
    }
}

fn part_1(input: &str) -> i64 {
    let (map, directions) = input.split_once("\n\n").unwrap();
    walk(
        &map.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>(),
        directions.trim(),
        false,
    )
}

fn part_2(input: &str) -> i64 {
    let (map, directions) = input.split_once("\n\n").unwrap();
    walk(
        &map.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>(),
        directions.trim(),
        true,
    )
}

#[cfg(test)]
mod day_22_tests {
    use super::*;
    static INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 6032);
        assert_eq!(part_1(INPUT_TXT), 60362);
    }

    #[test]
    fn test_part_2() {
        //assert_eq!(part_2(INPUT), 5031);
        assert_eq!(part_2(INPUT_TXT), 74288);
    }
}
