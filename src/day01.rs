use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
enum InputError {
    Line,
    Number,
    Direction,
}

impl Error for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Line => {
                write!(f, "invalid line found in the input file")
            }
            Self::Number => {
                write!(f, "invalid number found in the input file")
            }
            Self::Direction => {
                write!(f, "invalid direction found in the input file")
            }
        }
    }
}

#[inline]
pub fn main() {
    let input = include_str!("../assets/01.txt");

    match solve_part1(input) {
        Ok(value) => println!("Solution for part 1: {value}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn solve_part1(input: &str) -> Result<i32, InputError> {
    let mut position = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let Some(direction) = line.chars().next() else {
            return Err(InputError::Line);
        };
        let Some(distance) = line.get(1..) else {
            return Err(InputError::Line);
        };
        let Ok(distance) = distance.parse() else {
            return Err(InputError::Number);
        };

        position = update_position(position, direction, distance)?;

        if position == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

const fn update_position(
    current_position: i32,
    direction: char,
    distance: i32,
) -> Result<i32, InputError> {
    let delta = match direction {
        'R' => distance,
        'L' => -distance,
        _ => return Err(InputError::Direction),
    };

    let mut new_position = current_position + delta;

    while new_position < 0 {
        new_position += 100;
    }

    while new_position > 99 {
        new_position -= 100;
    }

    Ok(new_position)
}
