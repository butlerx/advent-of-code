use std::collections::HashMap;

fn main() {
    let input = parse_input(include_str!("../../input/05.txt"));
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| {
            let (one, two) = line.split_once("->").unwrap();
            let (x1, y1) = one.trim().split_once(',').unwrap();
            let (x2, y2) = two.trim().split_once(',').unwrap();
            let from = (x1.parse::<i64>().unwrap(), y1.parse::<i64>().unwrap());
            let to = (x2.parse::<i64>().unwrap(), y2.parse::<i64>().unwrap());
            if to < from {
                (to, from)
            } else {
                (from, to)
            }
        })
        .collect()
}

fn part_1(lines: Vec<((i64, i64), (i64, i64))>) -> usize {
    count_points_covered(
        lines
            .iter()
            .filter(|((x1, y1), (x2, y2))| y1 == y2 || x1 == x2)
            .cloned()
            .collect(),
    )
}

fn part_2(lines: Vec<((i64, i64), (i64, i64))>) -> usize {
    count_points_covered(lines)
}

fn count_points_covered(lines: Vec<((i64, i64), (i64, i64))>) -> usize {
    let mut points_covered = HashMap::<_, usize>::new();
    for ((x1, y1), (x2, y2)) in lines {
        if y1 == y2 {
            for x in x1..=x2 {
                *points_covered.entry((x, y1)).or_default() += 1;
            }
        } else if x1 == x2 {
            for y in y1..=y2 {
                *points_covered.entry((x1, y)).or_default() += 1;
            }
        } else if y1 < y2 {
            for delta in 0..=(x2 - x1) {
                *points_covered.entry((x1 + delta, y1 + delta)).or_default() += 1;
            }
        } else {
            for delta in 0..=(x2 - x1) {
                *points_covered.entry((x1 + delta, y1 - delta)).or_default() += 1;
            }
        }
    }

    points_covered
        .into_iter()
        .filter_map(|(_, num_lines)| Some(num_lines).filter(|&x| x >= 2))
        .count()
}

#[cfg(test)]
mod day_5_tests {
    use super::*;
    static INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_small_input() {
        let input = parse_input(INPUT);
        assert_eq!(part_1(input.clone()), 5);
        assert_eq!(part_2(input), 12);
    }

    #[test]
    fn test_large_input() {
        let input = parse_input(include_str!("../../input/05.txt"));
        assert_eq!(part_1(input.clone()), 7297);
        assert_eq!(part_2(input), 21038);
    }
}
