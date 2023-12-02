use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/02.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn part_1(input: &str) -> i64 {
    let count = input.trim().lines().fold((0, 0), |mut count, line| {
        let letter_counts: HashMap<char, i64> = line.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        if letter_counts.iter().any(|(_, &value)| 2 == value) {
            count.0 += 1;
        }
        if letter_counts.iter().any(|(_, &value)| 3 == value) {
            count.1 += 1;
        }
        count
    });
    count.0 * count.1
}

fn part_2(input: &str) -> String {
    let mut lines = input.trim().lines();
    'outer: loop {
        let Some(line) = lines.next() else {
            unreachable!()
        };
        for other_line in input.trim().lines() {
            let mut iter = line
                .chars()
                .zip(other_line.chars())
                .filter(|(c1, c2)| c1 != c2);
            if iter.next().is_some() && iter.next().is_none() {
                break 'outer line
                    .chars()
                    .zip(other_line.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c1, _)| c1)
                    .collect::<String>();
            }
        }
    }
}

#[cfg(test)]
mod day_2_tests {
    use super::*;

    static INPUT_1: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    static INPUT_2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 12);
        assert_eq!(part_1(INPUT_2), 0);
        assert_eq!(part_1(INPUT_TXT), 3952);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_1), "abcde".to_string());
        assert_eq!(part_2(INPUT_2), "fgij".to_string());
        assert_eq!(part_2(INPUT_TXT), "vtnikorkulbfejvyznqgdxpaw".to_string());
    }
}
