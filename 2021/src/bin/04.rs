use itertools::Itertools;

static BOARD_SIZE: usize = 5;

fn main() {
    let input = include_str!("../../input/04.txt");
    let (part1, part2) = run(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

pub fn run(input: &str) -> (i64, i64) {
    let numbers: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect();

    let boards: Vec<Vec<i64>> = input
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|board| {
            board
                .iter()
                .flat_map(|&row| row.split_whitespace())
                .map(|tile| tile.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let mut scoreboard = boards
        .iter()
        .filter_map(|board| {
            (0..numbers.len())
                .into_iter()
                .map(|i| &numbers[0..=i])
                .find(|drawn| has_won(board, drawn))
                .map(|winning_draws| (board, winning_draws))
        })
        .sorted_by(|(_, x), (_, y)| x.len().cmp(&y.len()));
    (
        checksum(scoreboard.next().unwrap()),
        checksum(scoreboard.last().unwrap()),
    )
}

fn has_won(board: &[i64], numbers: &[i64]) -> bool {
    let matches: Vec<bool> = board.iter().map(|tile| numbers.contains(tile)).collect();

    let any_row_won = matches
        .chunks(BOARD_SIZE)
        .map(|row| row.iter().all(|tile| *tile))
        .any(|row| row);

    any_row_won
        || (0..BOARD_SIZE)
            .into_iter()
            .map(|offset| {
                matches
                    .iter()
                    .skip(offset)
                    .step_by(BOARD_SIZE)
                    .all(|tile| *tile)
            })
            .any(|col| col)
}

fn checksum((board, numbers): (&Vec<i64>, &[i64])) -> i64 {
    board
        .iter()
        .filter(|tile| !numbers.contains(tile))
        .sum::<i64>()
        * numbers.last().unwrap()
}

#[cfg(test)]
mod day_4_tests {
    use super::*;
    static INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_small_input() {
        let (part1, part2) = run(INPUT);
        assert_eq!(part1, 4512);
        assert_eq!(part2, 1924);
    }

    #[test]
    fn test_large_input() {
        let (p1, p2) = run(include_str!("../../input/04.txt"));
        assert_eq!(p1, 49860);
        assert_eq!(p2, 24628);
    }
}
