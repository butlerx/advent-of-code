#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/01.txt");

fn main() {
    println!("🌟 --- Day 1 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .fold((50, 0), |(pos, count), line| {
            let (direction, distance_str) = line.split_at(1);
            let distance: i32 = distance_str.parse().expect("Failed to parse distance");
            let new_pos = match direction {
                "L" => pos - distance,
                "R" => pos + distance,
                _ => panic!("Invalid turn direction: {direction}"),
            }
            .rem_euclid(100);
            let new_count = if new_pos == 0 { count + 1 } else { count };
            (new_pos, new_count)
        })
        .1
}

fn part_2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .fold((50, 0), |(pos, count), line| {
            let (direction, distance_str) = line.split_at(1);
            let distance: i32 = distance_str.parse().expect("Failed to parse distance");

            let full_laps = distance / 100;
            let distance_mod = distance % 100;

            let (new_pos, crossed_zero) = match direction {
                "L" => {
                    let new_pos = (pos - distance).rem_euclid(100);
                    let crossed = i32::from(distance_mod > pos && pos != 0);
                    (new_pos, crossed)
                }
                "R" => {
                    let new_pos = (pos + distance).rem_euclid(100);
                    let crossed = i32::from((pos + distance_mod) > 100);
                    (new_pos, crossed)
                }
                _ => panic!("Invalid turn direction: {direction}"),
            };

            let landed_on_zero = i32::from(new_pos == 0);
            let new_count = count + full_laps + crossed_zero + landed_on_zero;

            (new_pos, new_count)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 3);
        assert_eq!(part_1(INPUT_TXT), 1165);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 6);
        assert_eq!(part_2(INPUT_TXT), 6496);
    }
}
