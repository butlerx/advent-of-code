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
    z: f64,
    vel_x: f64,
    vel_y: f64,
    vel_z: f64,
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
            z: xyz[2],
            vel_x: vel_xyz[0],
            vel_y: vel_xyz[1],
            vel_z: vel_xyz[2],
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

fn find_possible_speeds(
    possible_speeds: HashSet<i128>,
    differential: i128,
    velocity: i128,
) -> HashSet<i128> {
    let current_possible_speeds = (-300..300)
        .filter(|speed| *speed != velocity && differential % (speed - velocity) == 0)
        .collect::<HashSet<_>>();

    if possible_speeds.is_empty() {
        current_possible_speeds
    } else {
        possible_speeds
            .intersection(&current_possible_speeds)
            .copied()
            .collect()
    }
}

fn calculate_position(stones: &[Hailstone], vel_x: f64, vel_y: f64, vel_z: f64) -> f64 {
    let a = stones[0];
    let b = stones[1];

    let ma = (a.vel_y - vel_y) / (a.vel_x - vel_x);
    let mb = (b.vel_y - vel_y) / (b.vel_x - vel_x);

    let ca = a.y - (ma * a.x);
    let cb = b.y - (mb * b.x);

    let x_pos = (cb - ca) / (ma - mb);
    let y_pos = ma * x_pos + ca;

    let time = (x_pos - a.x) / (a.vel_x - vel_x);
    let z_pos = a.z + (a.vel_z - vel_z) * time;

    x_pos + y_pos + z_pos
}

fn part_2(input: &str) -> f64 {
    let mut stones = input
        .trim()
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<_>>();
    stones.sort_by(|a, b| {
        let x = a.x.partial_cmp(&b.x).unwrap();
        let y = a.y.partial_cmp(&b.y).unwrap();
        let z = a.z.partial_cmp(&b.z).unwrap();
        x.then(y).then(z)
    });

    let mut possible_x_speeds: HashSet<i128> = HashSet::new();
    let mut possible_y_speeds: HashSet<i128> = HashSet::new();
    let mut possible_z_speeds: HashSet<i128> = HashSet::new();

    for (i, h1) in stones.iter().enumerate() {
        for h2 in stones.iter().skip(i + 1) {
            if h1.vel_x == h2.vel_x {
                let dx = (h2.x - h1.x) as i128;
                possible_x_speeds = find_possible_speeds(possible_x_speeds, dx, h1.vel_x as i128);
            }

            if h1.vel_y == h2.vel_y {
                let dy = (h2.y - h1.y) as i128;
                possible_y_speeds = find_possible_speeds(possible_y_speeds, dy, h1.vel_y as i128);
            }

            if h1.vel_z == h2.vel_z {
                let dz = (h2.z - h1.z) as i128;
                possible_z_speeds = find_possible_speeds(possible_z_speeds, dz, h1.vel_z as i128);
            }
        }
    }

    let vel_x = *possible_x_speeds.iter().next().unwrap() as f64;
    let vel_y = *possible_y_speeds.iter().next().unwrap() as f64;
    let vel_z = *possible_z_speeds.iter().next().unwrap() as f64;

    calculate_position(&stones, vel_x, vel_y, vel_z)
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
