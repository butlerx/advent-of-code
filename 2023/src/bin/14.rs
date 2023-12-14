use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/14.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    (0..grid[0].len()).fold(0, |mut total, col| {
        (0..grid.len()).fold(0, |next, row| match grid[row].get(col) {
            Some('O') => {
                total += grid.len() - next;
                next + 1
            }
            Some('#') => row + 1,
            _ => next,
        });
        total
    })
}

fn roll_north(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    loop {
        let mut done = true;
        for row in 0..grid.len() - 1 {
            for col in 0..grid[row].len() {
                if grid[row + 1][col] == 'O' && grid[row][col] == '.' {
                    grid[row][col] = 'O';
                    grid[row + 1][col] = '.';
                    done = false;
                }
            }
        }
        if done {
            break grid;
        }
    }
}

fn rotate(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    grid.iter().enumerate().fold(
        vec![vec!['.'; grid.len()]; grid[0].len()],
        |mut newgrid, (r, row)| {
            for (c, &char) in row.iter().enumerate() {
                newgrid[c][grid.len() - 1 - r] = char;
            }
            newgrid
        },
    )
}

fn part_2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut seen = HashMap::new();
    for i in 1..1_000_000_000 {
        grid = (0..4).fold(grid, |grid, _| rotate(&roll_north(grid)));
        if let Some(seen_at) = seen.insert(grid.clone(), i) {
            if (1_000_000_000 - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }

    grid.iter()
        .enumerate()
        .map(|(height, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'O')
                .map(|_| grid.len() - height)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 136);
        assert_eq!(part_1(INPUT_TXT), 106_186);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 64);
        assert_eq!(part_2(INPUT_TXT), 106_390);
    }
}
