use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;
use thiserror::Error;

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

#[derive(Clone, Debug)]
struct Board {
    elems: Vec<Vec<i32>>,
}

#[derive(Clone, Debug)]
struct BoardTransformed {
    lines: Vec<Vec<i32>>,
    marked: Vec<i32>,
    items: HashSet<i32>,
    sum: i64,
    finished: bool,
}

impl From<&Board> for BoardTransformed {
    fn from(board: &Board) -> Self {
        let mut lines = Vec::new();
        let rows = board.elems.len();
        let cols = board.elems[0].len();
        let items = board.elems.iter().flat_map(|x| x.iter().cloned()).collect();
        let sum = board
            .elems
            .iter()
            .flat_map(|x| x.iter())
            .map(|&x| x as i64)
            .sum();
        for i in 0..rows {
            lines.push(board.elems[i].clone());
        }
        for j in 0..cols {
            let jcol = board.elems.iter().map(|l| l[j]).collect();
            lines.push(jcol);
        }
        for i in 0..rows {
            let mut x = Vec::new();
            x.push(board.elems[i][i]);
        }
        for i in 0..rows {
            let mut x = Vec::new();
            x.push(board.elems[i][cols - i - 1]);
        }
        BoardTransformed {
            lines,
            marked: Vec::new(),
            items,
            sum,
            finished: false,
        }
    }
}

impl From<Board> for BoardTransformed {
    fn from(board: Board) -> Self {
        board.into()
    }
}

#[derive(Debug, Error)]
enum AdventError {
    MalformedBoard,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdventError::MalformedBoard => write!(f, "Board is malformed"),
            AdventError::ParseInt(e) => write!(f, "{}", e),
            AdventError::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl FromStr for Board {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Vec::new();
        let _ = s
            .lines()
            .map(|l| {
                let v: Vec<i32> = l
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().map_err(|e| e.into()))
                    .collect::<Result<Vec<i32>, AdventError>>()?;
                if v.len() != BOARD_WIDTH {
                    return Err(AdventError::MalformedBoard);
                }
                res.push(v);
                Ok(())
            })
            .collect::<Result<Vec<_>, AdventError>>()?;
        if res.len() != BOARD_HEIGHT {
            return Err(AdventError::MalformedBoard);
        }
        Ok(Board { elems: res })
    }
}

#[derive(Clone, Debug)]
struct Input {
    seq: Vec<i32>,
    boards: Vec<Board>,
}

impl FromStr for Input {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut records = s.split("\n\n");
        let seq = if let Some(l1) = records.next() {
            l1.split(',')
                .map(|x| x.parse::<i32>().map_err(|e| e.into()))
                .collect::<Result<Vec<i32>, AdventError>>()?
        } else {
            Vec::new()
        };

        let mut boards = Vec::new();
        while let Some(board) = records.next() {
            let vv = board.parse::<Board>()?;
            boards.push(vv);
        }
        Ok(Input { seq, boards })
    }
}

impl Input {
    fn from_stdin() -> Result<Self, AdventError> {
        let mut stdin = io::stdin();
        let mut input = String::new();
        stdin.read_to_string(&mut input)?;
        input.parse()
    }
}

fn bingo(board: &BoardTransformed, ints: &HashSet<i32>) -> Option<Vec<i32>> {
    if board
        .lines
        .iter()
        .any(|line| line.iter().all(|x| ints.contains(x)))
    {
        let res = board.marked.clone();
        Some(res)
    } else {
        None
    }
}

fn part1(input: &Input) -> i64 {
    let mut ints = HashSet::new();
    let mut boards: Vec<BoardTransformed> = input.boards.iter().map(|x| x.into()).collect();

    for &x in &input.seq {
        ints.insert(x);
        for board in &mut boards {
            if board.items.contains(&x) {
                board.marked.push(x);
            }
            if let Some(matched) = bingo(board, &ints) {
                let unmarked_sum = board.sum - matched.iter().map(|x| *x as i64).sum::<i64>();
                return unmarked_sum * x as i64;
            }
        }
    }
    0
}

fn part2(input: &Input) -> i64 {
    let mut ints = HashSet::new();
    let mut boards: Vec<BoardTransformed> = input.boards.iter().map(|x| x.into()).collect();

    let mut res = 0;
    for &x in &input.seq {
        ints.insert(x);
        for board in &mut boards {
            if board.finished {
                continue;
            }
            if board.items.contains(&x) {
                board.marked.push(x);
            }
            if let Some(matched) = bingo(board, &ints) {
                let unmarked_sum = board.sum - matched.iter().map(|x| *x as i64).sum::<i64>();
                res = unmarked_sum * x as i64;
                board.finished = true;
            }
        }
    }
    res
}

fn main() -> Result<(), AdventError> {
    let input = Input::from_stdin()?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    const EXAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn can_parse_input() {
        let input = EXAMPLE.parse::<Input>();
        assert!(input.is_ok());
        let input = input.unwrap();
        assert_eq!(input.boards.len(), 3);
    }
    #[test]
    fn example_works() {
        let input = EXAMPLE.parse::<Input>();
        assert!(input.is_ok());
        let input = input.unwrap();
        assert_eq!(part1(&input), 4512);
        assert_eq!(part2(&input), 1924);
    }
}
