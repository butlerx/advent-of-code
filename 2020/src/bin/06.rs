use itertools::Itertools;
use std::convert::TryFrom;

fn main() {
    let input = include_str!("../../input/06.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn run(input: &str, part_2: bool) -> i64 {
    i64::try_from(
        input
            .split("\n\n")
            .map(|line| {
                if part_2 {
                    let lines = line.lines().count();
                    line.replace('\n', "")
                        .chars()
                        .unique()
                        .map(|c| usize::from(line.matches(c).count() == lines))
                        .sum()
                } else {
                    line.replace('\n', "").chars().unique().count()
                }
            })
            .sum::<usize>(),
    )
    .expect("cast within range")
}

#[cfg(test)]
mod day_6_tests {
    use super::*;
    static INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 11);
        let results = run(include_str!("../../input/06.txt"), false);
        println!("{results}");
        assert_eq!(results, 6742);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 6);
        let results = run(include_str!("../../input/06.txt"), true);
        println!("{results}");
        assert_eq!(results, 3447);
    }
}
