#![warn(clippy::pedantic, clippy::perf)]
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/10.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn connects_to(&self, grid: &[Vec<char>]) -> Option<(Self, Self)> {
        if self.y >= grid.len() || self.x >= grid[0].len() {
            return None;
        }
        match grid[self.y].get(self.x) {
            Some('|') => Some((
                Point::new(self.x, self.y.wrapping_sub(1)),
                Point::new(self.x, self.y + 1),
            )),
            Some('-') => Some((
                Point::new(self.x.wrapping_sub(1), self.y),
                Point::new(self.x + 1, self.y),
            )),
            Some('L') => Some((
                Point::new(self.x, self.y.wrapping_sub(1)),
                Point::new(self.x + 1, self.y),
            )),
            Some('J') => Some((
                Point::new(self.x.wrapping_sub(1), self.y),
                Point::new(self.x, self.y.wrapping_sub(1)),
            )),
            Some('7') => Some((
                Point::new(self.x.wrapping_sub(1), self.y),
                Point::new(self.x, self.y + 1),
            )),
            Some('F') => Some((
                Point::new(self.x, self.y + 1),
                Point::new(self.x + 1, self.y),
            )),
            _ => None,
        }
    }

    fn minus(&self, other: &Self) -> (i32, i32) {
        (
            i32::try_from(self.x).unwrap() - i32::try_from(other.x).unwrap(),
            i32::try_from(self.y).unwrap() - i32::try_from(other.y).unwrap(),
        )
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![
            Point::new(self.x.wrapping_sub(1), self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y.wrapping_sub(1)),
            Point::new(self.x, self.y + 1),
        ]
    }

    fn mark_grid(&self, grid: Vec<Vec<char>>, pipes: &HashSet<Point>) -> Vec<Vec<char>> {
        if self.y >= grid.len()
            || self.x >= grid[0].len()
            || grid[self.y][self.x] == 'X'
            || pipes.contains(self)
        {
            return grid;
        }
        let mut marked_grid = grid;
        marked_grid[self.y][self.x] = 'X';
        self.neighbours()
            .into_iter()
            .fold(marked_grid, |g, n| n.mark_grid(g, pipes))
    }

    fn mark_from_previous(
        &self,
        grid: Vec<Vec<char>>,
        prev: Point,
        pipe_set: &HashSet<Point>,
    ) -> Vec<Vec<char>> {
        match self.minus(&prev) {
            (1, 0) => {
                vec![
                    Point::new(self.x, self.y + 1),
                    Point::new(self.x.wrapping_sub(1), self.y + 1),
                ]
            }
            (0, 1) => {
                vec![
                    Point::new(self.x.wrapping_sub(1), self.y.wrapping_sub(1)),
                    Point::new(self.x.wrapping_sub(1), self.y),
                ]
            }
            (-1, 0) => {
                vec![
                    Point::new(self.x, self.y.wrapping_sub(1)),
                    Point::new(self.x + 1, self.y.wrapping_sub(1)),
                ]
            }
            (0, -1) => {
                vec![
                    Point::new(self.x + 1, self.y),
                    Point::new(self.x + 1, self.y + 1),
                ]
            }
            _ => vec![],
        }
        .iter()
        .fold(grid, |g, p| p.mark_grid(g, pipe_set))
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start(grid: &[Vec<char>]) -> Option<Point> {
    grid.iter().enumerate().find_map(|(y, line)| {
        line.iter()
            .position(|c| c == &'S')
            .map(|x| Point::new(x, y))
    })
}

fn get_pipes(grid: &[Vec<char>], start: Point) -> Vec<Point> {
    let mut curr = start
        .neighbours()
        .into_iter()
        .find(|&n| {
            if let Some((c1, c2)) = n.connects_to(grid) {
                c1 == start || c2 == start
            } else {
                false
            }
        })
        .unwrap();
    let mut pipes: Vec<Point> = vec![start];
    while grid[curr.y][curr.x] != 'S' {
        let (c1, c2) = curr.connects_to(grid).unwrap();
        let next = if c1 == *pipes.last().unwrap() { c2 } else { c1 };
        pipes.push(curr);
        curr = next;
    }
    pipes
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_start(&grid).expect("No Start Point");
    get_pipes(&grid, start).len() / 2
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_start(&grid).expect("No Start Point");
    let pipes = get_pipes(&grid, start);

    let pipe_set: HashSet<_> = pipes.iter().copied().collect();
    let mut prev = start;
    let marked_grid = pipes.iter().fold(grid, |marked_grid, &segment| {
        let g = segment.mark_from_previous(marked_grid, prev, &pipe_set);
        prev = segment;
        g
    });

    let nx = marked_grid
        .iter()
        .map(|line| line.iter().filter(|&&x| x == 'X').count())
        .sum();
    if marked_grid[0][0] == 'X' {
        (marked_grid.len() * marked_grid[0].len()) - nx - pipe_set.len()
    } else {
        nx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    static INPUT_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    static INPUT_3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    static INPUT_4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    static INPUT_5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 4);
        assert_eq!(part_1(INPUT_2), 8);
        assert_eq!(part_1(INPUT_TXT), 6649);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_3), 4);
        assert_eq!(part_2(INPUT_4), 8);
        assert_eq!(part_2(INPUT_5), 10);
        assert_eq!(part_2(INPUT_TXT), 601);
    }
}
