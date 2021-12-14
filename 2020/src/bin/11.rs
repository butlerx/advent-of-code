use itertools::iproduct;

fn main() {
    let input = include_str!("../../input/11.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn count_neighbors(map: &[Vec<char>], y: usize, x: usize) -> usize {
    let mut total = 0;
    for (ny, nx) in iproduct!(y.saturating_sub(1)..=y + 1, x.saturating_sub(1)..=x + 1) {
        if ny == y && nx == x || ny >= map.len() || nx >= map[0].len() {
            continue;
        }

        if map[ny][nx] == '#' {
            total += 1
        }
    }
    total
}
// check if current coordinate is in the grid
fn on_grid(grid: &[Vec<char>], c: (i64, i64)) -> bool {
    c.1 >= 0 && c.0 >= 0 && c.1 < grid[0].len() as i64 && c.0 < grid.len() as i64
}

// index into grid using i64 instead of usize with current coordinate
fn check_grid(grid: &[Vec<char>], c: (i64, i64)) -> char {
    grid[c.0 as usize][c.1 as usize]
}

fn count_visible_neighbors(map: &[Vec<char>], y: usize, x: usize) -> usize {
    let mut total = 0;

    for dy in -1..=1 {
        'dx: for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut current = (y as i64 + dy, x as i64 + dx);
            if !on_grid(map, current) {
                continue;
            }
            while check_grid(map, current) == '.' {
                current = (current.0 + dy, current.1 + dx);
                if !on_grid(map, current) {
                    continue 'dx;
                }
            }

            match check_grid(map, current) {
                'L' => continue,
                '#' => total += 1,
                _ => unreachable!(),
            }
        }
    }
    total
}

fn fill_seats(map: &[Vec<char>], visible: bool) -> Vec<Vec<char>> {
    let mut output = map.to_owned();
    for (y, row) in map.iter().enumerate() {
        for (x, seat) in row.iter().enumerate() {
            let (neighbors, threshold) = if visible {
                (count_visible_neighbors(map, y, x), 5)
            } else {
                (count_neighbors(map, y, x), 4)
            };

            match (seat, neighbors) {
                ('L', 0) => output[y][x] = '#',
                ('#', n) if n >= threshold => output[y][x] = 'L',
                _ => {}
            }
        }
    }
    output
}

fn run(input: &str, part_two: bool) -> i64 {
    let mut map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut next = fill_seats(&map, part_two);
    while map != next {
        map = next.clone();
        next = fill_seats(&map, part_two);
    }
    map.into_iter().flatten().filter(|x| x == &'#').count() as i64
}

#[cfg(test)]
mod day_11_tests {
    use super::*;
    static INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 37);
        let results = run(include_str!("../../input/11.txt"), false);
        println!("{}", results);
        assert!(results == 2438);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT, true) == 26);
        let results = run(include_str!("../../input/11.txt"), true);
        println!("{}", results);
        assert!(results == 2174);
    }
}
