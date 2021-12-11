use itertools::Itertools;
use std::collections::HashMap;

type Cell = (usize, usize);
type Grid = HashMap<Cell, usize>;

fn main() {
    let input = include_str!("../../input/11.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x, y), c.to_string().parse::<usize>().unwrap()))
                .collect::<Vec<(Cell, usize)>>()
        })
        .flatten()
        .collect()
}

fn step(grid: &mut Grid) -> usize {
    let mut total_flashed = 0;
    let mut queue = Itertools::cartesian_product(0..10, 0..10).collect_vec();

    while let Some(cell) = queue.pop() {
        let flashes = grid.get_mut(&cell).unwrap();
        *flashes += 1;
        if *flashes == 10 {
            total_flashed += 1;
            queue.append(&mut get_neighbours(cell))
        }
    }

    for flashes in grid.values_mut() {
        if *flashes > 9 {
            *flashes = 0;
        }
    }

    total_flashed
}

fn get_neighbours((x, y): Cell) -> Vec<Cell> {
    Itertools::cartesian_product(
        x.saturating_sub(1)..=(x + 1).min(9),
        y.saturating_sub(1)..=(y + 1).min(9),
    )
    .filter(|cell| cell.0 != x || cell.1 != y)
    .collect()
}

fn part_1(input: &str) -> usize {
    let mut octos: Grid = parse_input(input);
    (1..=100).fold(0, |total_flashes, _| total_flashes + step(&mut octos))
}

fn part_2(input: &str) -> usize {
    let mut octos: Grid = parse_input(input);
    let mut step_counter = 0;
    loop {
        step_counter += 1;
        if step(&mut octos) == 100 {
            break step_counter;
        }
    }
}

#[cfg(test)]
mod day_11_tests {
    use super::*;
    static INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1656);
        assert_eq!(part_1(include_str!("../../input/11.txt")), 1694);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 195);
        assert_eq!(part_2(include_str!("../../input/11.txt")), 346);
    }
}
