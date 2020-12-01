mod day_one;
mod input;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc-2020")]
enum Opt {
    DayOne {
        #[structopt(default_value = "day_one.txt", long)]
        input: String,
        #[structopt(default_value = "1", short, long)]
        depth: i64,
    },
}

fn main() {
    let result = match Opt::from_args() {
        Opt::DayOne { input, depth } => {
            let day_one_input = input::read(input).unwrap();
            day_one::run(day_one_input, depth).unwrap()
        }
    };
    println!("results: {}", result);
}
