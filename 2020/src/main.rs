mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod input;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc-2020")]
enum Opt {
    /// Day One, multiply numbers that add together to make 2020
    DayOne {
        /// Input File
        #[structopt(default_value = "day_one.txt", long)]
        input: String,
        /// Number of digits to add together
        #[structopt(default_value = "2", short, long)]
        num: i64,
    },
    /// Day Two, Validate strings for aritrary policy
    DayTwo {
        /// Input File
        #[structopt(default_value = "day_two.txt", long)]
        input: String,
        /// Flag for part two position policy
        #[structopt(short, long)]
        part_two: bool,
    },
    /// Day Three, calculate blocks in simple path
    DayThree {
        /// Input File
        #[structopt(default_value = "day_three.txt", long)]
        input: String,
        #[structopt(short, long)]
        right: usize,
        #[structopt(short, long)]
        down: usize,
        /// Flag for part 1
        #[structopt(long)]
        part_one: bool,
        /// Flag for part 2
        #[structopt(long)]
        part_two: bool,
    },
    /// Day Four,
    DayFour {
        /// Input File
        #[structopt(default_value = "day_four.txt", long)]
        input: String,
        /// Flag for part 2
        #[structopt(long)]
        part_two: bool,
    },
}

fn main() {
    let result = match Opt::from_args() {
        Opt::DayOne { input, num } => {
            let day_one_input = input::read(input).unwrap();
            day_one::run(day_one_input, num - 1).unwrap()
        }
        Opt::DayTwo { input, part_two } => {
            let day_two_input = input::read(input).unwrap();
            if part_two {
                day_two::part_2(day_two_input).unwrap()
            } else {
                day_two::part_1(day_two_input).unwrap()
            }
        }
        Opt::DayThree {
            input,
            right,
            down,
            part_one,
            part_two,
        } => {
            let day_three_input = input::read(input).unwrap();
            if part_one {
                day_three::part_1(day_three_input).unwrap()
            } else if part_two {
                day_three::part_2(day_three_input).unwrap()
            } else {
                day_three::run(day_three_input, right, down).unwrap()
            }
        }
        Opt::DayFour { input, part_two } => {
            let day_four_input = input::read(input).unwrap();
            if part_two {
                day_four::part_2(day_four_input).unwrap()
            } else {
                day_four::part_1(day_four_input).unwrap()
            }
        }
    };
    println!("results: {}", result);
}
