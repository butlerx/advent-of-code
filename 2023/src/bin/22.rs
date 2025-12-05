#![warn(clippy::pedantic, clippy::perf)]
use std::collections::{HashMap, HashSet};

static INPUT_TXT: &str = include_str!("../../input/22.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let mut split = s.split(',');
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    from: Pos,
    to: Pos,
}

impl From<&str> for Brick {
    fn from(s: &str) -> Self {
        let (from, to) = s.split_once('~').unwrap();
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Brick {
    fn intersects(&self, b: &Brick) -> bool {
        ((self.from.x..=self.to.x).contains(&b.from.x)
            || (self.from.x..=self.to.x).contains(&b.to.x)
            || (b.from.x..=b.to.x).contains(&self.from.x)
            || (b.from.x..=b.to.x).contains(&self.to.x))
            && ((self.from.y..=self.to.y).contains(&b.from.y)
                || (self.from.y..=self.to.y).contains(&b.to.y)
                || (b.from.y..=b.to.y).contains(&self.from.y)
                || (b.from.y..=b.to.y).contains(&self.to.y))
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.trim().lines().map(Brick::from).collect()
}

fn check_stability(
    bricks: &mut [Brick],
    mut supported_by: Option<&mut HashMap<usize, Vec<usize>>>,
) -> usize {
    bricks.sort_unstable_by_key(|b| b.from.z);
    let mut in_place: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut count = HashSet::new();
    for i in 0..bricks.len() {
        while bricks[i].from.z > 1 {
            let mut falling = true;
            let brick = &bricks[i];
            for &j in in_place.get(&(brick.from.z - 1)).unwrap_or(&vec![]) {
                let b = &bricks[j];
                if brick.intersects(b) {
                    if let Some(sb) = &mut supported_by {
                        sb.entry(i).or_default().push(j);
                    }
                    falling = false;
                }
            }
            if falling {
                let brick = &mut bricks[i];
                brick.from.z -= 1;
                brick.to.z -= 1;
                count.insert(i);
            } else {
                break;
            }
        }
        in_place.entry(bricks[i].to.z).or_default().push(i);
    }
    count.len()
}

fn part_1(input: &str) -> usize {
    let mut bricks = parse_input(input);
    let mut supported_by = HashMap::new();
    check_stability(&mut bricks, Some(&mut supported_by));
    (0..bricks.len())
        .filter(|i| !supported_by.values().any(|v| v.len() == 1 && v.contains(i)))
        .count()
}

fn part_2(input: &str) -> usize {
    let mut bricks = parse_input(input);
    check_stability(&mut bricks, None);
    (0..bricks.len())
        .map(|i| {
            let mut bricks = bricks.clone();
            bricks.remove(i);
            check_stability(&mut bricks, None)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 5);
        assert_eq!(part_1(INPUT_TXT), 421);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 7);
        assert_eq!(part_2(INPUT_TXT), 39247);
    }
}
