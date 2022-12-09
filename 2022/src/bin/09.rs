use std::collections::HashSet;
static INPUT_TXT: &str = include_str!("../../input/09.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn rope_sim(input: &str, followers: usize) -> usize {
    let mut rope = vec![(0i64, 0i64); followers + 1];
    input
        .trim()
        .lines()
        .fold(
            vec![(0, 0)].into_iter().collect::<HashSet<_>>(),
            |visited, line| {
                let (d, moves) = line.split_once(' ').unwrap();
                let (dx, dy) = match d.chars().next() {
                    Some('U') => (0, 1),
                    Some('D') => (0, -1),
                    Some('R') => (1, 0),
                    Some('L') => (-1, 0),
                    _ => unreachable!(),
                };
                (0..moves.parse::<usize>().unwrap()).fold(visited, |mut visited, _| {
                    rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
                    for i in 1..rope.len() {
                        let (x, y) = rope[i - 1];
                        if (x - rope[i].0).abs() > 1 || (y - rope[i].1).abs() > 1 {
                            rope[i].0 += (x - rope[i].0).signum();
                            rope[i].1 += (y - rope[i].1).signum();
                        }
                    }
                    visited.insert(rope[followers]);
                    visited
                })
            },
        )
        .len()
}
fn part_1(input: &str) -> usize {
    rope_sim(input, 1)
}

fn part_2(input: &str) -> usize {
    rope_sim(input, 9)
}

#[cfg(test)]
mod day_9_tests {
    use super::*;
    static INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    static INPUT_LARGE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13);
        assert_eq!(part_1(INPUT_LARGE), 88);
        assert_eq!(part_1(INPUT_TXT), 5902);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1);
        assert_eq!(part_2(INPUT_LARGE), 36);
        assert_eq!(part_2(INPUT_TXT), 2445);
    }
}
