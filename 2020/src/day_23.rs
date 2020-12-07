pub fn run(_input: &str, part_two: bool) -> i64 {
    if part_two {
        0
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 0);
        let results = run(include_str!("../input/day_23.txt"), false);
        println!("{}", results);
        assert!(results == 0);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 0);
        let results = run(include_str!("../input/day_23.txt"), true);
        println!("{}", results);
        assert!(results == 0);
    }
}
