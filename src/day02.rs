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

    match solve_part1(input) {
        Ok(value) => println!("Solution for part 1: {value}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn solve_part1(input: &str) -> Result<i128, InputError> {
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

        for id in first..=last {
            let id_str = id.to_string();

            if id_str.len() % 2 != 0 {
                continue;
            }

            if id_str[..id_str.len() / 2] == id_str[id_str.len() / 2..] {
                invalid_ids += id;
            }
        }
    }

    Ok(invalid_ids)
}
