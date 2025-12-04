#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_shared::{time_execution, Point};
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/08.txt");

fn main() {
    println!("🌟 --- Day 8 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

struct Map {
    antenna: HashMap<char, Vec<Point>>,
    max_x: i64,
    max_y: i64,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let antenna = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_alphanumeric())
                    .map(move |(x, c)| (c, Point::from((x, y))))
            })
            .fold(
                HashMap::new(),
                |mut map: HashMap<char, Vec<Point>>, (c, point)| {
                    map.entry(c).or_default().push(point);
                    map
                },
            );
        let max_boundary = Point::from((
            input.lines().next().map_or(0, |line| line.len() - 1),
            input.lines().count() - 1,
        ));
        Self {
            antenna,
            max_x: max_boundary.x,
            max_y: max_boundary.y,
        }
    }
}

impl Map {
    fn find_antenna(&self, include_path: bool) -> usize {
        self.antenna
            .values()
            .flat_map(|beacon_list| {
                beacon_list.iter().enumerate().flat_map(|(i, first)| {
                    beacon_list[i + 1..].iter().flat_map(move |second| {
                        let delta = *second - *first;
                        if include_path {
                            let negative = first.generate_sequence(-delta, self.max_x, self.max_y);
                            let positive = first.generate_sequence(delta, self.max_x, self.max_y);
                            negative.chain(positive.skip(1)).collect::<HashSet<_>>()
                        } else {
                            [*first - delta, *second + delta]
                                .into_iter()
                                .filter(|point| point.in_bounds(self.max_x, self.max_y))
                                .collect()
                        }
                    })
                })
            })
            .collect::<HashSet<_>>()
            .len()
    }
}

fn part_1(input: &str) -> usize {
    Map::from(input).find_antenna(false)
}

fn part_2(input: &str) -> usize {
    Map::from(input).find_antenna(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 14);
        assert_eq!(part_1(INPUT_TXT), 276);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 34);
        assert_eq!(part_2(INPUT_TXT), 991);
    }
}
