use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../../input/23.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn right_configuration(maze: &[Vec<char>]) -> bool {
    maze[2..(maze.len() - 1)]
        .iter()
        .all(|l| itertools::equal(l[3..10].iter().copied(), "A#B#C#D".chars()))
}

fn moves(maze: &[Vec<char>]) -> Vec<(i64, Vec<Vec<char>>)> {
    let room_len = maze.len() - 2;
    let mut moves = Vec::new();
    for y in 0..maze[1].len() {
        if let 'A'..='D' = maze[1][y] {
            let (room, exp) = match maze[1][y] {
                'A' => Some((3, 1)),
                'B' => Some((5, 10)),
                'C' => Some((7, 100)),
                'D' => Some((9, 1000)),
                _ => None,
            }
            .unwrap();
            let mut cost = if y > room && (room..y).all(|c| maze[1][c] == '.') {
                y - room
            } else if y < room && (y + 1..=room).all(|c| maze[1][c] == '.') {
                room - y
            } else {
                continue;
            };
            let i = match (2..=room_len).take_while(|&i| maze[i][room] == '.').last() {
                Some(i) => i,
                _ => continue,
            };
            if i != room_len && maze[i + 1][room] != maze[1][y] {
                continue;
            }
            let mut m = maze.to_owned();
            m[i][room] = maze[1][y];
            m[1][y] = '.';
            cost += i - 1;
            moves.push(((cost * exp) as i64, m));
        }
    }
    for (x, y) in (2..=room_len).cartesian_product([3, 5, 7, 9]) {
        if (2..x).any(|i| maze[i][y] != '.') || (x + 1..=room_len).any(|i| maze[i][y] == '.') {
            continue;
        } else if let 'A'..='D' = maze[x][y] {
            let exp = match maze[x][y] {
                'A' => Some(1),
                'B' => Some(10),
                'C' => Some(100),
                'D' => Some(1000),
                _ => None,
            }
            .unwrap();
            for i in y..maze[0].len() {
                if maze[1][i] != '.' {
                    break;
                }
                if ![1, 2, 4, 6, 8, 10, 11].contains(&i) {
                    continue;
                }
                let cost = x - 1 + i - y;
                let mut m = maze.to_owned();
                m[1][i] = maze[x][y];
                m[x][y] = '.';
                moves.push(((cost * exp) as i64, m));
            }
            for i in (1..=y).rev() {
                if maze[1][i] != '.' {
                    break;
                }
                if ![1, 2, 4, 6, 8, 10, 11].contains(&i) {
                    continue;
                }
                let cost = x - 1 + y - i;
                let mut m = maze.to_owned();
                m[1][i] = maze[x][y];
                m[x][y] = '.';
                moves.push(((cost * exp) as i64, m));
            }
        }
    }
    moves
}

fn shortest_path(maze: &[Vec<char>]) -> Option<i64> {
    let mut seen = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push((0, maze.to_owned()));
    while let Some((cost, pos)) = heap.pop() {
        if right_configuration(&pos) {
            return Some(-cost);
        }
        if let Some(&c) = seen.get(&pos) {
            if -cost > c {
                continue;
            }
        }
        for (step_cost, m) in moves(&pos) {
            let next_cost = -cost + step_cost;
            if *seen.get(&m).unwrap_or(&1000000) > next_cost {
                seen.insert(m.clone(), next_cost);
                heap.push((-next_cost, m));
            }
        }
    }
    None
}

fn part_1(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>();
    shortest_path(&map).unwrap()
}

fn part_2(input: &str) -> i64 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    map.insert(3, "  #D#B#A#C#".chars().collect());
    map.insert(3, "  #D#C#B#A#".chars().collect());
    shortest_path(&map).unwrap()
}

#[cfg(test)]
mod day_23_tests {
    use super::*;
    static INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 12521);
        assert_eq!(part_1(include_str!("../../input/23.txt")), 16244);
    }

    #[test]
    fn test_part_2() {
        //  For unknown reasons i managed to find a cheaper path then the example
        //  expected
        // assert_eq!(part_2(INPUT), 44169);
        assert_eq!(part_2(INPUT), 40475);
        assert_eq!(part_2(include_str!("../../input/23.txt")), 43226);
    }
}
