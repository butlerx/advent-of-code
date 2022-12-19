use std::{cmp, collections::HashMap};
static INPUT_TXT: &str = include_str!("../../input/19.txt");

fn main() {
    println!("Part 1: {}", part_1(INPUT_TXT));
    println!("Part 2: {}", part_2(INPUT_TXT));
}

type Robots = (i64, i64, i64);
type Costs = (i64, i64, i64);
type Cache = HashMap<(i64, Costs, Robots, u8), i64>;

const ROBOTS_ORE: u8 = 1 << 0;
const ROBOTS_CLAY: u8 = 1 << 1;
const ROBOTS_OBS: u8 = 1 << 2;
const ROBOTS_GEO: u8 = 1 << 3;
const ROBOTS_ALL: u8 = ROBOTS_ORE | ROBOTS_CLAY | ROBOTS_OBS | ROBOTS_GEO;

struct Blueprint {
    ore_robot_cost: i64,
    clay_robot_cost: i64,
    obs_robot_cost: (i64, i64),
    geode_robot_cost: (i64, i64),
    max_ore_cost: i64,
    max_clay_cost: i64,
    max_obs_cost: i64,
}

impl std::str::FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let costs = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|n| !n.is_empty())
            .skip(1)
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            ore_robot_cost: costs[0],
            clay_robot_cost: costs[1],
            obs_robot_cost: (costs[2], costs[3]),
            geode_robot_cost: (costs[4], costs[5]),
            max_ore_cost: costs[0].max(costs[1]).max(costs[2]).max(costs[4]),
            max_clay_cost: costs[3],
            max_obs_cost: costs[5],
        })
    }
}

impl Blueprint {
    fn max_geodes(
        &self,
        time: i64,
        costs: Costs,
        robots: Robots,
        available_robots: u8,
        cache: &mut Cache,
    ) -> i64 {
        if time == 0 {
            return 0;
        }

        let new_costs = (
            cmp::min(costs.0 + robots.0, self.max_ore_cost * time),
            cmp::min(costs.1 + robots.1, self.max_clay_cost * time),
            cmp::min(costs.2 + robots.2, self.max_obs_cost * time),
        );

        let key = (time, new_costs, robots, available_robots);
        if !cache.contains_key(&key) {
            let mut max_geodes = 0;
            let mut new_available_robots = 0;

            if (costs.0 < self.geode_robot_cost.0) || (costs.2 < self.geode_robot_cost.1) {
                new_available_robots |= ROBOTS_GEO;
            } else if (available_robots & ROBOTS_GEO) != 0 {
                max_geodes = cmp::max(
                    max_geodes,
                    self.max_geodes(
                        time - 1,
                        (
                            new_costs.0 - self.geode_robot_cost.0,
                            new_costs.1,
                            new_costs.2 - self.geode_robot_cost.1,
                        ),
                        robots,
                        ROBOTS_ALL,
                        cache,
                    ) + (time - 1),
                );
            }

            let next_potential = (time - 1) / 2 * time;
            if next_potential > max_geodes {
                if costs.0 < self.ore_robot_cost {
                    new_available_robots |= ROBOTS_ORE;
                } else if (self.max_ore_cost > robots.0) && ((available_robots & ROBOTS_ORE) != 0) {
                    max_geodes = cmp::max(
                        max_geodes,
                        self.max_geodes(
                            time - 1,
                            (new_costs.0 - self.ore_robot_cost, new_costs.1, new_costs.2),
                            (robots.0 + 1, robots.1, robots.2),
                            ROBOTS_ALL,
                            cache,
                        ),
                    );
                }

                if costs.0 < self.clay_robot_cost {
                    new_available_robots |= ROBOTS_CLAY;
                } else if (self.max_clay_cost > robots.1) && ((available_robots & ROBOTS_CLAY) != 0)
                {
                    max_geodes = cmp::max(
                        max_geodes,
                        self.max_geodes(
                            time - 1,
                            (new_costs.0 - self.clay_robot_cost, new_costs.1, new_costs.2),
                            (robots.0, robots.1 + 1, robots.2),
                            ROBOTS_ALL,
                            cache,
                        ),
                    );
                }

                if (costs.0 < self.obs_robot_cost.0) || (costs.1 < self.obs_robot_cost.1) {
                    new_available_robots |= ROBOTS_OBS;
                } else if (self.max_obs_cost > robots.2) && ((available_robots & ROBOTS_OBS) != 0) {
                    max_geodes = cmp::max(
                        max_geodes,
                        self.max_geodes(
                            time - 1,
                            (
                                new_costs.0 - self.obs_robot_cost.0,
                                new_costs.1 - self.obs_robot_cost.1,
                                new_costs.2,
                            ),
                            (robots.0, robots.1, robots.2 + 1),
                            ROBOTS_ALL,
                            cache,
                        ),
                    );
                }

                if new_available_robots != 0 {
                    max_geodes = cmp::max(
                        max_geodes,
                        self.max_geodes(time - 1, new_costs, robots, new_available_robots, cache),
                    );
                }
            }

            cache.insert(key, max_geodes);
        }

        *cache.get(&key).unwrap()
    }
}

fn part_1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .enumerate()
        .map(|(i, b)| {
            let mut cache = HashMap::new();
            (i as i64 + 1) * b.max_geodes(24, (0, 0, 0), (1, 0, 0), ROBOTS_ALL, &mut cache)
        })
        .sum()
}

fn part_2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .take(3)
        .map(|b| {
            let mut cache = HashMap::new();
            b.max_geodes(32, (0, 0, 0), (1, 0, 0), ROBOTS_ALL, &mut cache)
        })
        .product()
}

#[cfg(test)]
mod day_19_tests {
    use super::*;
    static INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 33);
        assert_eq!(part_1(INPUT_TXT), 1681);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 3472);
        assert_eq!(part_2(INPUT_TXT), 5394);
    }
}
