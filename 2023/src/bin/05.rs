static INPUT_TXT: &str = include_str!("../../input/05.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_seeds(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Mapping {
    src: i64,
    dest: i64,
    len: i64,
}

impl From<&str> for Mapping {
    fn from(s: &str) -> Self {
        let nums: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Self {
            src: nums[0],
            dest: nums[1],
            len: nums[2],
        }
    }
}

impl Mapping {
    fn map(&self, seed: i64) -> Option<i64> {
        let range = self.dest..self.dest + self.len;
        let offset = self.src - self.dest;
        range.contains(&seed).then_some(seed + offset)
    }
}

fn parse_mappings(input: &str) -> Vec<Mapping> {
    input.lines().skip(1).map(Mapping::from).collect()
}

fn part_1(input: &str) -> i64 {
    let mut lines = input.trim().split("\n\n");
    let seeds = parse_seeds(lines.next().unwrap());
    let mappings: Vec<_> = lines.map(parse_mappings).collect();
    seeds
        .into_iter()
        .map(|seed| {
            mappings.iter().fold(seed, |seed, mapping| {
                mapping.iter().find_map(|m| m.map(seed)).unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

fn map_field(mapping: &[Mapping], seed: (i64, i64)) -> Vec<(i64, i64)> {
    let (mut mapped, unmapped) =
        mapping
            .iter()
            .fold((vec![], vec![seed]), |(mut mapped, unmapped), m| {
                let updated = unmapped.iter().fold(Vec::new(), |mut map, &(start, end)| {
                    let min_dest = end.min(m.dest);
                    if min_dest > start {
                        map.push((start, min_dest));
                    }
                    let max_start = start.max(m.dest);
                    let min_end = (m.dest + m.len).min(end);
                    if min_end > max_start {
                        mapped.push((max_start - m.dest + m.src, min_end - m.dest + m.src));
                    }
                    let max_dest = (m.dest + m.len).max(start);
                    if end > max_dest {
                        map.push((max_dest, end));
                    }
                    map
                });
                (mapped, updated)
            });
    mapped.extend(unmapped);
    mapped
}

pub fn part_2(input: &str) -> i64 {
    let mut lines = input.trim().split("\n\n");
    let seeds: Vec<(i64, i64)> = parse_seeds(lines.next().unwrap())
        .chunks(2)
        .map(|s| (s[0], s[0] + s[1]))
        .collect();
    lines
        .map(parse_mappings)
        .fold(seeds, |seeds, mapping| {
            seeds
                .iter()
                .flat_map(|&seed| map_field(&mapping, seed))
                .collect::<Vec<_>>()
        })
        .iter()
        .map(|&(s, _)| s)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35);
        assert_eq!(part_1(INPUT_TXT), 57_075_758);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 46);
        assert_eq!(part_2(INPUT_TXT), 31_161_857);
    }
}
