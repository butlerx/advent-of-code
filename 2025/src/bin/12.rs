#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{time_execution, time_execution_us,  Point};
use std::collections::HashSet;

static INPUT_TXT: &str = include_str!("../../input/12.txt");

fn main() {
    println!("🌟 --- Day 12 Results --- 🌟");
    let (res_1, duration_1) = time_execution_us(|| part_1_quick(INPUT_TXT));
    println!("📌 Part 1 (Quick): {res_1}, complete in {duration_1} us");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");
}

fn part_1_quick(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter(|section| !section.contains('#'))
        .flat_map(str::lines)
        .filter_map(parse_region_line)
        .filter(|((width, height), present_counts)| {
            let three_by_three_squares = (width / 3) * (height / 3);
            three_by_three_squares >= present_counts.iter().sum()
        })
        .count()
}

fn parse_region_line(line: &str) -> Option<((usize, usize), Vec<usize>)> {
    let mut entries = line.split_ascii_whitespace();
    let dimensions = entries.next()?;
    let (width, height) = parse_dimensions(dimensions)?;
    let counts = entries.filter_map(|e| e.parse().ok()).collect();
    Some(((width, height), counts))
}

fn parse_dimensions(dimensions: &str) -> Option<(usize, usize)> {
    let x_pos = dimensions.find('x')?;
    let x = dimensions[..x_pos].parse().ok()?;
    let y = dimensions[x_pos + 1..dimensions.len() - 1].parse().ok()?;
    Some((x, y))
}

fn part_1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    regions
        .iter()
        .filter(|region| can_fit_region(region, &shapes))
        .count()
}

type Shape = Vec<Point>;


#[derive(Debug, Clone)]
struct OrientationInfo {
    coords: Shape,
    width: usize,
    height: usize,
    size: usize,
}

#[derive(Debug)]
struct ParseState {
    shapes: Vec<Shape>,
    regions: Vec<String>,
    current_shape_id: Option<usize>,
    shape_lines: Vec<String>,
}

impl ParseState {
    fn new() -> Self {
        Self {
            shapes: Vec::new(),
            regions: Vec::new(),
            current_shape_id: None,
            shape_lines: Vec::new(),
        }
    }

    fn finalize_current_shape(&mut self) {
        if let Some(id) = self.current_shape_id.take()
            && !self.shape_lines.is_empty() {
                while self.shapes.len() <= id {
                    self.shapes.push(Vec::new());
                }
                self.shapes[id] = parse_shape(&self.shape_lines);
                self.shape_lines.clear();
            }
    }

    fn process_line(mut self, line: &str) -> Self {
        if is_region_line(line) {
            self.regions.push(line.to_string());
        } else if let Some(shape_id) = parse_shape_header(line) {
            self.finalize_current_shape();
            self.current_shape_id = Some(shape_id);
        } else if is_shape_content(line) {
            self.shape_lines.push(line.to_string());
        }
        self
    }
}

fn is_region_line(line: &str) -> bool {
    line.contains(':') && line.split(':').next().is_some_and(|s| s.contains('x'))
}

fn parse_shape_header(line: &str) -> Option<usize> {
    if !line.contains(':') {
        return None;
    }
    line.split(':').next()?.trim().parse().ok()
}

fn is_shape_content(line: &str) -> bool {
    !line.trim().is_empty() && (line.contains('#') || line.contains('.'))
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<String>) {
    let mut state = input
        .lines()
        .fold(ParseState::new(), ParseState::process_line);
    state.finalize_current_shape();
    (state.shapes, state.regions)
}

fn parse_shape(lines: &[String]) -> Shape {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| (ch == '#').then_some((x, y).into()))
        })
        .collect()

}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

impl Region {
    fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dim = parts.first()?.trim_end_matches(':');

        let (width, height) = dim
            .split_once('x')
            .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))?;

        let shape_counts = parts[1..].iter().filter_map(|s| s.parse().ok()).collect();

        Some(Self {
            width,
            height,
            shape_counts,
        })
    }

    fn to_present_list(&self) -> Vec<usize> {
        self.shape_counts
            .iter()
            .enumerate()
            .flat_map(|(shape_id, &count)| std::iter::repeat_n(shape_id, count))
            .collect()
    }
}

fn can_fit_region(line: &str, shapes: &[Shape]) -> bool {
    let Some(region) = Region::from_line(line) else {
        return false;
    };

    let mut presents = region.to_present_list();

    if presents.is_empty() {
        return true;
    }

    if presents
        .iter()
        .any(|&id| id >= shapes.len() || shapes[id].is_empty())
    {
        return false;
    }

    let shape_orientations: Vec<_> = shapes
        .iter()
        .map(|shape| get_all_orientations_with_bounds(shape))
        .collect();

    presents.sort_by_key(|&id| std::cmp::Reverse(shape_orientations[id][0].size));

    let mut occupied = HashSet::new();
    solve_placement(
        &mut occupied,
        region.width,
        region.height,
        &shape_orientations,
        &presents,
        0,
    )
}

fn get_all_orientations_with_bounds(coords: &[Point]) -> Vec<OrientationInfo> {
    get_all_orientations(coords)
        .into_iter()
        .map(|coords| {
            let (max_x, max_y) = coords
                .iter()
                .fold((0i64, 0i64), |(mx, my), p| (mx.max(p.x), my.max(p.y)));
            OrientationInfo {
                size: coords.len(),
                width: usize::try_from(max_x + 1).expect("number too large"),
                height: usize::try_from(max_y + 1).expect("number too large"),
                coords,
            }
        })
        .collect()
}

fn get_all_orientations(coords: &[Point]) -> Vec<Vec<Point>> {
    let rotations = (0..4).scan(coords.to_vec(), |state, _| {
        let current = state.clone();
        *state = rotate_90(state);
        Some(normalize(&current))
    });

    let flipped = coords
        .iter()
        .map(|p| Point::new(-p.x, p.y))
        .collect::<Vec<_>>();
    let flipped_rotations = (0..4).scan(flipped, |state, _| {
        let current = state.clone();
        *state = rotate_90(state);
        Some(normalize(&current))
    });

    rotations
        .chain(flipped_rotations)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn rotate_90(coords: &[Point]) -> Vec<Point> {
    coords.iter().map(|p| Point::new(-p.y, p.x)).collect()
}

fn normalize(coords: &[Point]) -> Vec<Point> {
    if coords.is_empty() {
        return Vec::new();
    }

    let (min_x, min_y) = coords
        .iter()
        .fold((i64::MAX, i64::MAX), |(min_x, min_y), p| {
            (min_x.min(p.x), min_y.min(p.y))
        });

    let mut normalized: Vec<_> = coords
        .iter()
        .map(|p| Point::new(p.x - min_x, p.y - min_y))
        .collect();

    normalized.sort_unstable_by_key(|p| (p.y, p.x));
    normalized
}

fn solve_placement(
    occupied: &mut HashSet<Point>,
    grid_width: usize,
    grid_height: usize,
    orientations: &[Vec<OrientationInfo>],
    presents: &[usize],
    idx: usize,
) -> bool {
    if idx == presents.len() {
        return true;
    }

    let occupied_count = occupied.len();
    let total_space = grid_width * grid_height;
    let remaining_space = total_space - occupied_count;
    let remaining_shapes_size: usize = presents[idx..]
        .iter()
        .map(|&id| orientations[id][0].size)
        .sum();

    if remaining_space < remaining_shapes_size {
        return false;
    }

    let shape_id = presents[idx];

    for orientation in &orientations[shape_id] {
        let max_x = grid_width.saturating_sub(orientation.width);
        let max_y = grid_height.saturating_sub(orientation.height);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos: Point = (x, y).into();

                let placements: Vec<Point> = orientation
                    .coords
                    .iter()
                    .map(|&offset| pos + offset)
                    .collect();

                if placements.iter().all(|p| !occupied.contains(p)) {
                    for &p in &placements {
                        occupied.insert(p);
                    }

                    if solve_placement(
                        occupied,
                        grid_width,
                        grid_height,
                        orientations,
                        presents,
                        idx + 1,
                    ) {
                        return true;
                    }

                    for &p in &placements {
                        occupied.remove(&p);
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 2);
        assert_eq!(part_1(INPUT_TXT), 487);
    }

    #[test]
    fn test_part1_quick() {
        assert_eq!(part_1_quick(INPUT_TXT), 487);
    }
}
