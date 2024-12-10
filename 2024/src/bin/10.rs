use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
    time::Instant,
};

static INPUT_TXT: &str = include_str!("../../input/10.txt");

fn main() {
    println!("🌟 --- Day 10 Results --- 🌟");
    let start_1 = Instant::now();
    let res_1 = part_1(INPUT_TXT);
    let duration_1 = start_1.elapsed().as_millis();
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let start_2 = Instant::now();
    let res_2 = part_2(INPUT_TXT);
    let duration_2 = start_2.elapsed().as_millis();
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Grid {
    data: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let data: Vec<Vec<usize>> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("invalid number") as usize)
                    .collect()
            })
            .collect();
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }
}

impl Grid {
    fn get(&self, p: &Point) -> Option<usize> {
        let x = p.x as usize;
        let y = p.y as usize;
        if p.x >= 0 && p.y >= 0 && x < self.width && y < self.height {
            Some(self.data[y][x])
        } else {
            None
        }
    }

    fn rate_map(&self, part_2: bool) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Point::from((x as i32, y as i32))))
            .filter(|xy| matches!(self.get(xy), Some(0)))
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|start| {
                let mut todo = VecDeque::from([(start, 1)]);
                let mut scores = vec![];

                while let Some((xy, height)) = todo.pop_front() {
                    if height == 10 {
                        scores.push(xy);
                    }
                    todo.extend(self.get_valid_paths(&xy, height))
                }

                if part_2 {
                    scores.len()
                } else {
                    let scores_set: HashSet<Point> = scores.into_iter().collect();
                    scores_set.len()
                }
            })
            .sum()
    }

    fn get_valid_paths<'a>(
        &'a self,
        xy: &'a Point,
        height: usize,
    ) -> impl Iterator<Item = (Point, usize)> + '_ {
        DIRECTIONS
            .iter()
            .map(|(dx, dy)| *xy + Point::from((*dx, *dy)))
            .filter(move |p| self.get(p).map_or(false, |digit| height == digit))
            .map(move |p| (p, height + 1))
    }
}

fn part_1(input: &str) -> usize {
    Grid::from(input).rate_map(false)
}

fn part_2(input: &str) -> usize {
    Grid::from(input).rate_map(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "0123
1234
8765
9876";
    static INPUT_2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1);
        assert_eq!(part_1(INPUT_2), 36);
        assert_eq!(part_1(INPUT_TXT), 517);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 16);
        assert_eq!(part_2(INPUT_2), 81);
        assert_eq!(part_2(INPUT_TXT), 1116);
    }
}
