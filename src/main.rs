use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).and_then(|day| day.parse().ok()).unwrap_or(1);

    match day {
        _ => eprintln!("Day {day} not implemented yet."),
    }
}
