use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Input(Vec<i64>);

impl Input {
    fn from_stdin() -> Self {
        let stdin = io::stdin();
        Input(
            stdin
                .lock()
                .lines()
                .filter_map(|l| l.unwrap().parse::<i64>().ok())
                .collect::<Vec<_>>(),
        )
    }
}

fn solution1(input: &Input) -> Option<i64> {
    Some(input.0.iter().zip(input.0.iter().skip(1)).fold(0, |n, (&x, &y)| {
	if x < y {
	    1 + n
	} else {
	    n
	}
    }))
}

fn solution2(input: &Input) -> Option<i64> {
    let sums: Vec<_> = input.0.iter().zip(input.0.iter().skip(1)).zip(input.0.iter().skip(2)).map(|((&x, &y), &z)| x+y+z).collect();
    let modified = Input(sums);
    solution1(&modified)
}

fn main() {
    let input = Input::from_stdin();
    println!("{:?}", solution1(&input));
    println!("{:?}", solution2(&input));
}
