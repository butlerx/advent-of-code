use std::collections::HashSet;
use std::convert::TryFrom;

fn main() {
    let input = include_str!("../../input/24.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn find_tile(input: &str, x: i64, y: i64) -> (i64, i64) {
    let (x, y, _) = input.chars().fold((x, y, None), |(x, y, prev), c| match c {
        'n' => (x, y, Some('n')),
        's' => (x, y, Some('s')),
        'e' => match prev {
            Some('n') => (x, y + 1, None),
            Some('s') => (x - 1, y, None),
            Some(_) => (x, y, None),
            None => (x - 1, y + 1, None),
        },
        'w' => match prev {
            Some('n') => (x + 1, y, None),
            Some('s') => (x, y - 1, None),
            Some(_) => (x, y, None),
            None => (x + 1, y - 1, None),
        },
        _ => (x, y, prev),
    });
    (x, y)
}

struct Tile {
    black: bool,
    neighbors: u8,
    timestamp: u8,
}

fn flip_100(input: &str) -> i64 {
    const GRID_SIZE: i64 = 150;
    let mut tiles: Vec<Vec<_>> = (0..GRID_SIZE)
        .map(|_| {
            (0..GRID_SIZE)
                .map(|_| Tile {
                    black: false,
                    neighbors: 0,
                    timestamp: u8::MAX,
                })
                .collect()
        })
        .collect();
    let mut black = HashSet::new();
    for line in input.lines() {
        let (x, y) = find_tile(line, GRID_SIZE / 2, GRID_SIZE / 2);
        let (x, y) = (
            usize::try_from(x).expect("cast within range"),
            usize::try_from(y).expect("cast within range"),
        );
        if !black.insert((x, y)) {
            black.remove(&(x, y));
        }
        tiles[x][y].black = !tiles[x][y].black;
    }

    let mut black: Vec<_> = black.into_iter().collect();
    let mut touched = Vec::new();

    let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, -1), (-1, 1)];

    for i in 0..100 {
        for &(x, y) in &black {
            for (dx, dy) in neighbors.iter().chain([(0, 0)].iter()) {
                let (x, y) = (
                    usize::try_from(isize::try_from(x).expect("cast within range") + dx)
                        .expect("cast within range"),
                    usize::try_from(isize::try_from(y).expect("cast within range") + dy)
                        .expect("cast within range"),
                );
                let tile = &mut tiles[x][y];
                if tile.timestamp != i {
                    tile.timestamp = i;
                    tile.neighbors = 0;
                    touched.push((x, y));
                }
            }
            for (dx, dy) in &neighbors {
                let (x, y) = (
                    usize::try_from(isize::try_from(x).expect("cast within range") + dx)
                        .expect("cast within range"),
                    usize::try_from(isize::try_from(y).expect("cast within range") + dy)
                        .expect("cast within range"),
                );
                tiles[x][y].neighbors += 1;
            }
        }

        black = touched
            .drain(0..)
            .filter(|&(x, y)| {
                let tile = &mut tiles[x][y];
                tile.black = if tile.black {
                    tile.neighbors == 1 || tile.neighbors == 2
                } else {
                    tile.neighbors == 2
                };
                tile.black
            })
            .collect();
    }

    i64::try_from(black.len()).expect("cast within range")
}

fn flip_tiles(input: &str) -> i64 {
    let mut black = HashSet::new();
    for line in input.lines() {
        let cooridinates = find_tile(line, 0, 0);
        if !black.insert(cooridinates) {
            black.remove(&cooridinates);
        }
    }
    i64::try_from(black.len()).expect("cast within range")
}

fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        flip_100(input)
    } else {
        flip_tiles(input)
    }
}

#[cfg(test)]
mod day_24_tests {
    use super::*;
    static INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part_1() {
        assert_eq!(run(INPUT, false), 10);
        assert_eq!(run(include_str!("../../input/24.txt"), false), 228);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 2208);
        assert_eq!(run(include_str!("../../input/24.txt"), true), 3672);
    }
}
