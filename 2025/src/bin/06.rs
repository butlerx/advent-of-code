#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::time_execution;
static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("🌟 --- Day 6 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

fn part_1(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().collect();

    if lines.is_empty() {
        return 0;
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut problems = Vec::new();
    let mut col = 0;

    while col < width {
        if is_separator_column(&lines, col) {
            col += 1;
            continue;
        }

        let (problem, next_col) = extract_problem(&lines, col, width);
        if let Some(prob) = problem {
            problems.push(prob);
        }
        col = next_col;
    }

    problems.iter().map(|p| p.calculate()).sum()
}

fn is_separator_column(lines: &[&str], col: usize) -> bool {
    lines
        .iter()
        .all(|line| col >= line.len() || line.chars().nth(col).unwrap_or(' ') == ' ')
}

fn extract_problem(lines: &[&str], start_col: usize, width: usize) -> (Option<Problem>, usize) {
    // Find the width of this problem (until we hit a separator column or end)
    let mut end_col = start_col;
    while end_col < width && !is_separator_column(lines, end_col) {
        end_col += 1;
    }

    if start_col >= end_col {
        return (None, end_col);
    }

    // Extract the problem data
    let mut numbers = Vec::new();
    let mut operator = None;

    for line in lines {
        let segment: String = line
            .chars()
            .skip(start_col)
            .take(end_col - start_col)
            .collect();

        let trimmed = segment.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Check if it's an operator
        if trimmed == "*" || trimmed == "+" {
            operator = Some(trimmed.to_string());
        } else if let Ok(num) = trimmed.parse::<usize>() {
            numbers.push(num);
        }
    }

    if !numbers.is_empty() && operator.is_some() {
        (
            Some(Problem {
                numbers,
                operator: operator.unwrap(),
            }),
            end_col,
        )
    } else {
        (None, end_col)
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operator: String,
}

impl Problem {
    fn calculate(&self) -> usize {
        if self.numbers.is_empty() {
            return 0;
        }

        match self.operator.as_str() {
            "+" => self.numbers.iter().sum(),
            "*" => self.numbers.iter().product(),
            _ => 0,
        }
    }
}

fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 4277556);
        assert_eq!(part_1(INPUT_TXT), 6417439773370);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 3263827);
        assert_eq!(part_2(INPUT_TXT), 0);
    }
}
