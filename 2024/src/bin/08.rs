use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Neg, Sub},
    time::Instant,
};

static INPUT_TXT: &str = include_str!("../../input/08.txt");

fn main() {
    println!("🌟 --- Day 8 Results --- 🌟");
    let start_1 = Instant::now();
    let res_1 = part_1(INPUT_TXT);
    let duration_1 = start_1.elapsed().as_millis();
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let start_2 = Instant::now();
    let res_2 = part_2(INPUT_TXT);
    let duration_2 = start_2.elapsed().as_millis();
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    fn in_bounds(&self, bounds: &Self) -> bool {
        (0..=bounds.x).contains(&self.x) && (0..=bounds.y).contains(&self.y)
    }

    fn generate_sequence(self, delta: Self, bounds: &Self) -> impl Iterator<Item = Self> + use<'_> {
        std::iter::successors(Some(self), move |&point| {
            let next = point + delta;
            next.in_bounds(bounds).then_some(next)
        })
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
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

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

type Map = HashMap<char, Vec<Point>>;

fn parse_input(input: &str) -> (Map, Point) {
    let parsed = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphanumeric())
                .map(move |(x, c)| (c, Point::new(x, y)))
        })
        .fold(HashMap::new(), |mut map: Map, (c, point)| {
            map.entry(c).or_default().push(point);
            map
        });

    let max_boundary = Point::new(
        input.lines().next().map_or(0, |line| line.len() - 1),
        input.lines().count() - 1,
    );

    (parsed, max_boundary)
}

fn find_antenna(antenna: &Map, bounds: &Point, include_path: bool) -> usize {
    antenna
        .values()
        .flat_map(|beacon_list| {
            beacon_list.iter().enumerate().flat_map(|(i, first)| {
                beacon_list[i + 1..].iter().flat_map(move |second| {
                    let delta = *second - *first;
                    if include_path {
                        let negative = first.generate_sequence(-delta, bounds);
                        let positive = first.generate_sequence(delta, bounds);
                        negative.chain(positive.skip(1)).collect::<HashSet<_>>()
                    } else {
                        [*first - delta, *second + delta]
                            .into_iter()
                            .filter(|point| point.in_bounds(bounds))
                            .collect()
                    }
                })
            })
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part_1(input: &str) -> usize {
    let (antenna, bounds) = parse_input(input);
    find_antenna(&antenna, &bounds, false)
}

fn part_2(input: &str) -> usize {
    let (antenna, bounds) = parse_input(input);
    find_antenna(&antenna, &bounds, true)
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
