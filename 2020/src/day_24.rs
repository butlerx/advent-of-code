use std::collections::HashSet;

fn find_tile(input: &str, x: i64, y: i64) -> (usize, usize) {
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
    (x as usize, y as usize)
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
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                let tile = &mut tiles[x][y];
                if tile.timestamp != i {
                    tile.timestamp = i;
                    tile.neighbors = 0;
                    touched.push((x, y));
                }
            }
            for (dx, dy) in &neighbors {
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
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

    black.len() as i64
}

fn flip_tiles(input: &str) -> i64 {
    let mut black = HashSet::new();
    for line in input.lines() {
        let cooridinates = find_tile(line, 0, 0);
        if !black.insert(cooridinates) {
            black.remove(&cooridinates);
        }
    }
    black.len() as i64
}

pub fn run(input: &str, part_two: bool) -> i64 {
    if part_two {
        flip_100(input)
    } else {
        flip_tiles(input)
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(run(include_str!("../input/day_24.txt"), false), 228);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(run(INPUT, true), 2208);
        // assert_eq!(run(include_str!("../input/day_24.txt"), true), 3672);
    }
}
