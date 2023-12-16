static INPUT_TXT: &str = include_str!("../../input/16.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn step(row: usize, col: usize, dir: usize) -> (usize, usize, usize) {
    let (dr, dc) = match dir {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        _ => (0, -1),
    };
    (
        (isize::try_from(row).unwrap() + dr) as _,
        (isize::try_from(col).unwrap() + dc) as _,
        dir,
    )
}

fn energized_tiles(grid: &[Vec<char>], start: (usize, usize, usize)) -> usize {
    let mut seen = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
    let mut beams = vec![start];
    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .filter(|&(row, col, dir)| {
                if r < grid.len() && c < grid[0].len() && !seen[row][col][dir] {
                    seen[r][c][d] = true;
                    true
                } else {
                    false
                }
            })
            .flat_map(|(row, col, dir)| match (grid[row][col], dir) {
                ('.', _) | ('|', 0 | 2) | ('-', 1 | 3) => vec![step(row, col, dir)],
                ('\\', _) => vec![step(row, col, [3, 2, 1, 0][dir])],
                ('/', _) => vec![step(row, col, [1, 0, 3, 2][dir])],
                ('|', _) => vec![step(row, col, 0), step(row, col, 2)],
                ('-', _) => vec![step(row, col, 1), step(row, col, 3)],
                _ => unreachable!(),
            })
            .collect();
    }

    seen.iter()
        .flatten()
        .filter(|x| x.iter().any(|&b| b))
        .count()
}

fn part_1(input: &str) -> usize {
    energized_tiles(&parse_input(input), (0, 0, 1))
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    (0..grid.len())
        .flat_map(|r| [(r, 0, 1), (r, grid[0].len() - 1, 3)])
        .chain((0..grid[0].len()).flat_map(|c| [(0, c, 2), (grid.len() - 1, c, 0)]))
        .map(|start| energized_tiles(&grid, start))
        .max()
        .expect("failed to get max")
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 46);
        assert_eq!(part_1(INPUT_TXT), 6514);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 51);
        assert_eq!(part_2(INPUT_TXT), 8089);
    }
}
