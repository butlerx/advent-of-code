mod day_one;
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
    DayTwo {},
}

fn main() {
    let result = match Opt::from_args() {
        Opt::DayOne { input, num } => {
            let day_one_input = input::read(input).unwrap();
            day_one::run(day_one_input, num - 1).unwrap()
        }
        Opt::DayTwo {} => 0,
    };
    println!("results: {}", result);
}
