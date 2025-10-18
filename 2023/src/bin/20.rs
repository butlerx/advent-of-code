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

fn parse_input(input: &str) -> HashMap<&str, Module<'_>> {
    let modules = input
        .trim()
        .lines()
        .map(|l| {
            let (n, t) = l.split_once(" -> ").unwrap();
            let targets = t.split(',').map(str::trim).collect::<Vec<_>>();
            match n {
                "broadcaster" => (n, Module::Broadcaster(targets)),
                name if name.starts_with('%') => (
                    name.strip_prefix('%').unwrap(),
                    Module::Flipflop(false, targets),
                ),
                name if name.starts_with('&') => (
                    name.strip_prefix('&').unwrap(),
                    Module::Conjunction(HashMap::new(), targets),
                ),
                _ => unreachable!(),
            }
        })
        .chain(std::iter::once(("output", Module::Noop)))
        .collect::<HashMap<_, _>>();

    let target_links: Vec<(&str, &str)> = modules
        .iter()
        .flat_map(|(source_name, source_module)| match source_module {
            Module::Noop => Vec::new(),
            Module::Broadcaster(t) | Module::Flipflop(_, t) | Module::Conjunction(_, t) => t
                .iter()
                .map(move |target_name| (*source_name, *target_name))
                .collect(),
        })
        .collect();

    target_links
        .iter()
        .fold(modules, |mut acc_modules, (source_name, target_name)| {
            if let Entry::Occupied(mut target_module) = acc_modules.entry(*target_name) {
                if let Module::Conjunction(ref mut inputs, _) = target_module.get_mut() {
                    inputs.insert(*source_name, false);
                }
            }
            acc_modules
        })
}

fn part_1(input: &str) -> u32 {
    let mut modules = parse_input(input);

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

            match modules.entry(dest).or_insert(Module::Noop) {
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

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        _ if a == b => a,
        ((_, 0), _) => a,
        ((0, _), _) => b,
        (_, (0, 1) | (1, 0)) => gcd(a >> 1, b),
        (_, (0, 0)) => gcd(a >> 1, b >> 1) << 1,
        (_, (1, 1)) => {
            let (a, b) = (a.min(b), a.max(b));
            gcd((b - a) >> 1, a)
        }
        _ => unreachable!(),
    }
}

fn lcm(vals: impl Iterator<Item = usize>) -> usize {
    vals.fold(1, |ans, x| (x * ans) / gcd(x, ans))
}

fn part_2(input: &str) -> usize {
    let mut modules = parse_input(input);
    let mut cycles = [None; 4];
    for button_presses in 0.. {
        let mut queue = VecDeque::new();
        let Module::Broadcaster(broadcast_targets) = &modules["broadcaster"] else {
            unreachable!()
        };
        for bt in broadcast_targets {
            queue.push_back(("broadcaster", *bt, false));
        }

        while let Some((source, dest, sig_is_high)) = queue.pop_front() {
            if sig_is_high && dest == "bn" {
                let i = match source {
                    "pl" => 0,
                    "mz" => 1,
                    "lz" => 2,
                    "zm" => 3,
                    _ => unreachable!(),
                };
                cycles[i] = cycles[i].or(Some(button_presses + 1));
            }

            match modules.entry(dest).or_insert(Module::Noop) {
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
        if cycles.iter().all(std::option::Option::is_some) {
            break;
        }
    }
    lcm(cycles.iter().map(|o| o.unwrap()))
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
        assert_eq!(part_2(INPUT_TXT), 225_514_321_828_633);
    }
}
