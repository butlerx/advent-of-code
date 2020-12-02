mod day_one;
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
    /// Day Two,
    DayTwo {
        /// Input File
        #[structopt(default_value = "day_two.txt", long)]
        input: String,
        /// Flag for position policy
        #[structopt(short, long)]
        position: bool,
    },
}

fn main() {
    let result = match Opt::from_args() {
        Opt::DayOne { input, num } => {
            let day_one_input = input::read(input).unwrap();
            day_one::run(day_one_input, num - 1).unwrap()
        }
        Opt::DayTwo { input, position } => {
            let day_two_input = input::read(input).unwrap();
            day_two::run(day_two_input, position).unwrap()
        }
    };
    println!("results: {}", result);
}
