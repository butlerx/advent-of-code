#![warn(clippy::perf)]
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/24.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let xyz = pos
            .trim()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        let vel_xyz = vel
            .trim()
            .split(", ")
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            x: xyz[0],
            y: xyz[1],
            vel_x: vel_xyz[0],
            vel_y: vel_xyz[1],
        }
    }
}

impl Hailstone {
    fn intersects(self, h: &Hailstone) -> Option<(f64, f64)> {
        let m1 = self.vel_y / self.vel_x;
        let m2 = h.vel_y / h.vel_x;
        if (m2 - m1).abs() < f64::EPSILON {
            return None;
        }
        let x = (m1 * self.x - m2 * h.x + h.y - self.y) / (m1 - m2);
        let y = (m1 * m2 * (h.x - self.x) + m2 * self.y - m1 * h.y) / (m2 - m1);
        Some((x, y))
    }
}

fn intersections_between(stones: &[Hailstone], start: f64, end: f64) -> usize {
    stones
        .iter()
        .enumerate()
        .flat_map(|(i, h1)| {
            stones.iter().skip(i + 1).filter_map(move |h2| {
                h1.intersects(h2).and_then(|(x, y)| {
                    if (h1.vel_x < 0.0 && x > h1.x)
                        || (h1.vel_x > 0.0 && x < h1.x)
                        || (h2.vel_x < 0.0 && x > h2.x)
                        || (h2.vel_x > 0.0 && x < h2.x)
                    {
                        None
                    } else if (start..=end).contains(&x) && (start..=end).contains(&y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
        })
        .count()
}

fn part_1(input: &str) -> usize {
    let stones = input
        .trim()
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<_>>();
    intersections_between(&stones, 200_000_000_000_000.0, 400_000_000_000_000.0)
}

/// A hailstone with exact integer coordinates, used by part 2 so that every
/// intermediate value is exact.
#[derive(Debug, Clone, Copy)]
struct IntHailstone {
    pos: [i128; 3],
    vel: [i128; 3],
}

fn parse_vector(s: &str) -> [i128; 3] {
    let values = s
        .split(',')
        .map(|n| n.trim().parse::<i128>().unwrap())
        .collect::<Vec<_>>();
    [values[0], values[1], values[2]]
}

fn parse_integer_stones(input: &str) -> Vec<IntHailstone> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            IntHailstone {
                pos: parse_vector(pos),
                vel: parse_vector(vel),
            }
        })
        .collect()
}

fn sub(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [i128; 3], b: [i128; 3]) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Rock speeds on one axis that fit every pair of stones moving at the same speed on it.
/// A pair gives `differential` divisible by `speed - stone_speed`, or no constraint when
/// the rock moves with that stone on this axis.
fn candidate_speeds(stones: &[IntHailstone], axis: usize) -> Vec<i128> {
    let mut candidates: Option<HashSet<i128>> = None;
    for (i, a) in stones.iter().enumerate() {
        for b in &stones[i + 1..] {
            if a.vel[axis] != b.vel[axis] {
                continue;
            }
            let differential = b.pos[axis] - a.pos[axis];
            let possible = (-300..300)
                .filter(|&speed| speed == a.vel[axis] || differential % (speed - a.vel[axis]) == 0)
                .collect::<HashSet<_>>();
            candidates = Some(match candidates {
                Some(previous) => previous.intersection(&possible).copied().collect(),
                None => possible,
            });
        }
    }
    candidates.unwrap_or_default().into_iter().collect()
}

/// The rock's starting position for a given rock velocity, if one exists that every
/// stone's path meets. Uses the first two stones, then checks the rest exactly.
fn rock_start(stones: &[IntHailstone], velocity: [i128; 3]) -> Option<[i128; 3]> {
    let (a, b) = (stones[0], stones[1]);
    let rel_a = sub(a.vel, velocity);
    let rel_b = sub(b.vel, velocity);
    let normal = cross(rel_a, rel_b);
    let denom = dot(normal, normal);
    if denom == 0 {
        return None;
    }
    // The rock starts at a.pos + t * rel_a, where t solves (a.pos - b.pos + t * rel_a) x rel_b = 0.
    let offset = dot(cross(sub(a.pos, b.pos), rel_b), normal);
    let numerators = [0, 1, 2].map(|i| a.pos[i] * denom - offset * rel_a[i]);
    if numerators.iter().any(|n| n % denom != 0) {
        return None;
    }
    let start = numerators.map(|n| n / denom);
    stones
        .iter()
        .all(|s| cross(sub(start, s.pos), sub(s.vel, velocity)) == [0, 0, 0])
        .then_some(start)
}

/// Converts an integer to `f64` exactly, splitting it into 32-bit halves so that no
/// rounding happens for values up to 2^63.
fn to_f64(value: i128) -> f64 {
    let high = i32::try_from(value >> 32).expect("value fits in 2^63");
    let low = u32::try_from(value & 0xffff_ffff).expect("masked to 32 bits");
    f64::from(high) * 4_294_967_296.0 + f64::from(low)
}

fn part_2(input: &str) -> f64 {
    let stones = parse_integer_stones(input);
    let speeds_x = candidate_speeds(&stones, 0);
    let speeds_y = candidate_speeds(&stones, 1);
    let speeds_z = candidate_speeds(&stones, 2);

    for &vx in &speeds_x {
        for &vy in &speeds_y {
            for &vz in &speeds_z {
                if let Some(start) = rock_start(&stones, [vx, vy, vz]) {
                    return to_f64(start.iter().sum());
                }
            }
        }
    }
    panic!("no rock trajectory matches every hailstone");
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part_1() {
        let stones = INPUT
            .trim()
            .lines()
            .map(Hailstone::from)
            .collect::<Vec<_>>();
        assert_eq!(intersections_between(&stones, 7.0, 27.0), 2);
        assert_eq!(part_1(INPUT_TXT), 25810);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 47.0);
        assert_eq!(part_2(INPUT_TXT), 652_666_650_475_950.0);
    }
}
