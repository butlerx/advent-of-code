#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_2024::time_execution;

static INPUT_TXT: &str = include_str!("../../input/13.txt");

fn main() {
    println!("🌟 --- Day 13 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Button {
    x: i64,
    y: i64,
}

impl From<&str> for Button {
    fn from(s: &str) -> Self {
        let (_, distance) = s
            .strip_prefix("Button ")
            .expect("Expected button line to start with \"Button \"")
            .split_once(": ")
            .expect("Expected button line to contain a colon");

        let (x, y) = distance
            .split_once(", ")
            .expect("Expected button distance to contain a comma");
        let x = x
            .strip_prefix("X+")
            .expect("A buttons x distance should be given with \"X+\"")
            .parse()
            .expect("Failed to parse x distance");
        let y = y
            .strip_prefix("Y+")
            .expect("A buttons y distance should be given with \"Y+\"")
            .parse()
            .expect("Failed to parse y distance");

        Self { x, y }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClawMachine {
    a: Button,
    b: Button,
    x: i64,
    y: i64,
}

impl ClawMachine {
    #[must_use]
    pub fn cost(&self) -> Option<i64> {
        let b_top = self.x * self.a.y - self.a.x * self.y;
        let b_bottom = self.b.x * self.a.y - self.a.x * self.b.y;
        let b = if b_top % b_bottom != 0 {
            return None;
        } else {
            b_top / b_bottom
        };

        let a_top = self.y - b * self.b.y;
        let a_bottom = self.a.y;
        let a = if a_top % a_bottom != 0 {
            return None;
        } else {
            a_top / a_bottom
        };

        Some(a * 3 + b)
    }
}

impl From<&str> for ClawMachine {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().map(str::trim);
        let a_button = lines
            .next()
            .map(Button::from)
            .expect("Failed to parse button A");
        let b_button = lines
            .next()
            .map(Button::from)
            .expect("Failed to parse button B");
        let prize = lines.next().expect("Failed to parse the prize");

        let location = prize
            .strip_prefix("Prize: ")
            .map(str::trim)
            .expect("Expected \"Prize: \" label in the third row");
        let (x, y) = location
            .split_once(", ")
            .expect("Expected prize location to contain a comma");
        let x = x
            .strip_prefix("X=")
            .expect("price x location should be given with \"X=\"")
            .parse()
            .expect("Failed to parse x location");
        let y = y
            .strip_prefix("Y=")
            .expect("price y location should be given with \"Y=\"")
            .parse()
            .expect("Failed to parse y location");

        Self {
            a: a_button,
            b: b_button,
            x,
            y,
        }
    }
}

fn part_1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(ClawMachine::from)
        .filter_map(|c| c.cost())
        .sum()
}

fn part_2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(ClawMachine::from)
        .map(|mut claw_machine| {
            claw_machine.x += 10_000_000_000_000;
            claw_machine.y += 10_000_000_000_000;
            claw_machine
        })
        .filter_map(|c| c.cost())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 480);
        assert_eq!(part_1(INPUT_TXT), 28059);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 875_318_608_908);
        assert_eq!(part_2(INPUT_TXT), 102_255_878_088_512);
    }
}
