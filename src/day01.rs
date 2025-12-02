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
    let mut new_position = current_position;
    let mut zeros = 0;

    for _ in 0..distance {
        match direction {
            "R" => {
                new_position += 1;
                if new_position > 99 {
                    new_position = 0;
                }
            }
            "L" => {
                new_position -= 1;
                if new_position < 0 {
                    new_position = 99;
                }
            }
            _ => return Err(InputError::Direction),
        }

        if new_position == 0 {
            zeros += 1;
        }
    }

    Ok((new_position, zeros))
}
