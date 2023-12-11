static INPUT_TXT: &str = include_str!("../../input/11.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

struct Star {
    x: i64,
    y: i64,
}

impl Star {
    fn new(x: usize, y: usize) -> Self {
        let x = i64::try_from(x).expect("unable to convert x");
        let y = i64::try_from(y).expect("unable to convert y");
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn star_distance(stars: &[Star], expansion_factor: i64) -> i64 {
    let expansion_factor = expansion_factor - 1;
    let empty_columns = (0..stars.iter().map(|c| c.x).max().unwrap())
        .filter(|x| stars.iter().all(|c| c.x != *x))
        .collect::<Vec<_>>();
    let empty_rows = (0..stars.iter().map(|c| c.y).max().unwrap())
        .filter(|y| stars.iter().all(|c| c.y != *y))
        .collect::<Vec<_>>();

    stars.iter().enumerate().fold(0, |total, (idx, left_star)| {
        stars.iter().skip(idx + 1).fold(total, |total, right_star| {
            let empty_cols_count = i64::try_from(
                empty_columns
                    .iter()
                    .filter(|x| {
                        **x > i64::min(left_star.x, right_star.x)
                            && **x < i64::max(left_star.x, right_star.x)
                    })
                    .count(),
            )
            .unwrap();
            let empty_rows_count = i64::try_from(
                empty_rows
                    .iter()
                    .filter(|y| {
                        **y > i64::min(left_star.y, right_star.y)
                            && **y < i64::max(left_star.y, right_star.y)
                    })
                    .count(),
            )
            .unwrap();
            total
                + left_star.manhattan_distance(right_star)
                + (empty_cols_count + empty_rows_count) * expansion_factor
        })
    })
}

fn parse_input(input: &str) -> Vec<Star> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Star::new(x, y))
        })
        .collect()
}

fn part_1(input: &str) -> i64 {
    let stars = parse_input(input);
    star_distance(&stars, 2)
}

fn part_2(input: &str) -> i64 {
    let stars = parse_input(input);
    star_distance(&stars, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 374);
        assert_eq!(part_1(INPUT_TXT), 9_274_989);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 82_000_210);
        assert_eq!(part_2(INPUT_TXT), 357_134_560_737);
    }
}
