static INPUT_TXT: &str = include_str!("../../input/04.txt");

fn main() {
    println!("🌟 --- Day 3 Results --- 🌟");
    println!("📌 Part 1: {}", part_1(INPUT_TXT));
    println!("📌 Part 2: {}", part_2(INPUT_TXT));
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

const DIRECTIONS: [Point; 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn parse_input(input: &str) -> Grid {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part_1(input: &str) -> u32 {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] == 'X')
        .map(|(row, col)| {
            DIRECTIONS
                .iter()
                .filter(|(dx, dy)| {
                    WORD.iter().enumerate().all(|(i, item)| {
                        let new_row = (row as i32 + dx * i as i32) as usize;
                        let new_col = (col as i32 + dy * i as i32) as usize;
                        new_row < rows && new_col < cols && grid[new_row][new_col] == *item
                    })
                })
                .count() as u32
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let grid = parse_input(input);

    (1..grid.len() - 1)
        .flat_map(|r| (1..grid[0].len() - 1).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] == 'A')
        .map(|(r, c)| {
            [
                (
                    (r - 1, c - 1, 'M', r + 1, c + 1, 'S'),
                    (r - 1, c + 1, 'M', r + 1, c - 1, 'S'),
                ),
                (
                    (r - 1, c - 1, 'M', r + 1, c + 1, 'S'),
                    (r + 1, c - 1, 'M', r - 1, c + 1, 'S'),
                ),
                (
                    (r + 1, c + 1, 'M', r - 1, c - 1, 'S'),
                    (r + 1, c - 1, 'M', r - 1, c + 1, 'S'),
                ),
                (
                    (r + 1, c + 1, 'M', r - 1, c - 1, 'S'),
                    (r - 1, c + 1, 'M', r + 1, c - 1, 'S'),
                ),
            ]
            .iter()
            .any(|&(p1, p2)| check_pattern(&grid, p1) && check_pattern(&grid, p2))
                as u32
        })
        .sum()
}

fn check_pattern(
    grid: &Grid,
    (r1, c1, m, r2, c2, s): (usize, usize, char, usize, usize, char),
) -> bool {
    grid[r1][c1] == m && grid[r2][c2] == s
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 18);
        assert_eq!(part_1(INPUT_TXT), 2454);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 9);
        assert_eq!(part_2(INPUT_TXT), 1858);
    }
}
