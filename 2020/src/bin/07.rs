use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/07.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn contains_gold(map: &HashMap<&str, Vec<(i64, String)>>, bag: &str) -> bool {
    map[bag]
        .iter()
        .any(|(_, name)| name == "shiny gold" || contains_gold(map, name))
}

fn parse_input(input: &str) -> HashMap<&str, Vec<(i64, String)>> {
    input
        .lines()
        .map(|line| {
            let rule: Vec<&str> = line.trim().split("bags contain").collect();
            let bags = Regex::new(r"(\d+) (\w+ \w+) bag")
                .unwrap()
                .captures_iter(rule[1])
                .map(|cap| (cap[1].parse::<i64>().unwrap(), cap[2].to_string()))
                .collect();
            (rule[0].trim(), bags)
        })
        .collect::<HashMap<&str, Vec<(i64, String)>>>()
}

fn unpack(
    bags: &HashMap<&str, Vec<(i64, String)>>,
    name: &str,
    multiplier: i64,
) -> Vec<(i64, String)> {
    let inner_bags = bags.get(name).unwrap();
    inner_bags
        .iter()
        .map(|(count, bag)| (multiplier * count, bag.to_string()))
        .chain(
            inner_bags
                .iter()
                .map(|(count, bag)| unpack(bags, bag, multiplier * count))
                .flatten(),
        )
        .collect()
}

pub fn run(input: &str, part_two: bool) -> i64 {
    let bags = parse_input(input);
    if !part_two {
        bags.keys().filter(|k| contains_gold(&bags, k)).count() as i64
    } else {
        unpack(&bags, "shiny gold", 1)
            .iter()
            .map(|(count, _bag)| count)
            .sum::<i64>()
    }
}

#[cfg(test)]
mod day_7_tests {
    use super::*;
    static INPUT_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    static INPUT_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT_1, false) == 4);
        assert!(run(INPUT_2, false) == 0);
        assert!(run(include_str!("../../input/07.txt"), false) == 177);
    }

    #[test]
    fn test_part_2() {
        assert!(run(INPUT_1, true) == 32);
        assert!(run(INPUT_2, true) == 126);
        assert!(run(include_str!("../../input/07.txt"), true) == 34988);
    }
}
