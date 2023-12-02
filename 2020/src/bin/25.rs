use itertools::Itertools;

fn main() {
    println!("Part 1: {}", run("../../input/25.txt"));
}

fn transform(value: i64, subject: i64) -> i64 {
    (value * subject) % 20_201_227
}

fn find_loop(subject: i64, goal: i64) -> i64 {
    let mut loops = 0;
    let mut value = 1;
    loop {
        if value == goal {
            break loops;
        }
        value = transform(value, subject);
        loops += 1;
    }
}

fn run(input: &str) -> i64 {
    let (card_key, door_key) = input
        .trim()
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    (0..find_loop(7, door_key)).fold(1, |value, _| transform(value, card_key))
}

#[cfg(test)]
mod day_25_tests {
    use super::*;
    static INPUT: &str = "5764801
17807724";

    #[test]
    fn test_find_loop() {
        assert_eq!(find_loop(7, 5_764_801), 8);
        assert_eq!(find_loop(7, 17_807_724), 11);
    }

    #[test]
    fn test_run() {
        assert_eq!(run(INPUT), 14_897_079);
        assert_eq!(run(include_str!("../../input/25.txt")), 6_011_069);
    }
}
