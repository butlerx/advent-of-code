use itertools::Itertools;
pub fn run(input: &str, part_two: bool) -> i64 {
    let nums = input.lines().map(|num| num.trim().parse::<i64>().unwrap());
    if part_two {
        nums.tuple_windows()
            .map(|(prev, current, next)| prev + current + next)
            .tuple_windows()
            .map(|(prev, next)| if next > prev { 1 } else { 0 })
            .sum()
    } else {
        nums.tuple_windows()
            .map(|(prev, next)| if next > prev { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 7);
        let results = run(include_str!("../input/day_1.txt"), false);
        println!("{}", results);
        assert!(results == 1715);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 5);
        let results = run(include_str!("../input/day_1.txt"), true);
        println!("{}", results);
        assert!(results == 1739);
    }
}
