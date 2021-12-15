use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../../input/15.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn shortest_path(map: Vec<Vec<i64>>) -> i64 {
    let success = (map.len() - 1, map[0].len() - 1);
    let mut costs = vec![vec![i64::MAX; map[0].len()]; map.len()];
    let mut to_see = BinaryHeap::new();
    to_see.push((0, 0, 0));
    loop {
        let (cost, x, y) = to_see.pop().unwrap();
        if (x, y) == success {
            break -cost;
        }
        if -cost < costs[x][y] {
            continue;
        }
        for (x1, y1) in get_neighbours(x, y) {
            if map.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }
            let next_cost = -cost + map[x1][y1];
            if next_cost < costs[x1][y1] {
                to_see.push((-next_cost, x1, y1));
                costs[x1][y1] = next_cost;
            }
        }
    }
}

fn get_neighbours(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y + 1),
        (x, y.saturating_sub(1)),
    ]
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.bytes().map(|c| (c - b'0') as i64).collect())
        .collect()
}

fn part_1(input: &str) -> i64 {
    let map = parse_input(input);
    shortest_path(map)
}

fn part_2(input: &str) -> i64 {
    let map = parse_input(input);
    let (num_row, num_col) = (map.len(), map[0].len());
    let expanded_map = (0..(5 * map.len()))
        .map(|row| {
            (0..(5 * map[0].len()))
                .map(|col| {
                    let increase = ((row / num_row) + (col / num_col) - 1) as i64;
                    ((map[row % num_row][col % num_col] + increase) % 9) + 1
                })
                .collect()
        })
        .collect();
    shortest_path(expanded_map)
}

#[cfg(test)]
mod day_15_tests {
    use super::*;
    static INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 40);
        assert_eq!(part_1(include_str!("../../input/15.txt")), 583);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 315);
        assert_eq!(part_2(include_str!("../../input/15.txt")), 2927);
    }
}
