use std::collections::HashMap;
static INPUT_TXT: &str = include_str!("../../input/16.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

fn parse_line(line: &str) -> (String, (i64, Vec<String>)) {
    let (a, b) = line.split_once("; ").unwrap();
    let (a, flow_rate) = a.split_once(" has flow rate=").unwrap();
    let (_, valve_name) = a.split_once(' ').unwrap();

    let (_, b) = b.split_once(" to ").unwrap();
    let (_, tunnels) = b.split_once(' ').unwrap();

    (
        valve_name.to_string(),
        (
            flow_rate.parse().unwrap(),
            tunnels.split(", ").map(|s| s.to_string()).collect(),
        ),
    )
}

fn get_flowrates(input: &str) -> (HashMap<(i64, String, i64), i64>, HashMap<i64, i64>) {
    let valves: HashMap<String, (i64, Vec<String>)> =
        input.trim().lines().map(parse_line).collect();
    let idx: HashMap<_, _> = valves
        .iter()
        .enumerate()
        .map(|(i, (x, _))| (x.as_str(), i))
        .collect();

    let mut snapshot: HashMap<i64, i64> = Default::default();
    let mut flowrate: HashMap<(i64, String, i64), i64> = Default::default();
    flowrate.insert((0, "AA".to_string(), 0), 0);

    for time in 1..=30 {
        for locn in valves.keys() {
            macro_rules! upd {
                ($opn:expr, $flow:expr, $newopn:expr) => {
                    let newflow = $flow
                        + valves
                            .iter()
                            .enumerate()
                            .map(|(i, (_, (x, _)))| if $opn & (1 << i) != 0 { *x } else { 0 })
                            .sum::<i64>();
                    if let Some(oldflow) = flowrate.get(&(time, locn.clone(), $newopn)) {
                        if newflow > *oldflow {
                            flowrate.insert((time, locn.clone(), $newopn), newflow);
                        }
                    } else {
                        flowrate.insert((time, locn.clone(), $newopn), newflow);
                    }
                };
            }

            for (oldlocn, _) in valves.iter().filter(|l| l.1 .1.contains(locn)) {
                let prev: Vec<_> = flowrate
                    .iter()
                    .filter(|((t, ol, _), _)| *t == time - 1 && ol == oldlocn)
                    .map(|((_t, _ol, opn), flow)| (opn.clone(), *flow))
                    .collect();
                for (opn, flow) in prev {
                    upd!(opn, flow, opn);
                }
            }

            if valves[locn].0 > 0 {
                let prev: Vec<_> = flowrate
                    .iter()
                    .filter(|((t, ol, opn), _)| {
                        *t == time - 1 && ol == locn && (opn & (1 << idx[locn.as_str()]) == 0)
                    })
                    .map(|((_t, _ol, opn), flow)| (opn.clone(), *flow))
                    .collect();
                for (opn, flow) in prev {
                    let newopn: i64 = opn | (1 << idx[locn.as_str()]);
                    upd!(opn, flow, newopn);
                }
            }
        }

        flowrate.retain(|(t, _, _), _| *t == time);

        if time == 26 {
            for ((t, _l, opn), flow) in flowrate.iter() {
                if *t == 26 {
                    let s = snapshot.entry(*opn).or_default();
                    *s = (*s).max(*flow);
                }
            }
        }
    }
    (flowrate, snapshot)
}

fn part_1(input: &str) -> i64 {
    *get_flowrates(input).0.values().max().unwrap()
}

fn part_2(input: &str) -> i64 {
    let (_, snapshot) = get_flowrates(input);
    snapshot.iter().fold(0, |res, (&opn1, &flow1)| {
        snapshot.iter().fold(res, |res, (&opn2, &flow2)| {
            if opn1 & opn2 == 0 {
                res.max(flow1 + flow2)
            } else {
                res
            }
        })
    })
}

#[cfg(test)]
mod day_16_tests {
    use super::*;
    static INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1651);
        assert_eq!(part_1(INPUT_TXT), 1728);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1707);
        assert_eq!(part_2(INPUT_TXT), 2304);
    }
}
