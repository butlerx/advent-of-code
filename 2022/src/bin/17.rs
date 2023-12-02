static INPUT_TXT: &str = include_str!("../../input/17.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Point = (i64, i64);

#[derive(Debug)]
enum Rock {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

#[derive(Debug)]
struct Chamber {
    grid: Vec<bool>,
    height: i64,
    rock: Rock,
    x: i64,
    y: i64,
}

fn to_index((x, y): Point) -> usize {
    (y * 7 + x) as usize
}

impl Chamber {
    fn new() -> Self {
        Self {
            grid: vec![false; 7],
            height: 1,
            rock: Rock::Horizontal,
            x: 2,
            y: 3,
        }
    }

    fn is_occupied(&self, point: Point) -> bool {
        *self.grid.get(to_index(point)).unwrap_or(&false)
    }

    fn get_points(&self) -> Vec<Point> {
        match self.rock {
            Rock::Horizontal => (0..4).map(|delta| (self.x + delta, self.y)).collect(),
            Rock::Cross => vec![
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
                (self.x + 2, self.y + 1),
                (self.x + 1, self.y),
                (self.x + 1, self.y + 2),
            ],
            Rock::Angle => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x + 2, self.y + 1),
                (self.x + 2, self.y + 2),
            ],
            Rock::Vertical => (0..4).map(|delta| (self.x, self.y + delta)).collect(),
            Rock::Square => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y + 1),
            ],
        }
    }

    fn step(&mut self, direction: char) -> bool {
        let points = self.get_points();
        self.x += match direction {
            '>' if !points
                .iter()
                .any(|&(x, y)| x == 6 || self.is_occupied((x + 1, y))) =>
            {
                1
            }
            '<' if !points
                .iter()
                .any(|&(x, y)| x == 0 || self.is_occupied((x - 1, y))) =>
            {
                -1
            }
            _ => 0,
        };

        let new_points = self.get_points();
        if new_points
            .clone()
            .into_iter()
            .any(|(x, y)| y == 0 || self.is_occupied((x, y - 1)))
        {
            for (x, y) in new_points {
                if y > self.height - 1 {
                    self.height = y + 1;
                    self.grid.extend(vec![
                        false;
                        (self.height as usize + 1 - self.grid.len() / 7) * 7
                    ]);
                }
                self.grid[to_index((x, y))] = true;
            }
            self.rock = match self.rock {
                Rock::Horizontal => Rock::Cross,
                Rock::Cross => Rock::Angle,
                Rock::Angle => Rock::Vertical,
                Rock::Vertical => Rock::Square,
                Rock::Square => Rock::Horizontal,
            };
            self.y = self.height + 3;
            self.x = 2;
            true
        } else {
            self.y -= 1;
            false
        }
    }
}

fn part_1(input: &str) -> i64 {
    let mut ops = input.trim().chars().cycle();
    let mut chamber = Chamber::new();
    let mut count: usize = 0;
    loop {
        if count == 2022 {
            break chamber.height;
        }
        if chamber.step(ops.next().unwrap()) {
            count += 1;
        }
    }
}

fn part_2(input: &str) -> i64 {
    let mut ops = input.trim().chars().cycle();
    let mut chamber = Chamber::new();
    let deltas = {
        let mut deltas = [0; 5000];
        let mut previous = 0;
        let mut count = 0;
        loop {
            if count == 5000 {
                break deltas;
            }
            if chamber.step(ops.next().unwrap()) {
                deltas[count] = chamber.height - previous;
                previous = chamber.height;
                count += 1;
            }
        }
    };
    let (offset, size) = (0..500)
        .find_map(|offset| {
            let delta_iter = deltas.iter().skip(offset);
            let size = (2..=2500).find(|size| {
                let window = deltas[offset..offset + size].iter().cycle();
                delta_iter.clone().zip(window).all(|(a, b)| a == b)
            });
            size.map(|size| (offset, size))
        })
        .unwrap();
    let mut delta_iter = deltas.iter();
    let mut count = 1_000_000_000_000;
    let offset_delta = delta_iter.by_ref().take(offset).sum::<i64>();
    count -= offset;
    let cycle_deltas: Vec<_> = delta_iter.take(size).copied().collect();
    let cycle_delta = cycle_deltas.iter().sum::<i64>();
    let cycle_count = (count / size) as i64;
    count %= size;
    offset_delta + cycle_count * cycle_delta + cycle_deltas.into_iter().take(count).sum::<i64>()
}

#[cfg(test)]
mod day_17_tests {
    use super::*;
    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 3068);
        assert_eq!(part_1(INPUT_TXT), 3186);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1_514_285_714_288);
        assert_eq!(part_2(INPUT_TXT), 1_566_376_811_584);
    }
}
