#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::time_execution;
use std::collections::{HashSet, VecDeque};
static INPUT_TXT: &str = include_str!("../../input/10.txt");

const INF: f64 = f64::INFINITY;
const EPS: f64 = 1e-9;

fn pivot(
    tableau: &mut [Vec<f64>],
    basic_indices: &mut [i32],
    non_basic_indices: &mut [i32],
    pivot_row: usize,
    pivot_col: usize,
    num_rows: usize,
    num_cols: usize,
) {
    let pivot_scale = 1.0 / tableau[pivot_row][pivot_col];
    for row_idx in 0..num_rows + 2 {
        if row_idx == pivot_row {
            continue;
        }
        for col_idx in 0..num_cols + 2 {
            if col_idx != pivot_col {
                tableau[row_idx][col_idx] -= tableau[pivot_row][col_idx] * tableau[row_idx][pivot_col] * pivot_scale;
            }
        }
    }
    for val in &mut tableau[pivot_row] {
        *val *= pivot_scale;
    }
    for row in tableau.iter_mut() {
        row[pivot_col] *= -pivot_scale;
    }
    tableau[pivot_row][pivot_col] = pivot_scale;
    std::mem::swap(&mut basic_indices[pivot_row], &mut non_basic_indices[pivot_col]);
}

fn find(
    tableau: &mut [Vec<f64>],
    basic_indices: &mut [i32],
    non_basic_indices: &mut [i32],
    phase_idx: usize,
    num_rows: usize,
    num_cols: usize,
) -> bool {
    loop {
        let mut best_col = usize::MAX;
        let mut best_val = (INF, i32::MAX);
        for (col_idx, &non_basic_idx) in non_basic_indices.iter().enumerate().take(num_cols + 1) {
            if phase_idx != 0 || non_basic_idx != -1 {
                let val = tableau[num_rows + phase_idx][col_idx];
                let key = (val, non_basic_idx);
                if best_col == usize::MAX
                    || key.0 < best_val.0 - EPS
                    || ((key.0 - best_val.0).abs() <= EPS && key.1 < best_val.1)
                {
                    best_col = col_idx;
                    best_val = key;
                }
            }
        }
        let pivot_col = best_col;
        if tableau[num_rows + phase_idx][pivot_col] > -EPS {
            return true;
        }
        let mut best_row = usize::MAX;
        let mut best_row_key = (INF, i32::MAX);
        for row_idx in 0..num_rows {
            if tableau[row_idx][pivot_col] > EPS {
                let ratio = tableau[row_idx][num_cols + 1] / tableau[row_idx][pivot_col];
                let key = (ratio, basic_indices[row_idx]);
                if best_row == usize::MAX
                    || key.0 < best_row_key.0 - EPS
                    || ((key.0 - best_row_key.0).abs() <= EPS && key.1 < best_row_key.1)
                {
                    best_row = row_idx;
                    best_row_key = key;
                }
            }
        }
        let pivot_row = best_row;
        if pivot_row == usize::MAX {
            return false;
        }
        pivot(tableau, basic_indices, non_basic_indices, pivot_row, pivot_col, num_rows, num_cols);
    }
}

fn main() {
    println!("🌟 --- Day 10 Results --- 🌟");
    let (res_1, duration_1) = time_execution(|| part_1(INPUT_TXT));
    println!("📌 Part 1: {res_1}, complete in {duration_1} ms");

    let (res_2, duration_2) = time_execution(|| part_2(INPUT_TXT));
    println!("📌 Part 2: {res_2}, complete in {duration_2} ms");
}

pub struct Machine {
    pub goal_mask: u32,
    pub goal_counters: Vec<i64>,
    pub button_masks: Vec<u32>,
}


/// Parses the input string into a vector of Machines.
///
/// # Panics
/// Panics if the input format is invalid or if parsing fails (e.g., `.unwrap()` on missing parts).
#[must_use]
pub fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let goal_str = &parts[0][1..parts[0].len() - 1];
            let mut goal_mask = 0;
            for (i, c) in goal_str.chars().enumerate() {
                if c == '#' {
                    goal_mask |= 1 << i;
                }
            }

            let last_part = parts.last().unwrap();
            let counter_str = &last_part[1..last_part.len() - 1];
            let goal_counters = counter_str
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            let mut button_masks = Vec::new();
            for part in &parts[1..parts.len() - 1] {
                let mut mask = 0;
                let inner = if part.starts_with('(') || part.starts_with('{') {
                    &part[1..part.len() - 1]
                } else {
                    part
                };

                for num_str in inner.split(',') {
                    if let Ok(bit) = num_str.parse::<u32>() {
                        mask |= 1 << bit;
                    }
                }
                button_masks.push(mask);
            }

            Machine {
                goal_mask,
                goal_counters,
                button_masks,
            }
        })
        .collect()
}

fn solve_bfs(machine: &Machine) -> i64 {
    let mut queue = VecDeque::new();
    queue.push_back((0u32, 0));
    let mut visited = HashSet::new();
    visited.insert(0);

    while let Some((curr, steps)) = queue.pop_front() {
        if curr == machine.goal_mask {
            return steps;
        }

        for &b_mask in &machine.button_masks {
            let nxt = curr ^ b_mask;
            if visited.insert(nxt) {
                queue.push_back((nxt, steps + 1));
            }
        }
    }
    0
}

fn simplex(lhs: &[Vec<f64>], cost_vector: &[f64]) -> (f64, Option<Vec<f64>>) {
    let num_rows = lhs.len();
    let num_cols = lhs[0].len() - 1;

    let mut non_basic_indices: Vec<i32> = (0..i32::try_from(num_cols).expect("num_cols too large")).collect();
    non_basic_indices.push(-1);

    let mut basic_indices: Vec<i32> =
        (i32::try_from(num_cols).expect("num_cols too large")..i32::try_from(num_cols + num_rows).expect("couldnt parse n+m")).collect();

    let mut tableau = vec![vec![0.0; num_cols + 2]; num_rows + 2];

    for (tableau_row, lhs_row) in tableau.iter_mut().zip(lhs.iter()) {
        tableau_row[..=num_cols].copy_from_slice(lhs_row);
        tableau_row[num_cols + 1] = -1.0;
    }

    for row in tableau.iter_mut().take(num_rows) {
        row.swap(num_cols, num_cols + 1);
    }

    tableau[num_rows][..num_cols].copy_from_slice(&cost_vector[..num_cols]);
    tableau[num_rows + 1][num_cols] = 1.0;

    let mut split_row = 0;
    let mut min_val = tableau[0][num_cols + 1];
    for (row_idx, row) in tableau.iter().enumerate().take(num_rows).skip(1) {
        if row[num_cols + 1] < min_val {
            min_val = row[num_cols + 1];
            split_row = row_idx;
        }
    }

    if tableau[split_row][num_cols + 1] < -EPS {
        pivot(&mut tableau[..], &mut basic_indices[..], &mut non_basic_indices[..], split_row, num_cols, num_rows, num_cols);
        if !find(&mut tableau[..], &mut basic_indices[..], &mut non_basic_indices[..], 1, num_rows, num_cols) || tableau[num_rows + 1][num_cols + 1] < -EPS {
            return (-INF, None);
        }
        for row_idx in 0..num_rows {
            if basic_indices[row_idx] == -1 {
                let mut best_col = 0;
                let mut best_key = (tableau[row_idx][0], non_basic_indices[0]);
                for (col_idx, &non_basic_idx) in non_basic_indices.iter().enumerate().take(num_cols).skip(1) {
                    let key = (tableau[row_idx][col_idx], non_basic_idx);
                    if key.0 < best_key.0 - EPS
                        || ((key.0 - best_key.0).abs() <= EPS && key.1 < best_key.1)
                    {
                        best_col = col_idx;
                        best_key = key;
                    }
                }
                pivot(&mut tableau[..], &mut basic_indices[..], &mut non_basic_indices[..], row_idx, best_col, num_rows, num_cols);
            }
        }
    }

    if find(&mut tableau[..], &mut basic_indices[..], &mut non_basic_indices[..], 0, num_rows, num_cols) {
        let mut solution = vec![0.0; num_cols];
        for row_idx in 0..num_rows {
            if let Ok(idx) = usize::try_from(basic_indices[row_idx])
                && idx < num_cols {
                solution[idx] = tableau[row_idx][num_cols + 1];
            }
        }
        let mut sum_val = 0.0;
        for col_idx in 0..num_cols {
            sum_val += cost_vector[col_idx] * solution[col_idx];
        }
        return (sum_val, Some(solution));
    }

    (-INF, None)
}

fn solve_ilp_bnb(initial_a: Vec<Vec<f64>>, obj_coeffs: &[f64]) -> i64 {
    let mut best_val = INF;
    let mut stack = Vec::new();
    stack.push(initial_a);

    while let Some(current_a) = stack.pop() {
        let (val, x_opt) = simplex(&current_a, obj_coeffs);

        if val == -INF || val >= best_val - EPS {
            continue;
        }

        let mut fractional_idx = None;
        let mut fractional_val = 0.0;

        if let Some(x) = x_opt {
            for (i, &xv) in x.iter().enumerate() {
                if (xv - xv.round()).abs() > EPS {
                    fractional_idx = Some(i);
                    fractional_val = xv;
                    break;
                }
            }

            if let Some(idx) = fractional_idx {
                let floor_v = fractional_val.floor();
                let n_cols = current_a[0].len();

                let mut row1 = vec![0.0; n_cols];
                row1[idx] = 1.0;
                row1[n_cols - 1] = floor_v;
                let mut a1 = current_a.clone();
                a1.push(row1);
                stack.push(a1);

                let ceil_v = fractional_val.ceil();
                let mut row2 = vec![0.0; n_cols];
                row2[idx] = -1.0;
                row2[n_cols - 1] = -ceil_v;
                let mut a2 = current_a.clone();
                a2.push(row2);
                stack.push(a2);
            } else if val < best_val {
                best_val = val;
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    if best_val == INF {
        0
    } else {
        i64::try_from(best_val.round() as i128).unwrap_or(0)
    }
}

fn solve_lp(machine: &Machine) -> i64 {
    let num_goals = machine.goal_counters.len();
    let num_buttons = machine.button_masks.len();

    let rows = 2 * num_goals + num_buttons;
    let cols = num_buttons + 1;

    let mut matrix = vec![vec![0.0; cols]; rows];

    for (j, row) in matrix.iter_mut().rev().take(num_buttons).enumerate() {
        row[j] = -1.0;
    }

    for (j, &mask) in machine.button_masks.iter().enumerate() {
        for i in 0..num_goals {
            if (mask >> i) & 1 == 1 {
                matrix[i][j] = 1.0;
                matrix[i + num_goals][j] = -1.0;
            }
        }
    }

    for i in 0..num_goals {
        #[allow(clippy::cast_precision_loss)]
        let val = machine.goal_counters[i] as f64;
        matrix[i][cols - 1] = val;
        matrix[i + num_goals][cols - 1] = -val;
    }

    let obj_coeffs = vec![1.0; num_buttons];
    solve_ilp_bnb(matrix, &obj_coeffs)
}

pub fn part_1(input: &str) -> i64 {
    let machines = parse(input);
    machines.iter().map(solve_bfs).sum()
}

#[must_use]
pub fn part_2(input: &str) -> i64 {
    let machines = parse(input);
    machines.iter().map(solve_lp).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(TEST_INPUT), 7);
        assert_eq!(part_1(INPUT_TXT), 419);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT), 33);
        assert_eq!(part_2(INPUT_TXT), 18_369);
    }
}
