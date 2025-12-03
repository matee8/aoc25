use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
enum InputError {
    Range,
    Number,
}

impl Error for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Range => {
                write!(f, "invalid range in input file")
            }
            Self::Number => {
                write!(f, "invalid number (can't parse) in input file")
            }
        }
    }
}

#[inline]
pub fn main() {
    let input = include_str!("../assets/02.txt");

    match solve_task(input, solve_part1) {
        Ok(value) => println!("Solution for part 1: {value}"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match solve_task(input, solve_part2) {
        Ok(value) => println!("Solution for part 2: {value}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn solve_task<F>(input: &str, solver: F) -> Result<i128, InputError>
where
    F: Fn(i128, i128) -> i128,
{
    let mut invalid_ids = 0;

    for id_range in input.split(',') {
        let range_limits: Vec<&str> = id_range.split('-').collect();

        if range_limits.len() > 2 {
            return Err(InputError::Range);
        }

        let Some(first) = range_limits.first() else {
            return Err(InputError::Range);
        };

        let Some(last) = range_limits.get(1) else {
            return Err(InputError::Range);
        };

        if first.starts_with('0') {
            continue;
        }

        if last.starts_with('0') {
            continue;
        }

        let Ok(first): Result<i128, _> = first.trim().parse() else {
            return Err(InputError::Number);
        };

        let Ok(last): Result<i128, _> = last.trim().parse() else {
            return Err(InputError::Number);
        };

        invalid_ids += solver(first, last);
    }

    Ok(invalid_ids)
}

fn solve_part1(first: i128, last: i128) -> i128 {
    let mut invalid_ids = 0;

    for id in first..=last {
        let id_str = id.to_string();

        if id_str.len() % 2 != 0 {
            continue;
        }

        if id_str[..id_str.len() / 2] == id_str[id_str.len() / 2..] {
            invalid_ids += id;
        }
    }

    invalid_ids
}

fn solve_part2(first: i128, last: i128) -> i128 {
    let mut invalid_ids = 0;

    for id in first..=last {
        let id_str = id.to_string();

        for n in 1..=id_str.len() / 2 {
            if id_str.len() % n != 0 {
                continue;
            }

            let sequence = &id_str[0..n];
            let repetitions = id_str.len() / n;

            if sequence.repeat(repetitions) == id_str {
                invalid_ids += id;
                break;
            }
        }
    }

    invalid_ids
}
