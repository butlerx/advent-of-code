use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/03.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Point = (usize, usize);

#[derive(Debug)]
struct Number {
    value: u32,
    x_range: (usize, usize),
    y: usize,
}

impl Number {
    fn new(num: &str, (x, y): Point) -> Self {
        let value = num.parse::<u32>().expect("Invalid number");
        let start_x = x + 1 - num.len();
        Self {
            value,
            x_range: (start_x, x),
            y,
        }
    }

    fn is_adjacent(&self, (x, y): &Point) -> bool {
        !(self.y.abs_diff(*y) > 1
            || *x < self.x_range.0.saturating_sub(1)
            || *x > self.x_range.1 + 1)
    }
}

fn parse_input(input: &str) -> (Vec<Number>, HashMap<Point, char>) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    for (pos_y, l) in input.trim().lines().enumerate() {
        let mut num = String::new();
        for (pos_x, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
                if c != '.' {
                    symbols.insert((pos_x, pos_y), c);
                }
                if !num.is_empty() {
                    numbers.push(Number::new(&num, (pos_x - 1, pos_y)));
                    num.clear();
                }
            }
        }

        if !num.is_empty() {
            numbers.push(Number::new(&num, (l.len() - 1, pos_y)));
        }
    }
    (numbers, symbols)
}

fn part_1(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);
    symbols.keys().map(|pos| {
            numbers
                .iter()
                .filter(|n| n.is_adjacent(pos))
                .map(|n| n.value)
                .sum::<u32>()
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);
    symbols
        .iter()
        .map(|(pos, c)| {
            if *c == '*' {
                let neighbours = numbers
                    .iter()
                    .filter(|n| n.is_adjacent(pos))
                    .map(|n| n.value)
                    .collect::<Vec<u32>>();
                if neighbours.len() == 2 {
                    neighbours.into_iter().product()
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4361);
        assert_eq!(part_1(INPUT_TXT), 527_369);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 467_835);
        assert_eq!(part_2(INPUT_TXT), 73_074_886);
    }
}
