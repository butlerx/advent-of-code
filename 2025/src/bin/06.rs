#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{Grid, time_execution_us};
static INPUT_TXT: &str = include_str!("../../input/06.txt");

fn main() {
    println!("🌟 --- Day 6 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} us");

    let (res_2, duration_2) = time_execution_us(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} us");
}

fn part_1(input: &str) -> usize {
    solve_problems(input, extract_problem_horizontal)
}

fn part_2(input: &str) -> usize {
    solve_problems(input, extract_problem_vertical)
}

fn solve_problems<F>(input: &str, extractor: F) -> usize
where
    F: Fn(&Grid<char>, usize) -> (Option<Problem>, usize),
{
    let grid = parse_grid(input);

    std::iter::successors(Some(0), |&col| (col < grid.width).then_some(col))
        .scan(0, |current_col, _| {
            if *current_col >= grid.width {
                return None;
            }

            if is_separator_column(&grid, *current_col) {
                *current_col += 1;
                Some(None)
            } else {
                let (problem, next_col) = extractor(&grid, *current_col);
                *current_col = next_col;
                Some(problem)
            }
        })
        .flatten()
        .map(Problem::calculate)
        .sum()
}

fn parse_grid(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let rows: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    Grid::from(rows)
}

fn is_separator_column(grid: &Grid<char>, col: usize) -> bool {
    (0..grid.height).all(|row| grid.get((col, row).into()).is_none_or(|ch| ch == ' '))
}

fn extract_problem_horizontal(grid: &Grid<char>, start_col: usize) -> (Option<Problem>, usize) {
    let end_col = find_next_separator(grid, start_col);

    let (numbers, operator) = (0..grid.height)
        .filter_map(|row| parse_row_segment(grid, row, start_col, end_col))
        .fold(
            (Vec::new(), None),
            |(mut numbers, operator), entry| match entry {
                Entry::Number(n) => {
                    numbers.push(n);
                    (numbers, operator)
                }
                Entry::Operator(op) => (numbers, Some(op)),
            },
        );

    let problem = operator.map(|op| Problem {
        numbers,
        operator: op,
    });
    (problem, end_col)
}

fn extract_problem_vertical(grid: &Grid<char>, start_col: usize) -> (Option<Problem>, usize) {
    let end_col = find_next_separator(grid, start_col);

    let (mut numbers, operator) =
        (start_col..end_col).fold((Vec::new(), None::<char>), |(mut nums, op), col| {
            let (digits, col_op) =
                (0..grid.height).fold((String::new(), op), |(mut digits, op), row| {
                    match grid.get((col, row).into()) {
                        Some(ch) if ch == '*' || ch == '+' => (digits, Some(ch)),
                        Some(ch) if ch != ' ' => {
                            digits.push(ch);
                            (digits, op)
                        }
                        _ => (digits, op),
                    }
                });

            if let Ok(num) = digits.parse::<usize>() {
                nums.push(num);
            }
            (nums, col_op)
        });

    numbers.reverse();
    let problem = operator.map(|op| Problem {
        numbers,
        operator: op,
    });
    (problem, end_col)
}

fn find_next_separator(grid: &Grid<char>, start_col: usize) -> usize {
    (start_col..grid.width)
        .find(|&col| is_separator_column(grid, col))
        .unwrap_or(grid.width)
}

fn parse_row_segment(
    grid: &Grid<char>,
    row: usize,
    start_col: usize,
    end_col: usize,
) -> Option<Entry> {
    let segment: String = (start_col..end_col)
        .filter_map(|col| grid.get((col, row).into()))
        .collect();

    match segment.trim() {
        "*" => Some(Entry::Operator('*')),
        "+" => Some(Entry::Operator('+')),
        s => s.parse::<usize>().ok().map(Entry::Number),
    }
}

#[derive(Debug)]
enum Entry {
    Number(usize),
    Operator(char),
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operator: char,
}

impl Problem {
    fn calculate(self) -> usize {
        match self.operator {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => 0,
        }
    }
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
        assert_eq!(part_1(TEST_INPUT), 4_277_556);
        assert_eq!(part_1(INPUT_TXT), 6_417_439_773_370);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 3_263_827);
        assert_eq!(part_2(INPUT_TXT), 11_044_319_475_191);
    }
}
