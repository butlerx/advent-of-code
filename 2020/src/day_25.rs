use itertools::Itertools;

fn transform(mut value: i64, subject: i64) -> i64 {
    value *= subject;
    value % 20201227
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

pub fn run(input: &str) -> i64 {
    let (card_key, door_key) = input
        .trim()
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut value = 1;
    for _ in 0..find_loop(7, door_key) {
        value = transform(value, card_key);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "5764801
17807724";

    #[test]
    fn test_find_loop() {
        assert_eq!(find_loop(7, 5764801), 8);
        assert_eq!(find_loop(7, 17807724), 11);
    }

    #[test]
    fn test_run() {
        assert_eq!(run(INPUT), 14897079);
        assert_eq!(run(include_str!("../input/day_25.txt")), 6011069);
    }
}
