use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/06.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn run(input: &str, part_2: bool) -> i64 {
    input
        .split("\n\n")
        .map(|line| {
            if !part_2 {
                line.replace('\n', "").chars().unique().count()
            } else {
                let lines = line.lines().count();
                line.replace('\n', "")
                    .chars()
                    .unique()
                    .map(|c| usize::from(line.matches(c).count() == lines))
                    .sum()
            }
        })
        .sum::<usize>() as i64
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
        assert!(run(INPUT, false) == 11);
        let results = run(include_str!("../../input/06.txt"), false);
        println!("{results}");
        assert!(results == 6742);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 6);
        let results = run(include_str!("../../input/06.txt"), true);
        println!("{results}");
        assert!(results == 3447);
    }
}
