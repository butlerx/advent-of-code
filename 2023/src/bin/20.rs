use std::collections::{hash_map::Entry, HashMap, VecDeque};

static INPUT_TXT: &str = include_str!("../../input/20.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

#[derive(Debug, Eq, PartialEq)]
enum Module<'a> {
    Noop,
    Broadcaster(Vec<&'a str>),
    Flipflop(bool, Vec<&'a str>),
    Conjunction(HashMap<&'a str, bool>, Vec<&'a str>),
}

fn parse_input(input: &str) -> HashMap<&str, Module> {
    let mut modules = input
        .trim()
        .lines()
        .map(|l| {
            let (n, t) = l.split_once(" -> ").unwrap();
            let targets = t.split(',').map(str::trim).collect::<Vec<_>>();
            if n == "broadcaster" {
                (n, Module::Broadcaster(targets))
            } else if let Some(name) = n.strip_prefix('%') {
                (name, Module::Flipflop(false, targets))
            } else if let Some(name) = n.strip_prefix('&') {
                (name, Module::Conjunction(HashMap::new(), targets))
            } else {
                unreachable!()
            }
        })
        .collect::<HashMap<_, _>>();

    modules.insert("output", Module::Noop);
    modules
}

fn part_1(input: &str) -> u32 {
    let mut modules = parse_input(input);
    let mut target_links = vec![];
    for (source_name, source_module) in &modules {
        match source_module {
            Module::Noop => {}
            Module::Broadcaster(t) | Module::Flipflop(_, t) | Module::Conjunction(_, t) => {
                for target_name in t {
                    target_links.push((*source_name, *target_name));
                }
            }
        };
    }

    for (source_name, target_name) in target_links {
        if let Entry::Occupied(mut target_module) = modules.entry(target_name) {
            if let Module::Conjunction(ref mut inputs, _) = target_module.get_mut() {
                inputs.insert(source_name, false);
            }
        }
    }

    let mut low_signals_sent = 0;
    let mut high_signals_sent = 0;

    for _ in 0..1000 {
        low_signals_sent += 1;
        let mut queue = VecDeque::new();
        let Module::Broadcaster(broadcast_targets) = &modules["broadcaster"] else {
            unreachable!()
        };
        for bt in broadcast_targets {
            queue.push_back(("broadcaster", *bt, false));
        }

        while let Some((source, dest, sig_is_high)) = queue.pop_front() {
            if sig_is_high {
                high_signals_sent += 1;
            } else {
                low_signals_sent += 1;
            }
            let target_module = modules.entry(dest).or_insert(Module::Noop);

            match target_module {
                Module::Flipflop(ref mut ff_is_on, ff_targets) => {
                    if !sig_is_high {
                        *ff_is_on = !*ff_is_on;
                        for t in ff_targets {
                            queue.push_back((dest, t, *ff_is_on));
                        }
                    }
                }
                Module::Conjunction(ref mut inputs, c_targets) => {
                    inputs.insert(source, sig_is_high);
                    let output_is_high = !inputs.values().all(|i| *i);

                    for t in c_targets {
                        queue.push_back((dest, t, output_is_high));
                    }
                }
                _ => {}
            }
        }
    }

    low_signals_sent * high_signals_sent
}

fn part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    static INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 32_000_000);
        assert_eq!(part_1(INPUT_2), 11_687_500);
        assert_eq!(part_1(INPUT_TXT), 825_167_435);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_TXT), 0);
    }
}
