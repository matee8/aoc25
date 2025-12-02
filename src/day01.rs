use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
enum InputError {
    Number,
    Direction,
}

impl Error for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
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

    match solve_part2(input) {
        Ok(value) => println!("Solution for part 2: {value}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn solve_part1(input: &str) -> Result<i32, InputError> {
    let mut position = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let Ok(distance) = distance.parse() else {
            return Err(InputError::Number);
        };

        let (new_position, _) = update_position(position, direction, distance)?;

        position = new_position;

        if position == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

// ugly bugly code duplication :c
fn solve_part2(input: &str) -> Result<i32, InputError> {
    let mut position = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let Ok(distance) = distance.parse() else {
            return Err(InputError::Number);
        };

        let (new_position, new_zeros) =
            update_position(position, direction, distance)?;

        position = new_position;
        zeros += new_zeros;
    }

    Ok(zeros)
}

fn update_position(
    current_position: i32,
    direction: &str,
    distance: i32,
) -> Result<(i32, i32), InputError> {
    let mut zeros = distance / 100;
    let delta = distance % 100;

    let new_position = match direction {
        "R" => {
            if current_position + delta > 100 {
                zeros += 1;
            }

            (current_position + delta) % 100
        }
        "L" => {
            if current_position > 0 && delta > current_position {
                zeros += 1;
            }

            let mut value = current_position - delta;

            if value < 0 {
                value += 100;
            }

            value
        }
        _ => return Err(InputError::Direction),
    };

    if new_position == 0 {
        zeros += 1;
    }

    Ok((new_position, zeros))
}
