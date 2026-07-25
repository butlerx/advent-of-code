//! Systems Check - Mission 1
//!
//! <https://systemscheck.dev/missions/1>

use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (usize, usize, bool) {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let time = parts[0]
        .strip_prefix("t=")
        .expect("Invalid time format")
        .parse::<usize>()
        .expect("Failed to parse time");
    let cell: usize = parts[1]
        .strip_prefix('#')
        .expect("Invalid cell format")
        .parse()
        .expect("Failed to parse cell");
    let disabled = parts[2] == "disabled";
    (time, cell, disabled)
}

/// # Panics
///
/// Panics if input is empty.
pub fn part_1(input: &str) -> usize {
    let events: Vec<_> = input.trim().lines().map(parse_line).collect();
    let last_time = events.last().expect("No events found").0;

    let (disabled_at, max_duration) = events.iter().fold(
        (HashMap::new(), 0),
        |(mut disabled_at, mut max_duration), &(time, cell, is_disabled)| {
            if is_disabled {
                disabled_at.insert(cell, time);
            } else if let Some(start) = disabled_at.remove(&cell) {
                max_duration = max_duration.max(time - start);
            }
            (disabled_at, max_duration)
        },
    );

    disabled_at
        .values()
        .map(|start| last_time + 1 - start)
        .fold(max_duration, usize::max)
        .max(max_duration)
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>()
        .chunk_by(|a, b| a.0 == b.0)
        .scan(HashSet::new(), |disabled, group| {
            for &(_, cell, is_disabled) in group {
                if is_disabled {
                    disabled.insert(cell);
                } else {
                    disabled.remove(&cell);
                }
            }
            Some((group[0].0, disabled.len()))
        })
        .collect::<Vec<_>>()
        .windows(2)
        .fold(
            (0, 0, 0, 0),
            |(max_disabled, best_duration, current_start, current_count), window| {
                let (t, count) = window[0];
                let next_t = window[1].0;
                if count > max_disabled {
                    (count, next_t - t, t, count)
                } else if count == max_disabled && current_count == max_disabled {
                    (
                        max_disabled,
                        best_duration.max(next_t - current_start),
                        current_start,
                        count,
                    )
                } else if count == max_disabled {
                    (max_disabled, best_duration.max(next_t - t), t, count)
                } else {
                    (max_disabled, best_duration, current_start, count)
                }
            },
        )
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "t=0 #0 disabled
t=1 #1 disabled
t=3 #0 enabled
t=3 #2 disabled
t=3 #3 disabled
t=3 #4 disabled
t=6 #3 enabled
t=7 #2 enabled
t=8 #1 enabled
t=9 #4 enabled";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 3);
    }
}
