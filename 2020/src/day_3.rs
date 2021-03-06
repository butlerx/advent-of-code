fn find_path(input: &str, right: usize, down: usize) -> i64 {
    let tree = '#';
    let (num_tree, _right_count) =
        input
            .lines()
            .step_by(down)
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
                    (right_count + right) % len,
                )
            });
    num_tree
}

pub fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(right, down)| find_path(input, right, down))
            .product::<i64>()
    } else {
        find_path(input, 3, 1)
    }
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
    fn test_part_1() {
        assert!(run(INPUT, false) == 7);
        assert!(run(include_str!("../input/day_3.txt"), false) == 250);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 336);
        assert!(run(include_str!("../input/day_3.txt"), true) == 1592662500);
    }

    #[test]
    fn test_run() {
        assert!(find_path(INPUT, 1, 1) == 2);
        assert!(find_path(INPUT, 3, 1) == 7);
        assert!(find_path(INPUT, 5, 1) == 3);
        assert!(find_path(INPUT, 7, 1) == 4);
        assert!(find_path(INPUT, 1, 2) == 2);
    }
}
