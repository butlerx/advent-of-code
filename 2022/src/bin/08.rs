static INPUT_TXT: &str = include_str!("../../input/08.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let rows = grid.len() - 1;
    let cols = grid[0].len() - 1;
    (grid, rows, cols)
}

fn part_1(input: &str) -> u32 {
    let (grid, rows, cols) = parse_input(input);
    (1..rows).fold((2 * (rows + cols)) as u32, |visible, row| {
        (1..cols).fold(visible, |visible, col| {
            let height = grid[row][col];
            if (0..col).all(|i| grid[row][i] < height)
                || (0..row).all(|j| grid[j][col] < height)
                || (col + 1..=cols).all(|i| grid[row][i] < height)
                || (row + 1..=rows).all(|j| grid[j][col] < height)
            {
                visible + 1
            } else {
                visible
            }
        })
    })
}

fn part_2(input: &str) -> u32 {
    let (grid, rows, cols) = parse_input(input);
    (1..rows).fold(0u32, |score, row| {
        (1..cols).fold(score, |score, col| {
            let height = grid[row][col];
            let left = (0..col)
                .enumerate()
                .rev()
                .find(|(_, c)| grid[row][*c] >= height )
                .map(|(i, _)| col-i )
                .unwrap_or(col);
            let right = (col + 1..=cols)
                .enumerate()
                .find(|(_, c)| grid[row][*c] >= height )
                .map(|(i, _)| i+1 )
                .unwrap_or(cols - col);
            let up = (0..row)
                .enumerate()
                .rev()
                .find(|(_, r)| grid[*r][col] >= height )
                .map(|(i, _)| row-i )
                .unwrap_or(row);
            let down = (row + 1..=rows)
                .enumerate()
                .find(|(_, r)| grid[*r][col] >= height )
                .map(|(i, _)| i+1 )
                .unwrap_or(rows - row);
            score.max((left * up * right * down) as u32)
        })
    })
}

#[cfg(test)]
mod day_8_tests {
    use super::*;
    static INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 21);
        assert_eq!(part_1(INPUT_TXT), 1662);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 8);
        assert_eq!(part_2(INPUT_TXT), 537600);
    }
}
