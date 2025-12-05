#![warn(clippy::pedantic, clippy::perf)]
use aoc_shared::time_execution;
use std::collections::HashMap;

static INPUT_TXT: &str = include_str!("../../input/19.txt");

fn main() {
    println!("🌟 --- Day 19 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

#[derive(Default)]
struct Node {
    towel: bool,
    next: [u16; 26],
}

struct Trie {
    trie: Vec<Node>,
}

impl Trie {
    fn next(&self, i: usize, j: usize) -> usize {
        self.trie[i].next[j] as usize
    }

    fn is_towel(&self, i: usize) -> bool {
        self.trie[i].towel
    }
}

impl From<&str> for Trie {
    fn from(available_colours: &str) -> Self {
        let trie = available_colours
            .split(", ")
            .fold(vec![Node::default()], |mut trie, towel| {
                let final_index = towel.bytes().map(to_index).fold(0, |i, j| {
                    if trie[i].next[j] == 0 {
                        trie.push(Node::default());
                        let next_index = trie.len() - 1;
                        trie[i].next[j] = u16::try_from(next_index).expect("Index too large");
                        next_index
                    } else {
                        trie[i].next[j] as usize
                    }
                });
                trie[final_index].towel = true;
                trie
            });
        Self { trie }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = u64> + '_ {
    let (available_colours, desgins) = input.split_once("\n\n").expect("Invalid input");
    let trie = Trie::from(available_colours);
    let seen = &mut HashMap::with_capacity(20_000);
    desgins
        .lines()
        .map(move |design| count_arrangements(&trie, seen, design.as_bytes()))
        .collect::<Vec<_>>()
        .into_iter()
}

#[inline]
fn count_arrangements<'a>(trie: &Trie, seen: &mut HashMap<&'a [u8], u64>, design: &'a [u8]) -> u64 {
    if design.is_empty() {
        return 1;
    }
    seen.get(design).copied().unwrap_or_else(|| {
        let ways = (0..design.len())
            .scan(0, |state, depth| {
                *state = trie.next(*state, to_index(design[depth]));
                Some((*state, depth))
            })
            .take_while(|(i, _)| *i != 0)
            .filter(|(i, _)| trie.is_towel(*i))
            .map(|(_, depth)| count_arrangements(trie, seen, &design[depth + 1..]))
            .sum();

        seen.insert(design, ways);
        ways
    })
}

#[inline]
fn to_index(b: u8) -> usize {
    (b - b'a') as usize
}

fn part_1(input: &str) -> usize {
    parse_input(input).filter(|&n| n > 0).count()
}

fn part_2(input: &str) -> u64 {
    parse_input(input).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 6);
        assert_eq!(part_1(INPUT_TXT), 296);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 16);
        assert_eq!(part_2(INPUT_TXT), 619_970_556_776_002);
    }
}
