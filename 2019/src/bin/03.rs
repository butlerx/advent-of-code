use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

const COMMA: &str = ",";

fn main() {
    let input = include_str!("../../input/03.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn find_path(path: Vec<(char, usize)>) -> HashMap<(i64, i64), i64> {
    let mut cells = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    for (direction, distance) in path {
        for _ in 0..distance {
            match direction {
                'L' => x -= 1,
                'R' => x += 1,
                'U' => y += 1,
                'D' => y -= 1,
                _ => unreachable!(),
            }
            steps += 1;
            cells.insert((x, y), steps);
        }
    }

    cells
}
fn parse(path: &str) -> Vec<(char, usize)> {
    path.split(COMMA)
        .map(|direction| {
            (
                direction.chars().next().unwrap(),
                direction[1..].parse().unwrap(),
            )
        })
        .collect()
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let paths: Vec<_> = input.lines().map(|line| find_path(parse(line))).collect();
    if part_two {
        paths[0]
            .iter()
            .filter_map(|(cell, steps)| paths[1].get(cell).map(|s| steps + s))
            .min()
            .unwrap()
    } else {
        let path_1: HashSet<&(i64, i64)> = HashSet::from_iter(paths[0].keys());
        let path_2: HashSet<&(i64, i64)> = HashSet::from_iter(paths[1].keys());
        path_1
            .intersection(&path_2)
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    static INPUT_2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT_1, false), 159);
        assert_eq!(run(INPUT_2, false), 135);
        assert_eq!(run(include_str!("../../input/03.txt"), false), 1519);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT_1, true), 610);
        assert_eq!(run(INPUT_2, true), 410);
        assert_eq!(run(include_str!("../../input/03.txt"), true), 14358);
    }
}
