use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/20.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

#[derive(Debug, Clone)]
struct Tile {
    data: Matrix<bool>,
    id: usize,
    nesw: [u16; 4],
}

impl Tile {
    fn new(id: usize, img: &str) -> Tile {
        let data: Matrix<bool> = Matrix::from_rows(
            img.lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect::<Vec<Vec<bool>>>(),
        )
        .unwrap();
        let (n, e, s, w) = (0..4)
            .map(|i| {
                data.rotated_ccw(i)
                    .iter()
                    .next()
                    .unwrap()
                    .iter()
                    .copied()
                    .fold(0, |a, b| (a << 1) | u16::from(b))
            })
            .collect_tuple()
            .unwrap();
        Tile {
            data,
            id,
            nesw: [n, e, s, w],
        }
    }

    fn rotated(&self) -> Tile {
        Tile {
            data: self.data.rotated_cw(1),
            id: self.id,
            nesw: [self.nesw[3], self.nesw[0], self.nesw[1], self.nesw[2]],
        }
    }

    fn flipped(&self) -> Tile {
        Tile {
            data: self.data.flipped_lr(),
            id: self.id,
            nesw: [
                self.matching(0),
                self.matching(3),
                self.matching(2),
                self.matching(1),
            ],
        }
    }

    fn matching(&self, side: usize) -> u16 {
        (0..self.data.rows).fold(0, |total, row| {
            (total << 1)
                | u16::from(
                    (if row < 16 {
                        self.nesw[side] & (1 << row)
                    } else {
                        0
                    }) != 0,
                )
        })
    }

    fn collection(self) -> Vec<Self> {
        let rotated_1 = self.rotated();
        let rotated_2 = rotated_1.rotated();
        let rotated_3 = rotated_2.rotated();
        let mut rotations = vec![self, rotated_1, rotated_2, rotated_3];
        rotations.extend(rotations.clone().iter().map(Tile::flipped));
        rotations
    }
}

fn find_corners(tiles: &[Tile]) -> Vec<usize> {
    let mut matching = HashMap::new();
    for t in tiles {
        matching
            .entry(t.nesw[0])
            .or_insert_with(HashSet::new)
            .insert(t.id);
    }
    let mut counter = HashMap::new();
    matching
        .into_iter()
        .filter_map(|(_, v)| {
            if v.len() == 1 {
                Some(v.into_iter().next().unwrap())
            } else {
                None
            }
        })
        .for_each(|s| *counter.entry(s).or_insert(0) += 1);
    counter
        .into_iter()
        .filter_map(|(s, i)| if i == 4 { Some(s) } else { None })
        .collect()
}

fn find_img(tiles: &[Tile]) -> usize {
    let angles = find_corners(tiles);
    let topleft = angles[0];
    let topleft_index = tiles
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if t.id == topleft {
                let [n, _, _, w] = t.nesw;
                if tiles
                    .iter()
                    .all(|t| t.id == topleft || (t.nesw[0] != n && t.nesw[0] != w))
                {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();
    let mut placed = Matrix::square_from_vec(vec![0; tiles.len() / 8]).unwrap();
    placed[&(0, 0)] = topleft_index;
    let mut seen = HashSet::new();
    seen.insert(topleft);
    for col in 1..placed.columns {
        let east_match = tiles[placed[&(0, col - 1)]].matching(1);
        let next = tiles
            .iter()
            .enumerate()
            .find(|(_, t)| !seen.contains(&t.id) && t.nesw[3] == east_match)
            .unwrap()
            .0;
        placed[&(0, col)] = next;
        seen.insert(tiles[next].id);
    }
    for row in 1..placed.rows {
        for col in 0..placed.columns {
            let south_match = tiles[placed[&(row - 1, col)]].matching(2);
            let next = tiles
                .iter()
                .enumerate()
                .find(|(_, t)| !seen.contains(&t.id) && t.nesw[0] == south_match)
                .unwrap()
                .0;
            placed[&(row, col)] = next;
            seen.insert(tiles[next].id);
            if col > 0 {
                assert_eq!(
                    tiles[placed[&(row, col - 1)]].matching(1),
                    tiles[next].nesw[3]
                );
            }
        }
    }
    let tile_side = tiles[0].data.rows;
    let mut assembled = Matrix::new_square(placed.rows * (tile_side - 2), false);
    for row in 0..placed.rows {
        for col in 0..placed.columns {
            assembled.set_slice(
                &(row * (tile_side - 2), col * (tile_side - 2)),
                &tiles[placed[&(row, col)]]
                    .data
                    .slice(1..tile_side - 1, 1..tile_side - 1)
                    .unwrap(),
            );
        }
    }
    let all_assembled = Tile {
        data: assembled,
        nesw: [0, 0, 0, 0],
        id: 0,
    }
    .collection()
    .into_iter()
    .map(|t| t.data)
    .collect::<Vec<_>>();
    let monster_pos = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .enumerate()
    .flat_map(|(r, l)| {
        l.chars()
            .enumerate()
            .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
    })
    .collect::<Vec<_>>();
    let monsters = all_assembled
        .iter()
        .map(|t| {
            (0..t.rows - 2)
                .map(|r| {
                    (0..t.columns - 19)
                        .filter(|c| monster_pos.iter().all(|&(or, oc)| t[&(r + or, c + oc)]))
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    all_assembled[0].values().filter(|&&b| b).count() - monsters * monster_pos.len()
}

fn run(input: &str, part_two: bool) -> i64 {
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .flat_map(|tile| {
            let (tile_id, img) = tile.split(":\n").collect_tuple().unwrap();
            let (_, id) = tile_id.split(' ').collect_tuple().unwrap();
            Tile::new(id.parse::<usize>().unwrap(), img).collection()
        })
        .collect();
    (if part_two {
        find_img(&tiles)
    } else {
        find_corners(&tiles).into_iter().product()
    }) as i64
}

#[cfg(test)]
mod day_20_tests {
    use super::*;
    static INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 20_899_048_083_289);
        assert_eq!(
            run(include_str!("../../input/20.txt"), false),
            15_405_893_262_491
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 273);
        assert_eq!(run(include_str!("../../input/20.txt"), true), 2133);
    }
}
