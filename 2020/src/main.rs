mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc-2020")]
enum Opt {
    /// Day One, multiply numbers that add together to make 2020
    DayOne {
        /// Flag for part two position policy
        #[structopt(short, long)]
        part_two: bool,
    },
    /// Day Two, Validate strings for aritrary policy
    DayTwo {
        /// Flag for part two position policy
        #[structopt(short, long)]
        part_two: bool,
    },
    /// Day Three, calculate blocks in simple path
    DayThree {
        /// Input File
        #[structopt(default_value = "day_three.txt", long)]
        input: String,
        /// Flag for part 2
        #[structopt(long)]
        part_two: bool,
    },
    /// Day Four, Validate Passport data
    DayFour {
        /// Flag for part 2
        #[structopt(long)]
        part_two: bool,
    },
    /// Day Five, convert byte data to string and find missing number
    DayFive {
        /// Flag for part 2
        #[structopt(long)]
        part_two: bool,
    },
    /// Day Six
    DaySix {
        /// Flag for part 2
        #[structopt(long)]
        part_two: bool,
    },
}

fn main() {
    let result = match Opt::from_args() {
        Opt::DayOne { part_two } => day_one::run(
            include_str!("../input/day_one.txt"),
            if part_two { 3 } else { 2 },
        )
        .unwrap(),
        Opt::DayTwo { part_two } => {
            if part_two {
                day_two::part_2(include_str!("../input/day_two.txt")).unwrap()
            } else {
                day_two::part_1(include_str!("../input/day_two.txt")).unwrap()
            }
        }
        Opt::DayThree { input, part_two } => {
            let day_three_input = input::read(input).unwrap();
            if part_two {
                day_three::part_2(include_str!("../input/day_three.txt")).unwrap()
            } else {
                day_three::part_1(include_str!("../input/day_three.txt")).unwrap()
            }
        }
        Opt::DayFour { part_two } => {
            day_four::run(include_str!("../input/day_four.txt"), part_two).unwrap()
        }
        Opt::DayFive { part_two } => {
            if part_two {
                day_five::part_2(include_str!("../input/day_five.txt")).unwrap()
            } else {
                day_five::part_1(include_str!("../input/day_five.txt")).unwrap()
            }
        }
        Opt::DaySix { part_two } => {
            if part_two {
                day_six::part_2(include_str!("../input/day_six.txt")).unwrap()
            } else {
                day_six::part_1(include_str!("../input/day_six.txt")).unwrap()
            }
        }
    };
    println!("results: {}", result);
}
