use regex::Regex;

fn parse_policy(policy: &str) -> (usize, usize, char, String) {
    let cap = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")
        .unwrap()
        .captures(policy)
        .unwrap();
    (
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].chars().next().unwrap(),
        cap[4].to_string(),
    )
}

fn valid_policy(policy: &str, position: bool) -> bool {
    if policy == "" {
        return false;
    }
    let (low, high, letter, password) = parse_policy(policy);
    if position {
        // the first char will be a space but index 0 doesnt exist so ignore
        let pass = password.chars().collect::<Vec<char>>();
        let pos_1 = pass[low - 1] == letter;
        let pos_2 = pass[high - 1] == letter;
        return (pos_1 || pos_2) && (pos_1 != pos_2);
    }
    let count = password.matches(letter).count();
    (low <= count) && (count <= high)
}

fn run(input: &str, position: bool) -> i64 {
    input
        .split("\n")
        .filter(|line| valid_policy(line.trim(), position))
        .collect::<Vec<_>>()
        .len() as i64
}

pub fn part_1(input: &str) -> i64 {
    run(input, false)
}

pub fn part_2(input: &str) -> i64 {
    run(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn test_parse_policy() {
        let (low, high, letter, password) = parse_policy("1-3 a: test");
        assert!(low == 1 && high == 3 && letter == 'a' && password == "test")
    }

    #[test]
    fn test_valid_policy() {
        let input = "1-3 a: abcde";
        assert!(valid_policy(input.clone(), false));
        assert!(valid_policy(input.clone(), true));
        assert!(!valid_policy("2-9 c: ccccccccc", true))
    }

    #[test]
    fn test_part_1() {
        assert!(part_1(INPUT) == 2);
        //assert!(part_1(include_str!("../input/day_2.txt")) == 515);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT) == 1);
        //assert!(part_2(include_str!("../input/day_2.txt")) == 711);
    }
}
