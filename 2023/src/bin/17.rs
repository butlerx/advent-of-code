use std::collections::{BinaryHeap, HashMap};

static INPUT_TXT: &str = include_str!("../../input/17.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|c| {
            c.chars()
                .map(|c| {
                    c.to_digit(10).map_or(0, |digit| {
                        i32::try_from(digit).expect("failed to convert char to int")
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn dijkstra(grid: &[Vec<i32>], min_step: isize, max_step: isize) -> i32 {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    loop {
        let (cost, (row, col, dist)) = queue.pop().expect("queue is empty");
        if (row, col) == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }
        if dists
            .get(&(row, col, dist))
            .filter(|&&c| -cost > c)
            .is_some()
        {
            continue;
        }
        for (delta_row, delta_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if dist == (delta_row, delta_col) || dist == (-delta_row, -delta_col) {
                continue;
            }
            let mut next_cost = -cost;
            for steps in 1..=max_step {
                let next_row = (isize::try_from(row).unwrap() + delta_row * steps) as usize;
                let next_col = (isize::try_from(col).unwrap() + delta_col * steps) as usize;
                if next_row >= grid.len() || next_col >= grid[0].len() {
                    continue;
                }
                next_cost += grid[next_row][next_col];
                let key = (next_row, next_col, (delta_row, delta_col));
                if min_step <= steps && next_cost < *dists.get(&key).unwrap_or(&10_000_000) {
                    dists.insert(key, next_cost);
                    queue.push((-next_cost, key));
                }
            }
        }
    }
}

fn part_1(input: &str) -> i32 {
    let grid = parse_input(input);
    dijkstra(&grid, 1, 3)
}

fn part_2(input: &str) -> i32 {
    let grid = parse_input(input);
    dijkstra(&grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    static INPUT_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 102);
        assert_eq!(part_1(INPUT_2), 59);
        assert_eq!(part_1(INPUT_TXT), 959);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 94);
        assert_eq!(part_2(INPUT_2), 71);
        assert_eq!(part_2(INPUT_TXT), 1135);
    }
}
