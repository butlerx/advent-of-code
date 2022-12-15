use std::str::FromStr;
static INPUT_TXT: &str = include_str!("../../input/14.txt");
const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

const TESTS: [(i64, i64); 3] = [(0, 1), (-1, 1), (1, 1)];
struct Map(Vec<bool>, i64);
impl Default for Map {
    fn default() -> Self {
        Self(vec![false; HEIGHT * WIDTH], 0)
    }
}
impl Map {
    fn is_occupied(&self, (x, y): (i64, i64)) -> bool {
        self.0[y as usize * WIDTH + x as usize]
    }

    fn set(&mut self, (x, y): (i64, i64)) {
        self.0[y as usize * WIDTH + x as usize] = true;
    }

    fn is_overflowing(&self, y: i64) -> bool {
        y >= self.1 + 3
    }

    fn next_position(&self, xy: (i64, i64), use_floor: bool) -> bool {
        !(self.is_occupied(xy) || use_floor && xy.1 == self.1 + 2)
    }

    fn drop_sand_floor(&mut self, use_floor: bool) -> i64 {
        let mut count = 0;
        loop {
            let mut xy = (500, 0);
            loop {
                if self.is_occupied(xy) || self.is_overflowing(xy.1) {
                    return count;
                } else if let Some(next) = TESTS
                    .iter()
                    .map(|&(x, y)| (x + xy.0, y + xy.1))
                    .find(|&next| self.next_position(next, use_floor))
                {
                    xy = next;
                } else {
                    self.set(xy);
                    break;
                }
            }
            count += 1;
        }
    }
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Map::default();
        let mut depth = 0;
        for line in s.trim().lines() {
            let mut points = line.split(" -> ").map(|s| {
                let parts = s
                    .split(',')
                    .map(|p| p.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                (parts[0], parts[1])
            });
            let mut xy = points.next().unwrap();
            for new in points {
                depth = depth.max(new.1);
                result.set(xy);
                while xy != new {
                    xy.0 += match xy.0 {
                        x if x < new.0 => 1,
                        x if x > new.0 => -1,
                        _ => 0,
                    };
                    xy.1 += match xy.1 {
                        y if y < new.1 => 1,
                        y if y > new.1 => -1,
                        _ => 0,
                    };
                    result.set(xy);
                }
            }
        }
        result.1 = depth;
        Ok(result)
    }
}

fn part_1(input: &str) -> i64 {
    input.parse::<Map>().unwrap().drop_sand_floor(false)
}

fn part_2(input: &str) -> i64 {
    input.parse::<Map>().unwrap().drop_sand_floor(true)
}

#[cfg(test)]
mod day_14_tests {
    use super::*;
    static INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 24);
        assert_eq!(part_1(INPUT_TXT), 1513);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 93);
        assert_eq!(part_2(INPUT_TXT), 22646);
    }
}
