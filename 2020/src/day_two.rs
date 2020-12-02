use std::io::Error;

struct Rule {
    high: i64,
    low: i64,
    letter: char,
}

fn parse_rule(rule: &str) -> Rule {
    let rules: Vec<_> = rule.split_whitespace().map(|x| x.trim()).collect();
    let nums: Vec<_> = rules[0]
        .split("-")
        .map(|x| x.trim().parse::<i64>().expect("parse error"))
        .collect();
    Rule {
        high: nums[1],
        low: nums[0],
        letter: rules[1].chars().next().unwrap(),
    }
}

fn valid_policy(policy: String, position: bool) -> bool {
    if policy == "" {
        return false;
    }
    let policy_args: Vec<_> = policy.split(":").collect();
    let rule = parse_rule(policy_args[0]);
    if position {
        // the first char will be a space but index 0 doesnt exist so ignore
        let password = policy_args[1].chars().collect::<Vec<char>>();
        let pos_1 = password[rule.low as usize] == rule.letter;
        let pos_2 = password[rule.high as usize] == rule.letter;
        return (pos_1 || pos_2) && (pos_1 != pos_2);
    }
    let count = policy_args[1].matches(rule.letter).count();
    (rule.low as usize <= count) && (count <= rule.high as usize)
}

pub fn run(input: String, position: bool) -> Result<i64, Error> {
    Ok(input
        .split("\n")
        .filter(|line| valid_policy(line.trim().parse::<String>().unwrap(), position))
        .collect::<Vec<_>>()
        .len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let input = "1-3 a";
        let rule = parse_rule(input);
        assert!(rule.low == 1 && rule.high == 3 && rule.letter == 'a')
    }

    #[test]
    fn test_valid_policy() {
        let input = "1-3 a: abcde".to_string();
        let valid = valid_policy(input, false);
        assert!(valid)
    }

    #[test]
    fn test_valid_policy_position() {
        let input = "1-3 a: abcde".to_string();
        let valid = valid_policy(input, true);
        assert!(valid);
        let input_2 = "2-9 c: ccccccccc".to_string();
        assert!(!valid_policy(input_2, true))
    }

    #[test]
    fn test_run() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".to_string();
        let results = run(input, false).unwrap();
        assert!(results == 2);
    }

    #[test]
    fn test_run_position() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".to_string();
        let results = run(input, true).unwrap();
        assert!(results == 1);
    }
}
