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
#[structopt(name = "aoc-2020")]
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
        1 => day_1::run(
            include_str!("../input/day_1.txt"),
            if opts.part_two { 3 } else { 2 },
        ),
        2 => {
            if opts.part_two {
                day_2::part_2(include_str!("../input/day_2.txt"))
            } else {
                day_2::part_1(include_str!("../input/day_2.txt"))
            }
        }
        3 => {
            if opts.part_two {
                day_3::part_2(include_str!("../input/day_3.txt"))
            } else {
                day_3::part_1(include_str!("../input/day_3.txt"))
            }
        }
        4 => day_4::run(include_str!("../input/day_4.txt"), opts.part_two),
        5 => {
            if opts.part_two {
                day_5::part_2(include_str!("../input/day_5.txt"))
            } else {
                day_5::part_1(include_str!("../input/day_5.txt"))
            }
        }
        6 => {
            if opts.part_two {
                day_6::part_2(include_str!("../input/day_6.txt"))
            } else {
                day_6::part_1(include_str!("../input/day_6.txt"))
            }
        }
        7 => {
            if opts.part_two {
                day_7::part_2(include_str!("../input/day_7.txt"))
            } else {
                day_7::part_1(include_str!("../input/day_7.txt"))
            }
        }
        _ => 0,
    };
    println!("results: {}", result);
}
