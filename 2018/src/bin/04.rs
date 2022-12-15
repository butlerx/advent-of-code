use itertools::{enumerate, Itertools};
use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> HashMap<i64, Vec<i64>> {
    let mut guard = 0;
    let mut timer = 0;

    let re = regex::Regex::new(concat!(
        r"\[\d{4}-(?P<month>\d{2})-(?P<day>\d{2}) \d{2}:(?P<minute>\d{2})] ",
        r"(Guard #(?P<guard>\d+)|(falls asleep)|(?P<wakes>wakes up))",
    ))
    .unwrap();

    input
        .trim()
        .lines()
        .sorted()
        .fold(HashMap::new(), |mut roster, event| {
            let cap = re.captures(event).unwrap();
            let now = cap["minute"].parse().unwrap();

            if let Some(id) = cap.name("guard") {
                guard = id.as_str().parse().unwrap();
            } else if cap.name("wakes").is_some() {
                let timetable = roster.entry(guard).or_insert_with(|| vec![0; 60]);
                for time in &mut timetable[timer..now] {
                    *time += 1;
                }
            }
            timer = now;
            roster
        })
}

fn part_1(input: &str) -> i64 {
    let timetables = parse_input(input);
    let (&id, timetable): (&i64, &Vec<i64>) = timetables
        .iter()
        .max_by_key(|(_, v)| v.iter().sum::<i64>())
        .unwrap();
    id * enumerate(timetable).max_by_key(|&(_, c)| c).unwrap().0 as i64
}

fn part_2(input: &str) -> i64 {
    let timetables = parse_input(input);
    let (&id, timetable): (&i64, &Vec<i64>) = timetables
        .iter()
        .max_by_key(|(_, v)| *v.iter().max().unwrap())
        .unwrap();
    id * enumerate(timetable).max_by_key(|&(_, c)| c).unwrap().0 as i64
}

#[cfg(test)]
mod day_1_tests {
    use super::*;
    static INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up ";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 240);
        assert_eq!(part_1(INPUT_TXT), 143415);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4455);
        assert_eq!(part_2(INPUT_TXT), 49944);
    }
}
