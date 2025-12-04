#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_shared::{Point, time_execution};
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/14.txt");

fn main() {
    println!("🌟 --- Day 14 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT, 101, 103));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT, 101, 103));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn parse_point(s: &str) -> Vec<i64> {
    s.split(',')
        .map(|x| x.parse::<i64>().expect("invalid number"))
        .collect::<Vec<i64>>()
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|l| {
            let (p_str, v_str) = l.split_once(" v=").expect("no v prefix");
            let p_vec = parse_point(p_str.strip_prefix("p=").expect("no p prefix"));
            let p = Point::from((p_vec[0], p_vec[1]));
            let v_vec = parse_point(v_str);
            let v = Point::from((v_vec[0], v_vec[1]));
            (p, v)
        })
        .collect::<Vec<(Point, Point)>>()
}

fn part_1(input: &str, width: i64, height: i64) -> i64 {
    let robots = parse_input(input);
    let mid_x = (width - 1) / 2;
    let mid_y = (height - 1) / 2;
    robots
        .into_iter()
        .map(|(pos, vel)| {
            let x = (pos.x + (vel.x * 100)).rem_euclid(width);
            let y = (pos.y + (vel.y * 100)).rem_euclid(height);
            (x, y)
        })
        .filter_map(|(x, y)| match (x.cmp(&mid_x), y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(0),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(1),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(2),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(3),
            _ => None,
        })
        .fold([0; 4], |mut counts, quadrant| {
            counts[quadrant] += 1;
            counts
        })
        .iter()
        .product()
}

fn part_2(input: &str, width: i64, height: i64) -> i64 {
    let robots = parse_input(input);
    (0..10_000) // arbitrary limit
        .find(|&seconds| {
            let occupied = robots
                .iter()
                .map(|(pos, vel)| Point {
                    x: (pos.x + (vel.x * seconds)).rem_euclid(width),
                    y: (pos.y + (vel.y * seconds)).rem_euclid(height),
                })
                .collect::<HashSet<Point>>();
            occupied.len() == robots.len()
        })
        .expect("no solution in 10,000 seconds")
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT, 11, 7), 12);
        assert_eq!(part_1(INPUT_TXT, 101, 103), 210_587_128);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_TXT, 101, 103), 7286);
    }
}
