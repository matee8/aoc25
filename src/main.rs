use std::env;

use aoc25::{day01, day02};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).and_then(|day| day.parse().ok()).unwrap_or(1);

    match day {
        1 => day01::main(),
        2 => day02::main(),
        _ => eprintln!("Day {day} not implemented yet."),
    }
}
