fn main() {
    let input = include_str!("../../input/15.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn play_game(nums: Vec<usize>, turns: usize) -> i64 {
    let mut spoken: Vec<(usize, usize)> = vec![(usize::MAX, usize::MAX); turns];
    for (i, num) in nums.iter().enumerate() {
        spoken[*num].1 = i;
    }
    let mut next_num: usize = *nums.last().unwrap();
    let mut turn_num = nums.len();
    loop {
        if turn_num >= turns {
            break next_num as i64;
        }
        let (last_turn, turn) = spoken[next_num];
        next_num = if last_turn != usize::MAX && turn != usize::MAX {
            turn - last_turn
        } else {
            0
        };
        spoken[next_num] = (spoken[next_num].1, turn_num);
        turn_num += 1;
    }
}

fn run(input: &str, part_two: bool) -> i64 {
    let nums = input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    play_game(nums, if part_two { 30_000_000 } else { 2020 })
}

#[cfg(test)]
mod day_15_tests {
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
        assert_eq!(run(INPUT, false), 436);
        assert_eq!(run(INPUT_1, false), 1);
        assert_eq!(run(INPUT_2, false), 10);
        assert_eq!(run(INPUT_3, false), 27);
        assert_eq!(run(INPUT_4, false), 78);
        assert_eq!(run(INPUT_5, false), 438);
        assert_eq!(run(INPUT_6, false), 1836);
        assert_eq!(run(include_str!("../../input/15.txt"), false), 700);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 175_594);
        assert_eq!(run(INPUT_1, true), 2578);
        assert_eq!(run(INPUT_2, true), 3_544_142);
        assert_eq!(run(INPUT_3, true), 261_214);
        assert_eq!(run(INPUT_4, true), 6_895_259);
        assert_eq!(run(INPUT_5, true), 18);
        assert_eq!(run(INPUT_6, true), 362);
        assert_eq!(run(include_str!("../../input/15.txt"), true), 51358);
    }
}
