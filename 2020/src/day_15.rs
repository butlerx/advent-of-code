use std::collections::HashMap;
fn play_game(nums: Vec<i64>, turns: usize) -> i64 {
    let mut spoken: HashMap<i64, usize> = HashMap::new();
    let mut last = 0;
    for turn in 0..turns {
        last = if turn < nums.len() {
            let number = nums[turn];
            spoken.insert(number, turn);
            number
        } else {
            let number = match spoken.get(&last) {
                Some(last_turn) => turn - 1 - last_turn,
                None => 0,
            };
            spoken.insert(last, turn - 1);
            number as i64
        }
    }
    last
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    play_game(nums, if part_two { 30_000_000 } else { 2020 })
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "0,3,6";
    static INPUT_1: &str = "1,3,2";
    static INPUT_2: &str = "2,1,3";
    static INPUT_3: &str = "1,2,3";
    static INPUT_4: &str = "2,3,1";
    static INPUT_5: &str = "3,2,1";
    static INPUT_6: &str = "3,1,2";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 436);
        assert!(run(INPUT_1, false) == 1);
        assert!(run(INPUT_2, false) == 10);
        assert!(run(INPUT_3, false) == 27);
        assert!(run(INPUT_4, false) == 78);
        assert!(run(INPUT_5, false) == 438);
        assert!(run(INPUT_6, false) == 1836);
        let results = run(include_str!("../input/day_15.txt"), false);
        println!("{}", results);
        assert!(results == 700);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 175594);
        assert!(run(INPUT_1, false) == 2578);
        assert!(run(INPUT_2, false) == 3544142);
        assert!(run(INPUT_3, false) == 261214);
        assert!(run(INPUT_4, false) == 6895259);
        assert!(run(INPUT_5, false) == 18);
        assert!(run(INPUT_6, false) == 362);
        let results = run(include_str!("../input/day_15.txt"), true);
        println!("{}", results);
        assert!(results == 51358);
    }
}
