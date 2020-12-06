pub fn run(input: &str, right: usize, down: usize) -> i64 {
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

pub fn part_1(input: &str) -> i64 {
    run(input, 3, 1)
}

pub fn part_2(input: &str) -> i64 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| run(input, right, down))
        .product::<i64>()
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
        assert!(part_1(INPUT) == 7);
        assert!(part_1(include_str!("../input/day_3.txt")) == 250);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(INPUT) == 336);
        assert!(part_2(include_str!("../input/day_3.txt")) == 1592662500);
    }

    #[test]
    fn test_run() {
        assert!(run(INPUT, 1, 1) == 2);
        assert!(run(INPUT, 3, 1) == 7);
        assert!(run(INPUT, 5, 1) == 3);
        assert!(run(INPUT, 7, 1) == 4);
        assert!(run(INPUT, 1, 2) == 2);
    }
}
