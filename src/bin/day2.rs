use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Clone, Debug, Error)]
enum ParseError {
    InvalidDirection(String),
    ExpectSpace,
    ParseIntError(String),
    UnexpectedLeftover(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidDirection(dir) => write!(f, "Invalid direction: {}", dir),
            ParseError::ExpectSpace => write!(f, "Expect space separator"),
            ParseError::ParseIntError(s) => write!(f, "Expect an integer, but got: {}", s),
            ParseError::UnexpectedLeftover(s) => write!(f, "Expect end of input, but got: {}", s),
        }
    }
}

impl FromStr for Direction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(ParseError::InvalidDirection(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Movement {
    dir: Direction,
    unit: i32,
}

impl Movement {
    fn new(dir: Direction, unit: i32) -> Self {
        Self { dir, unit }
    }
}

impl FromStr for Movement {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let dir = it
            .next()
            .ok_or_else(|| ParseError::InvalidDirection(String::new()))
            .and_then(|s| s.parse::<Direction>())?;
        let unit = it.next().ok_or(ParseError::ExpectSpace).and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| ParseError::ParseIntError(s.to_string()))
        })?;
        if let Some(leftover) = it.next() {
            return Err(ParseError::UnexpectedLeftover(leftover.to_string()));
        }
        Ok(Movement::new(dir, unit))
    }
}

#[derive(Debug, Error)]
enum AdventError {
    #[error(transparent)]
    InvalidInput(#[from] ParseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Clone, Debug)]
struct Input(Vec<Movement>);

impl Input {
    fn from_stdin() -> Self {
        let stdin = io::stdin();
        Input(
            stdin
                .lock()
                .lines()
                .filter_map(|l| l.unwrap().parse::<Movement>().ok())
                .collect::<Vec<_>>(),
        )
    }
}

fn part1(input: &Input) -> Result<i64, AdventError> {
    let pos = input.0.iter().fold((0, 0), |acc, next| match next.dir {
        Direction::Up => (acc.0, acc.1 - next.unit),
        Direction::Down => (acc.0, acc.1 + next.unit),
        Direction::Forward => (acc.0 + next.unit, acc.1),
    });
    Ok(pos.0 as i64 * pos.1 as i64)
}

fn main() -> Result<(), AdventError> {
    let input = Input::from_stdin();
    println!("{:?}", part1(&input));
    Ok(())
}
