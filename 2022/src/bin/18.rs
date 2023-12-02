use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};
static INPUT_TXT: &str = include_str!("../../input/18.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}
type Point = (i64, i64, i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cube {
    position: Point,
    sides: [Point; 6],
}
impl Cube {
    #[must_use]
    pub fn new(position: Point) -> Self {
        let x = position.0 * 2;
        let y = position.1 * 2;
        let z = position.2 * 2;
        Self {
            position,
            sides: Self::sides((x, y, z)),
        }
    }

    #[must_use]
    pub fn sides((x, y, z): Point) -> [Point; 6] {
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(|s| s.parse::<i64>().unwrap());
        let position = (
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        );

        Ok(Self::new(position))
    }
}

fn part_1(input: &str) -> usize {
    let sides = input
        .trim()
        .lines()
        .map(|l| l.parse::<Cube>().unwrap())
        .collect::<HashSet<_>>()
        .into_iter()
        .flat_map(|c| c.sides.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    sides.len() - (sides.len() - sides.iter().copied().collect::<HashSet<_>>().len()) * 2
}

fn part_2(input: &str) -> i64 {
    let cubes = input
        .trim()
        .lines()
        .map(|l| l.parse::<Cube>().unwrap())
        .collect::<HashSet<_>>();

    let (x, y, z) = cubes.iter().fold(
        (
            (i64::MAX, i64::MIN),
            (i64::MAX, i64::MIN),
            (i64::MAX, i64::MIN),
        ),
        |(mut x, mut y, mut z), cube| {
            x.0 = x.0.min(cube.position.0);
            x.1 = x.1.max(cube.position.0);
            y.0 = y.0.min(cube.position.1);
            y.1 = y.1.max(cube.position.1);
            z.0 = z.0.min(cube.position.2);
            z.1 = z.1.max(cube.position.2);
            (x, y, z)
        },
    );

    let wet_sides = {
        let x_range = x.0 - 1..=x.1 + 1;
        let y_range = y.0 - 1..=y.1 + 1;
        let z_range = z.0 - 1..=z.1 + 1;
        let cubes_pos = cubes.iter().map(|c| c.position).collect::<HashSet<_>>();
        let mut water = HashSet::new();
        let mut queue =
            VecDeque::from(vec![(*x_range.start(), *y_range.start(), *z_range.start())]);
        loop {
            if let Some(position @ (x, y, z)) = queue.pop_front() {
                if x_range.contains(&x)
                    && y_range.contains(&y)
                    && z_range.contains(&z)
                    && !water.contains(&position)
                    && !cubes_pos.contains(&position)
                {
                    water.insert(position);
                    queue.extend(Cube::sides(position));
                }
            } else {
                break water;
            }
        }
    }
    .into_iter()
    .map(Cube::new)
    .collect::<HashSet<_>>()
    .iter()
    .flat_map(|c| c.sides)
    .collect::<HashSet<_>>();

    let sides = cubes.iter().flat_map(|c| c.sides).collect::<Vec<_>>();

    cubes
        .iter()
        .filter(|cube| cube.sides.iter().any(|s| wet_sides.contains(s)))
        .fold((0, HashSet::new()), |(n, mut shared), cube| {
            (
                cube.sides
                    .iter()
                    .filter(|side| wet_sides.contains(side))
                    .fold(n, |n, side| {
                        if shared.contains(side) {
                            n
                        } else {
                            match sides.iter().filter(|s| s == &side).count() {
                                1 => n + 1,
                                2 => {
                                    shared.insert(*side);
                                    n + 1
                                }
                                _ => n,
                            }
                        }
                    }),
                shared,
            )
        })
        .0
}

#[cfg(test)]
mod day_18_tests {
    use super::*;
    static INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 64);
        assert_eq!(part_1(INPUT_TXT), 4308);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 58);
        assert_eq!(part_2(INPUT_TXT), 2540);
    }
}
