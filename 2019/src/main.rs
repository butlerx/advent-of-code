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
#[structopt(name = "aoc-2019")]
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
        1 => {
            if opts.part_two {
                day_1::part_2(include_str!("../input/day_1.txt"))
            } else {
                day_1::part_1(include_str!("../input/day_1.txt"))
            }
        }
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
        4 => {
            if opts.part_two {
                day_4::part_2(include_str!("../input/day_4.txt"))
            } else {
                day_4::part_1(include_str!("../input/day_4.txt"))
            }
        }
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
        8 => {
            if opts.part_two {
                day_8::part_2(include_str!("../input/day_8.txt"))
            } else {
                day_8::part_1(include_str!("../input/day_8.txt"))
            }
        }
        9 => {
            if opts.part_two {
                day_9::part_2(include_str!("../input/day_9.txt"))
            } else {
                day_9::part_1(include_str!("../input/day_9.txt"))
            }
        }
        10 => {
            if opts.part_two {
                day_10::part_2(include_str!("../input/day_10.txt"))
            } else {
                day_10::part_1(include_str!("../input/day_10.txt"))
            }
        }
        11 => {
            if opts.part_two {
                day_11::part_2(include_str!("../input/day_11.txt"))
            } else {
                day_11::part_1(include_str!("../input/day_11.txt"))
            }
        }
        12 => {
            if opts.part_two {
                day_12::part_2(include_str!("../input/day_12.txt"))
            } else {
                day_12::part_1(include_str!("../input/day_12.txt"))
            }
        }
        _ => 0,
    };
    println!("results: {}", result);
}
