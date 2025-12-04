#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use aoc_shared::{Grid, Point, time_execution};
use std::collections::{HashSet, VecDeque};

static INPUT_TXT: &str = include_str!("../../input/12.txt");

fn main() {
    println!("🌟 --- Day 12 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

const DIR: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];

fn parse_input(input: &str) -> Grid<char> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_neighbors(pos: Point, grid: &Grid<char>, plant: char) -> Vec<Point> {
    DIR.into_iter()
        .filter_map(|dir| {
            let neighbor_pos = pos + dir;
            match grid.get(neighbor_pos) {
                Some(cell) if cell == plant => Some(neighbor_pos),
                _ => None,
            }
        })
        .collect()
}

fn bfs(pos: Point, visited: &mut HashSet<Point>, grid: &Grid<char>) -> (HashSet<Point>, usize) {
    let mut queue = VecDeque::from([pos]);
    let mut area = HashSet::default();
    let mut perimeter = 0;
    let plant = grid.get(pos).expect("invalid position");
    visited.insert(pos);

    while let Some(curr) = queue.pop_front() {
        area.insert(curr);
        let neighbors = get_neighbors(curr, grid, plant);
        perimeter += 4 - neighbors.len();

        let todo = neighbors
            .into_iter()
            .filter(|neighbor| !visited.contains(neighbor))
            .collect::<Vec<_>>();

        visited.extend(todo.clone());
        queue.extend(todo);
    }

    (area, perimeter)
}

fn count_region_sides(region: &HashSet<Point>) -> usize {
    DIR.into_iter()
        .map(|dir| {
            let sides: HashSet<Point> = region
                .iter()
                .map(|&pos| pos + dir)
                .filter(|&pos| !region.contains(&pos))
                .collect();

            let mut remove: HashSet<Point> = HashSet::default();

            for &side in &sides {
                let mut tmp = Point {
                    x: side.x + dir.y,
                    y: side.y + dir.x,
                };
                while sides.contains(&tmp) {
                    remove.insert(tmp);
                    tmp = Point {
                        x: tmp.x + dir.y,
                        y: tmp.y + dir.x,
                    };
                }
            }

            sides.len() - remove.len()
        })
        .sum()
}

fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut visited = HashSet::default();

    (0..grid.height)
        .flat_map(move |y| (0..grid.width).map(move |x| Point::from((x, y))))
        .filter_map(|pos| {
            if visited.contains(&pos) {
                return None;
            }
            let (region_area, region_perim) = bfs(pos, &mut visited, &grid);
            Some(region_area.len() * region_perim)
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut visited = HashSet::default();

    (0..grid.height)
        .flat_map(move |y| (0..grid.width).map(move |x| Point::from((x, y))))
        .filter_map(|pos| {
            if visited.contains(&pos) {
                return None;
            }
            let (region_area, _) = bfs(pos, &mut visited, &grid);
            let sides = count_region_sides(&region_area);
            Some(region_area.len() * sides)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "AAAA
BBCD
BBCC
EEEC";
    static INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    static INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    static INPUT_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    static INPUT_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 140);
        assert_eq!(part_1(INPUT_2), 772);
        assert_eq!(part_1(INPUT_3), 1930);
        assert_eq!(part_1(INPUT_4), 692);
        assert_eq!(part_1(INPUT_5), 1184);
        assert_eq!(part_1(INPUT_TXT), 1_546_338);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 80);
        assert_eq!(part_2(INPUT_2), 436);
        assert_eq!(part_2(INPUT_3), 1206);
        assert_eq!(part_2(INPUT_4), 236);
        assert_eq!(part_2(INPUT_5), 368);
        assert_eq!(part_2(INPUT_TXT), 978_590);
    }
}
