mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;

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
        1 => day_one::run(
            include_str!("../input/day_one.txt"),
            if opts.part_two { 3 } else { 2 },
        )
        .unwrap(),
        2 => {
            if opts.part_two {
                day_two::part_2(include_str!("../input/day_two.txt")).unwrap()
            } else {
                day_two::part_1(include_str!("../input/day_two.txt")).unwrap()
            }
        }
        3 => {
            if opts.part_two {
                day_three::part_2(include_str!("../input/day_three.txt")).unwrap()
            } else {
                day_three::part_1(include_str!("../input/day_three.txt")).unwrap()
            }
        }
        4 => day_four::run(include_str!("../input/day_four.txt"), opts.part_two).unwrap(),
        5 => {
            if opts.part_two {
                day_five::part_2(include_str!("../input/day_five.txt")).unwrap()
            } else {
                day_five::part_1(include_str!("../input/day_five.txt")).unwrap()
            }
        }
        6 => {
            if opts.part_two {
                day_six::part_2(include_str!("../input/day_six.txt")).unwrap()
            } else {
                day_six::part_1(include_str!("../input/day_six.txt")).unwrap()
            }
        }
        7 => {
            if opts.part_two {
                day_seven::part_2(include_str!("../input/day_seven.txt")).unwrap()
            } else {
                day_seven::part_1(include_str!("../input/day_seven.txt")).unwrap()
            }
        }
        _ => 0,
    };
    println!("results: {}", result);
}
