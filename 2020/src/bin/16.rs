use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Rule = (RangeInclusive<i64>, RangeInclusive<i64>);
type Schema = HashMap<String, Rule>;
type Ticket = Vec<i64>;

fn main() {
    let input = include_str!("../../input/16.txt");
    println!("Part 1: {}", run(input, false));
    println!("Part 2: {}", run(input, true));
}

fn parse_schema(input: &str) -> Schema {
    let re = Regex::new(r"^([a-zA-Z\s]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                cap[1].trim().to_string(),
                (
                    RangeInclusive::new(cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                    RangeInclusive::new(cap[4].parse().unwrap(), cap[5].parse().unwrap()),
                ),
            )
        })
        .collect::<Schema>()
}

fn parse_tickets(input: &str) -> Vec<Ticket> {
    input
        .trim()
        .lines()
        .skip(1)
        .map(|line| {
            line.trim()
                .split(',')
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn invalid_fields(schema: &Schema, ticket: &[i64]) -> i64 {
    ticket
        .iter()
        .filter(|value| !schema.values().any(|ranges| valid_rule(ranges, value)))
        .sum()
}

fn valid_rule(ranges: &Rule, value: &i64) -> bool {
    ranges.0.contains(value) || ranges.1.contains(value)
}

fn generate_name_index(mut map: HashMap<String, HashSet<usize>>) -> HashMap<String, usize> {
    let mut name_index = HashMap::new();
    while !map.is_empty() {
        for (k, v) in map.iter().filter(|(_, v)| v.len() == 1) {
            name_index.insert(k.clone(), v.clone().into_iter().next().unwrap());
        }
        map = map.into_iter().filter(|(_, v)| v.len() != 1).collect();
        for (_, indexes) in map.iter_mut() {
            for finished in name_index.values() {
                if indexes.contains(finished) {
                    indexes.remove(finished);
                }
            }
        }
    }
    name_index
}

fn departure_data(schema: Schema, nearby_tickets: Vec<Vec<i64>>, your_ticket: Ticket) -> i64 {
    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| invalid_fields(&schema, ticket) == 0)
        .collect();
    generate_name_index(
        schema
            .iter()
            .map(|(name, rule)| {
                let valid_options: HashSet<usize> = (0..schema.len())
                    .filter(|field_index| {
                        valid_tickets
                            .iter()
                            .map(|ticket| ticket[*field_index])
                            .all(|field| valid_rule(rule, &field))
                    })
                    .collect();
                (name.clone(), valid_options)
            })
            .collect(),
    )
    .iter()
    .filter(|(name, _)| name.starts_with("departure"))
    .map(|(_, index)| your_ticket[*index])
    .product()
}

fn run(input: &str, part_two: bool) -> i64 {
    let data: Vec<&str> = input.trim().split("\n\n").collect();
    let schema = parse_schema(data[0]);
    let nearby_tickets = parse_tickets(data[2]);
    if part_two {
        let your_ticket = parse_tickets(data[1]);
        departure_data(schema, nearby_tickets, your_ticket[0].clone())
    } else {
        nearby_tickets
            .iter()
            .fold(0, |total, ticket| total + invalid_fields(&schema, ticket))
    }
}

#[cfg(test)]
mod day_16_tests {
    use super::*;
    static INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part_1() {
        assert!(run(INPUT, false) == 71);
        let results = run(include_str!("../../input/16.txt"), false);
        println!("{}", results);
        assert!(results == 22057);
    }

    #[test]
    fn test_part_2() {
        let results = run(include_str!("../../input/16.txt"), true);
        println!("{}", results);
        assert!(results == 1093427331937);
    }
}
