mod day_one;
mod input;

fn main() {
    let day_one_input = input::read("day_one.txt").unwrap();
    let day_one_result = day_one::run(day_one_input).unwrap();
    println!("Day one result {}", day_one_result);
}
