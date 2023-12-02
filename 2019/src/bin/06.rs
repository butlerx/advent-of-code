use itertools::Itertools;
use std::{collections::HashMap, iter::successors};

fn main() {
    let input = include_str!("../../input/06.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn calculate(orbits: &HashMap<&str, Vec<&str>>, object: &str, distance: i64) -> i64 {
    orbits.get(object).map_or(0, |v| {
        v.iter()
            .map(|orbit| calculate(orbits, orbit, distance + 1))
            .sum::<i64>()
    }) + distance
}

fn calc_distance(orbits: HashMap<&str, &str>, source: &str, dest: &str) -> i64 {
    let mut you: Vec<_> = successors(orbits.get(source), |&o| orbits.get(o))
        .copied()
        .collect();
    you.reverse();
    let mut san: Vec<_> = successors(orbits.get(dest), |&o| orbits.get(o))
        .copied()
        .collect();
    san.reverse();
    let shared = you.iter().zip(&san).take_while(|(a, b)| a == b).count() * 2;
    (you.len() + san.len() - shared) as i64
}

fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        calc_distance(
            input
                .lines()
                .map(|line| {
                    let l: (&str, &str) = line.split(')').collect_tuple().unwrap();
                    (l.1, l.0)
                })
                .collect(),
            "YOU",
            "SAN",
        )
    } else {
        calculate(
            &input
                .lines()
                .map(|line| line.split(')').collect_tuple().unwrap())
                .into_group_map(),
            "COM",
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    static INPUT_2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 42);
        assert_eq!(run(include_str!("../../input/06.txt"), false), 200_001);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT_2, true), 4);
        assert_eq!(run(include_str!("../../input/06.txt"), true), 379);
    }
}
