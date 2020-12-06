use itertools::Itertools;
use std::io::Error;

fn run(input: &str, part_2: bool) -> Result<i64, Error> {
    Ok(input
        .split("\n\n")
        .map(|line| {
            if !part_2 {
                line.replace('\n', "")
                    .chars()
                    .unique()
                    .collect::<Vec<char>>()
                    .len()
            } else {
                let lines = line.lines().collect::<Vec<&str>>().len();
                line.replace('\n', "")
                    .chars()
                    .unique()
                    .map(|c| {
                        if line.matches(c).count() == lines {
                            1
                        } else {
                            0
                        }
                    })
                    .sum()
            }
        })
        .sum::<usize>() as i64)
}

pub fn part_1(input: &str) -> Result<i64, Error> {
    run(input, false)
}

pub fn part_2(input: &str) -> Result<i64, Error> {
    run(input, true)
}

#[cfg(test)]
mod tests {
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
        assert!(part_1(INPUT).unwrap() == 11);
        let results = part_1(include_str!("../input/day_six.txt")).unwrap();
        println!("{}", results);
        assert!(results == 6742);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT).unwrap() == 6);
        let results = part_2(include_str!("../input/day_six.txt")).unwrap();
        println!("{}", results);
        assert!(results == 3447);
    }
}
