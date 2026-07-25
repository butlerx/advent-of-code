#![warn(clippy::pedantic, clippy::perf)]

use aoc_shared::{Args, read_input, time_execution};

type Solver = fn(&str) -> usize;

fn main() {
    let args = Args::parse("systems_check");
    let input = read_input(env!("CARGO_MANIFEST_DIR"), args.day);

    let (part_1, part_2): (Solver, Solver) = match args.day {
        1 => (
            systems_check::mission_01::part_1,
            systems_check::mission_01::part_2,
        ),
        2 => (
            systems_check::mission_02::part_1,
            systems_check::mission_02::part_2,
        ),
        3 => (
            systems_check::mission_03::part_1,
            systems_check::mission_03::part_2,
        ),
        4 => (
            systems_check::mission_04::part_1,
            systems_check::mission_04::part_2,
        ),
        5 => (
            systems_check::mission_05::part_1,
            systems_check::mission_05::part_2,
        ),
        _ => panic!("Unknown mission: {}", args.day),
    };

    println!("🌟 --- Mission {} Results --- 🌟", args.day);
    if args.part != Some(2) {
        let (res, duration) = time_execution(|| part_1(&input));
        println!("📌 Part 1: {res}, complete in {duration} ms");
    }
    if args.part != Some(1) {
        let (res, duration) = time_execution(|| part_2(&input));
        println!("📌 Part 2: {res}, complete in {duration} ms");
    }
}
