use std::collections::{HashMap, VecDeque};

type Coordinate = (isize, isize);

fn main() {
    let input = include_str!("../../input/09.txt");
    let (part_1, part_2) = run(input);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

pub fn run(input: &str) -> (usize, usize) {
    let mut map: HashMap<Coordinate, usize> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x as isize, y as isize),
                        c.to_string().parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(Coordinate, usize)>>()
        })
        .flatten()
        .collect();
    let mut low_points: Vec<Coordinate> = vec![];
    let part_1 = map.keys().fold(0, |total, square| {
        let height = map.get(square).unwrap();
        if get_neighbours(*square)
            .iter()
            .any(|n| height >= map.get(n).unwrap_or(&usize::MAX))
        {
            total
        } else {
            low_points.push(*square);
            total + height + 1
        }
    });
    let mut sizes: Vec<usize> = low_points
        .iter()
        .map(|startpoint| size_of_basin(&mut map, *startpoint))
        .collect();
    sizes.sort_by(|a, b| b.cmp(a));
    (part_1, sizes[0] * sizes[1] * sizes[2])
}

fn get_neighbours((x, y): (isize, isize)) -> Vec<(isize, isize)> {
    vec![(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
}

fn size_of_basin(map: &mut HashMap<Coordinate, usize>, startpoint: Coordinate) -> usize {
    let mut considered: VecDeque<Coordinate> = VecDeque::from([startpoint]);
    let mut size = 0;
    while !considered.is_empty() {
        let c = considered.pop_front().unwrap();
        size = match map.get(&c) {
            Some(height) => {
                for n in get_neighbours(c) {
                    let nh = *map.get(&n).unwrap_or(&usize::MAX);
                    if nh > *height && nh < 9 {
                        considered.push_back(n);
                    }
                }
                map.remove(&c);
                size + 1
            }
            _ => size,
        }
    }
    size
}

#[cfg(test)]
mod day_9_tests {
    use super::*;
    static INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_small_input() {
        let (part_1, part_2) = run(INPUT);
        assert_eq!(part_1, 15);
        assert_eq!(part_2, 1134);
    }

    #[test]
    fn test_large_input() {
        let (part_1, part_2) = run(include_str!("../../input/09.txt"));
        assert_eq!(part_1, 580);
        assert_eq!(part_2, 856716);
    }
}
