use itertools::Itertools;

pub fn run(input: &str, part_2: bool) -> i64 {
    input
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
        .sum::<usize>() as i64
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
        assert!(run(INPUT, false) == 11);
        let results = run(include_str!("../input/day_6.txt"), false);
        println!("{}", results);
        assert!(results == 6742);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 6);
        let results = run(include_str!("../input/day_6.txt"), true);
        println!("{}", results);
        assert!(results == 3447);
    }
}
