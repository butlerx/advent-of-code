#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{Point, time_execution, time_execution_us};
static INPUT_TXT: &str = include_str!("../../input/09.txt");

fn main() {
    println!("🌟 --- Day 9 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn part_1(input: &str) -> i64 {
    let coords: Vec<Point> = input.trim().lines().map(Point::from).collect();

    coords
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| coords[i + 1..].iter().map(move |&p2| p1.area_distance(p2)))
        .max()
        .unwrap_or(0)
}

fn part_2(input: &str) -> i64 {
    let coords: Vec<Point> = input.trim().lines().map(Point::from).collect();

    coords
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| {
            coords[i + 1..]
                .iter()
                .map(move |&p2| Rect::from_points(p1, p2))
        })
        .filter(|&rect| is_valid_rect(rect, &coords))
        .map(Rect::area)
        .max()
        .unwrap_or(0)
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Rect {
    fn from_points(p1: Point, p2: Point) -> Self {
        Self {
            x_min: p1.x.min(p2.x),
            x_max: p1.x.max(p2.x),
            y_min: p1.y.min(p2.y),
            y_max: p1.y.max(p2.y),
        }
    }

    const fn area(self) -> i64 {
        (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)
    }

    const fn center_doubled(self) -> (i64, i64) {
        (self.x_min + self.x_max, self.y_min + self.y_max)
    }

    const fn contains_x(self, x: i64) -> bool {
        self.x_min < x && x < self.x_max
    }

    const fn contains_y(self, y: i64) -> bool {
        self.y_min < y && y < self.y_max
    }
}

fn is_valid_rect(rect: Rect, poly: &[Point]) -> bool {
    let (mx, my) = rect.center_doubled();

    is_point_in_poly(mx, my, poly) && !has_edge_intersection(rect, poly)
}

fn has_edge_intersection(rect: Rect, poly: &[Point]) -> bool {
    let len = poly.len();

    (0..len).any(|i| {
        let u = poly[i];
        let v = poly[(i + 1) % len];

        if u.x == v.x {
            if rect.contains_x(u.x) {
                let ey_min = u.y.min(v.y);
                let ey_max = u.y.max(v.y);
                let overlap_start = rect.y_min.max(ey_min);
                let overlap_end = rect.y_max.min(ey_max);

                overlap_start < overlap_end
            } else {
                false
            }
        } else if rect.contains_y(u.y) {
            let ex_min = u.x.min(v.x);
            let ex_max = u.x.max(v.x);
            let overlap_start = rect.x_min.max(ex_min);
            let overlap_end = rect.x_max.min(ex_max);

            overlap_start < overlap_end
        } else {
            false
        }
    })
}

fn is_point_in_poly(x: i64, y: i64, poly: &[Point]) -> bool {
    let len = poly.len();

    let on_edge = (0..len).any(|i| {
        let u = poly[i];
        let v = poly[(i + 1) % len];
        is_on_edge(x, y, u, v)
    });

    if on_edge {
        return true;
    }

    let intersections = (0..len)
        .filter(|&i| {
            let u = poly[i];
            let v = poly[(i + 1) % len];

            if u.x == v.x {
                let min_y = u.y.min(v.y) * 2;
                let max_y = u.y.max(v.y) * 2;
                let ex = u.x * 2;

                y >= min_y && y < max_y && ex > x
            } else {
                false
            }
        })
        .count();

    intersections % 2 == 1
}

#[inline]
fn is_on_edge(x: i64, y: i64, u: Point, v: Point) -> bool {
    if u.x == v.x {
        let min_y = (u.y * 2).min(v.y * 2);
        let max_y = (u.y * 2).max(v.y * 2);
        u.x * 2 == x && y >= min_y && y <= max_y
    } else {
        let min_x = (u.x * 2).min(v.x * 2);
        let max_x = (u.x * 2).max(v.x * 2);
        u.y * 2 == y && x >= min_x && x <= max_x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 50);
        assert_eq!(part_1(INPUT_TXT), 4_750_092_396);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 24);
        assert_eq!(part_2(INPUT_TXT), 1_468_516_555);
    }
}
