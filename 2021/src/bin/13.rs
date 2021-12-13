use itertools::Itertools;
use std::iter::FromIterator;

type Dots = Vec<(usize, usize)>;
type Instructions = Vec<(char, usize)>;

fn main() {
    let input = include_str!("../../input/13.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: \n{}", part_2(input));
}

fn parse_input(input: &str) -> (Dots, Instructions) {
    let (positions, instructions) = input.split_once("\n\n").unwrap();
    (
        positions
            .lines()
            .map(|lines| {
                let (x, y) = lines.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect(),
        instructions
            .lines()
            .map(|line| {
                let args = line.replace("fold along", "");
                let (l, n) = args.trim().split_once('=').unwrap();
                (l.parse::<char>().unwrap(), n.parse::<usize>().unwrap())
            })
            .collect(),
    )
}

fn fold(dots: Dots, folds: Instructions) -> Dots {
    dots.into_iter()
        .map(|pos| {
            folds.iter().fold(pos, |(x, y), &(l, n)| match l {
                'x' if x > n => ((2 * n) - x, y),
                'y' if y > n => (x, (2 * n) - y),
                _ => (x, y),
            })
        })
        .unique()
        .collect()
}

fn part_1(input: &str) -> usize {
    let (dots, mut folds) = parse_input(input);
    folds.truncate(1);
    fold(dots, folds).len()
}

fn part_2(input: &str) -> String {
    let (dots, folds) = parse_input(input);
    let folded_dots = fold(dots, folds);
    let (max_x, max_y) = folded_dots.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
        (max_x.max(x), max_y.max(y))
    });
    folded_dots
        .iter()
        .fold(
            vec![vec![' '; max_x + 1]; max_y + 1],
            |mut matrix, &(x, y)| {
                matrix[y][x] = '#';
                matrix
            },
        )
        .into_iter()
        .map(String::from_iter)
        .join("\n")
}

#[cfg(test)]
mod day_13_tests {
    use super::*;
    static INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 17);
        assert_eq!(part_1(include_str!("../../input/13.txt")), 790);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), "#####\n#   #\n#   #\n#   #\n#####");
        assert_eq!(
            "\n".to_owned() + &part_2(include_str!("../../input/13.txt")),
            "
###   ##  #  # #### ###  ####   ##  ## 
#  # #  # #  #    # #  # #       # #  #
#  # #    ####   #  ###  ###     # #   
###  # ## #  #  #   #  # #       # #   
#    #  # #  # #    #  # #    #  # #  #
#     ### #  # #### ###  #     ##   ## "
        );
    }
}
