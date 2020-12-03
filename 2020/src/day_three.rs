use std::io::Error;

pub fn run(input: String, right: i64, down: i64) -> Result<i64, Error> {
    let tree = '#';
    let (num_tree, _right_count) =
        input
            .lines()
            .step_by(down as usize)
            .fold((0, 0), |(num_tree, right_count), line| {
                let mut chars = line.chars();
                let len = chars.clone().collect::<Vec<_>>().len();
                (
                    num_tree
                        + if chars.nth(right_count).unwrap() == tree {
                            1
                        } else {
                            0
                        },
                    (right_count + right as usize) % len,
                )
            });
    Ok(num_tree)
}

pub fn run_multiply(input: String) -> Result<i64, Error> {
    Ok(run(input.clone(), 1, 1).unwrap()
        * run(input.clone(), 3, 1).unwrap()
        * run(input.clone(), 5, 1).unwrap()
        * run(input.clone(), 7, 1).unwrap()
        * run(input.clone(), 1, 2).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

    #[test]
    fn test_run_multiply() {
        assert!(run_multiply(INPUT.to_string()).unwrap() == 336);
    }

    #[test]
    fn test_run() {
        assert!(run(INPUT.to_string(), 1, 1).unwrap() == 2);
        assert!(run(INPUT.to_string(), 3, 1).unwrap() == 7);
        assert!(run(INPUT.to_string(), 5, 1).unwrap() == 3);
        assert!(run(INPUT.to_string(), 7, 1).unwrap() == 4);
        assert!(run(INPUT.to_string(), 1, 2).unwrap() == 2);
    }
}
