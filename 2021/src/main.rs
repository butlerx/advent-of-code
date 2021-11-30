mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc-2021")]
struct Opt {
    /// day to run challenge for
    #[structopt(short, long)]
    day: i64,
    /// Flag for part 2
    #[structopt(long)]
    part_two: bool,
}

fn main() {
    let opts = Opt::from_args();
    let result = match opts.day {
        1 => day_1::run(include_str!("../input/day_1.txt"), opts.part_two),
        2 => day_2::run(include_str!("../input/day_2.txt"), opts.part_two),
        3 => day_3::run(include_str!("../input/day_3.txt"), opts.part_two),
        4 => day_4::run(include_str!("../input/day_4.txt"), opts.part_two),
        5 => day_5::run(include_str!("../input/day_5.txt"), opts.part_two),
        6 => day_6::run(include_str!("../input/day_6.txt"), opts.part_two),
        7 => day_7::run(include_str!("../input/day_7.txt"), opts.part_two),
        8 => day_8::run(include_str!("../input/day_8.txt"), opts.part_two),
        9 => day_9::run(include_str!("../input/day_9.txt"), opts.part_two),
        10 => day_10::run(include_str!("../input/day_10.txt"), opts.part_two),
        11 => day_11::run(include_str!("../input/day_11.txt"), opts.part_two),
        12 => day_12::run(include_str!("../input/day_12.txt"), opts.part_two),
        13 => day_13::run(include_str!("../input/day_13.txt"), opts.part_two),
        14 => day_14::run(include_str!("../input/day_14.txt"), opts.part_two),
        15 => day_15::run(include_str!("../input/day_15.txt"), opts.part_two),
        16 => day_16::run(include_str!("../input/day_16.txt"), opts.part_two),
        17 => day_17::run(include_str!("../input/day_17.txt"), opts.part_two),
        18 => day_18::run(include_str!("../input/day_18.txt"), opts.part_two),
        19 => day_19::run(include_str!("../input/day_19.txt"), opts.part_two),
        20 => day_20::run(include_str!("../input/day_20.txt"), opts.part_two),
        21 => day_21::run(include_str!("../input/day_21.txt"), opts.part_two),
        22 => day_22::run(include_str!("../input/day_22.txt"), opts.part_two),
        23 => day_23::run(include_str!("../input/day_23.txt"), opts.part_two),
        24 => day_24::run(include_str!("../input/day_24.txt"), opts.part_two),
        25 => day_25::run(include_str!("../input/day_25.txt"), opts.part_two),
        _ => 0,
    };
    println!("results: {}", result);
}
