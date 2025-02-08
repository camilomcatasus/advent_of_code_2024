use std::env;

mod utils;
mod day6;
mod day5;
mod day4;
mod day3;
mod day2;
mod day1;
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("Could not parse argument");

    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        _ => todo!("Unsupported")
    }
}

