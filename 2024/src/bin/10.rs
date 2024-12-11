use aoc_2024::{Grid, Point};
use std::{
    collections::{HashSet, VecDeque},
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

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Map {
    grid: Grid<usize>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let grid: Grid<usize> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("invalid number") as usize)
                    .collect()
            })
            .collect();
        Self { grid }
    }
}

impl Map {
    fn rate_map(&self, part_2: bool) -> usize {
        (0..self.grid.height)
            .flat_map(|y| (0..self.grid.width).map(move |x| Point::from((x as i32, y as i32))))
            .filter(|xy| matches!(self.grid.get(*xy), Some(0)))
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|start| {
                let mut todo = VecDeque::from([(start, 1)]);
                let mut scores = vec![];

                while let Some((xy, height)) = todo.pop_front() {
                    if height == 10 {
                        scores.push(xy);
                    }
                    todo.extend(self.get_valid_paths(&xy, height));
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
            .into_iter()
            .map(|(dx, dy)| *xy + Point::from((dx, dy)))
            .filter(move |p| self.grid.get(*p).map_or(false, |digit| height == digit))
            .map(move |p| (p, height + 1))
    }
}

fn part_1(input: &str) -> usize {
    Map::from(input).rate_map(false)
}

fn part_2(input: &str) -> usize {
    Map::from(input).rate_map(true)
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
